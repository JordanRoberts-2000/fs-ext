use {
    crate::{CollisionStrategy, WriteOptions, file::atomic},
    std::{error, fs::File, io, path::Path},
};

pub fn create_with<F, T, E>(
    path: impl AsRef<Path>, write_fn: F, options: impl AsRef<WriteOptions>,
) -> io::Result<Option<T>>
where
    E: Into<Box<dyn error::Error + Send + Sync>>,
    F: Fn(&mut File) -> Result<T, E>,
{
    let path = path.as_ref();
    let options = options.as_ref();

    if let Some(parent) = path.parent() {
        options.parent.ensure(parent)?;
    }

    match &options.collision {
        CollisionStrategy::Error => atomic::create_new(path, write_fn).map(Some),

        CollisionStrategy::Skip => match atomic::create_new(path, write_fn) {
            Ok(f) => Ok(Some(f)),
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => Ok(None),
            Err(e) => Err(e),
        },

        CollisionStrategy::Overwrite => atomic::overwrite(path, write_fn).map(Some),

        CollisionStrategy::Rename(rename_opts) => match atomic::create_new(path, &write_fn) {
            Ok(f) => Ok(Some(f)),
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {
                let unique_path = rename_opts.generate_unique_path(path)?;
                atomic::create_new(&unique_path, write_fn).map(Some)
            }
            Err(e) => Err(e),
        },
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{CollisionStrategy, ParentPolicy, RenameOptions, WriteOptions},
        std::{
            fs,
            io::{self, Write},
        },
        tempfile::TempDir,
    };

    fn setup_temp_dir() -> TempDir {
        tempfile::tempdir().unwrap()
    }

    #[test]
    fn test_create_new_file_with_error_strategy() {
        let temp_dir = setup_temp_dir();
        let path = temp_dir.path().join("test.txt");

        let options = WriteOptions {
            parent: ParentPolicy::RequireExists,
            collision: CollisionStrategy::Error,
        };

        let result = create_with(
            &path,
            |f| {
                writeln!(f, "test content")?;
                Ok::<_, io::Error>(42)
            },
            &options,
        )
        .unwrap();

        assert_eq!(result, Some(42));
        assert!(path.exists());

        let content = fs::read_to_string(&path).unwrap();
        assert_eq!(content, "test content\n");
    }

    #[test]
    fn test_error_strategy_fails_on_existing_file() {
        let temp_dir = setup_temp_dir();
        let path = temp_dir.path().join("test.txt");

        // Create the file first
        fs::write(&path, "original").unwrap();

        let options = WriteOptions {
            parent: ParentPolicy::RequireExists,
            collision: CollisionStrategy::Error,
        };

        let result = create_with(
            &path,
            |f| {
                writeln!(f, "new content")?;
                Ok::<_, io::Error>(())
            },
            &options,
        );

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::AlreadyExists);

        // Original content should be unchanged
        let content = fs::read_to_string(&path).unwrap();
        assert_eq!(content, "original");
    }

    #[test]
    fn test_skip_strategy_creates_new_file() {
        let temp_dir = setup_temp_dir();
        let path = temp_dir.path().join("test.txt");

        let options = WriteOptions {
            parent: ParentPolicy::RequireExists,
            collision: CollisionStrategy::Skip,
        };

        let result = create_with(
            &path,
            |f| {
                writeln!(f, "test content")?;
                Ok::<_, io::Error>("success")
            },
            &options,
        )
        .unwrap();

        assert_eq!(result, Some("success"));
        assert!(path.exists());
    }

    #[test]
    fn test_skip_strategy_skips_existing_file() {
        let temp_dir = setup_temp_dir();
        let path = temp_dir.path().join("test.txt");

        // Create the file first
        fs::write(&path, "original content").unwrap();

        let options = WriteOptions {
            parent: ParentPolicy::RequireExists,
            collision: CollisionStrategy::Skip,
        };

        let result = create_with(
            &path,
            |f| {
                writeln!(f, "new content")?;
                Ok::<_, io::Error>("wrote")
            },
            &options,
        )
        .unwrap();

        assert_eq!(result, None);

        // Verify original content is unchanged
        let content = fs::read_to_string(&path).unwrap();
        assert_eq!(content, "original content");
    }

    #[test]
    fn test_overwrite_strategy_creates_new_file() {
        let temp_dir = setup_temp_dir();
        let path = temp_dir.path().join("test.txt");

        let options = WriteOptions {
            parent: ParentPolicy::RequireExists,
            collision: CollisionStrategy::Overwrite,
        };

        let result = create_with(
            &path,
            |f| {
                writeln!(f, "test content")?;
                Ok::<_, io::Error>(123)
            },
            &options,
        )
        .unwrap();

        assert_eq!(result, Some(123));
        assert!(path.exists());
    }

    #[test]
    fn test_overwrite_strategy_overwrites_existing_file() {
        let temp_dir = setup_temp_dir();
        let path = temp_dir.path().join("test.txt");

        // Create the file first
        fs::write(&path, "original content that is quite long").unwrap();

        let options = WriteOptions {
            parent: ParentPolicy::RequireExists,
            collision: CollisionStrategy::Overwrite,
        };

        let result = create_with(
            &path,
            |f| {
                writeln!(f, "new")?;
                Ok::<_, io::Error>(())
            },
            &options,
        )
        .unwrap();

        assert!(result.is_some());

        // Verify content was overwritten (and truncated)
        let content = fs::read_to_string(&path).unwrap();
        assert_eq!(content, "new\n");
    }

    #[test]
    fn test_rename_counter_creates_new_file() {
        let temp_dir = setup_temp_dir();
        let path = temp_dir.path().join("test.txt");

        let options = WriteOptions {
            parent: ParentPolicy::RequireExists,
            collision: CollisionStrategy::Rename(RenameOptions::Counter),
        };

        let result = create_with(
            &path,
            |f| {
                writeln!(f, "test content")?;
                Ok::<_, io::Error>(())
            },
            &options,
        )
        .unwrap();

        assert!(result.is_some());
        assert!(path.exists());
    }

    #[test]
    fn test_rename_counter_renames_on_collision() {
        let temp_dir = setup_temp_dir();
        let path = temp_dir.path().join("test.txt");

        // Create the original file
        fs::write(&path, "original").unwrap();

        let options = WriteOptions {
            parent: ParentPolicy::RequireExists,
            collision: CollisionStrategy::Rename(RenameOptions::Counter),
        };

        let result = create_with(
            &path,
            |f| {
                writeln!(f, "renamed content")?;
                Ok::<_, io::Error>(())
            },
            &options,
        )
        .unwrap();

        assert!(result.is_some());

        // Verify the renamed file exists
        let renamed_path = temp_dir.path().join("test_1.txt");
        assert!(renamed_path.exists());

        let renamed_content = fs::read_to_string(&renamed_path).unwrap();
        assert_eq!(renamed_content, "renamed content\n");

        // Original should be unchanged
        let original_content = fs::read_to_string(&path).unwrap();
        assert_eq!(original_content, "original");
    }

    #[test]
    fn test_rename_counter_increments_correctly() {
        let temp_dir = setup_temp_dir();
        let path = temp_dir.path().join("test.txt");

        // Create test.txt and test_1.txt
        fs::write(&path, "original").unwrap();
        fs::write(temp_dir.path().join("test_1.txt"), "first rename").unwrap();

        let options = WriteOptions {
            parent: ParentPolicy::RequireExists,
            collision: CollisionStrategy::Rename(RenameOptions::Counter),
        };

        let result = create_with(
            &path,
            |f| {
                writeln!(f, "second rename")?;
                Ok::<_, io::Error>(())
            },
            &options,
        )
        .unwrap();

        assert!(result.is_some());

        // Should create test_2.txt
        let renamed_path = temp_dir.path().join("test_2.txt");
        assert!(renamed_path.exists());

        let content = fs::read_to_string(&renamed_path).unwrap();
        assert_eq!(content, "second rename\n");
    }

    #[test]
    fn test_rename_timestamp_creates_unique_file() {
        let temp_dir = setup_temp_dir();
        let path = temp_dir.path().join("test.txt");

        // Create the original file
        fs::write(&path, "original").unwrap();

        let options = WriteOptions {
            parent: ParentPolicy::RequireExists,
            collision: CollisionStrategy::Rename(RenameOptions::Timestamp),
        };

        let result = create_with(
            &path,
            |f| {
                writeln!(f, "timestamped content")?;
                Ok::<_, io::Error>(())
            },
            &options,
        )
        .unwrap();

        assert!(result.is_some());

        // Verify a timestamped file was created
        let entries: Vec<_> = fs::read_dir(temp_dir.path())
            .unwrap()
            .map(|e| e.unwrap().file_name().to_string_lossy().to_string())
            .collect();

        assert_eq!(entries.len(), 2); // Original + renamed
        assert!(entries.contains(&"test.txt".to_string()));
        assert!(
            entries.iter().any(|name| name.starts_with("test_")
                && name.ends_with(".txt")
                && name != "test.txt")
        );
    }

    #[test]
    fn test_rename_uuid_creates_unique_file() {
        let temp_dir = setup_temp_dir();
        let path = temp_dir.path().join("test.txt");

        // Create the original file
        fs::write(&path, "original").unwrap();

        let options = WriteOptions {
            parent: ParentPolicy::RequireExists,
            collision: CollisionStrategy::Rename(RenameOptions::Uuid),
        };

        let result = create_with(
            &path,
            |f| {
                writeln!(f, "uuid content")?;
                Ok::<_, io::Error>(())
            },
            &options,
        )
        .unwrap();

        assert!(result.is_some());

        // Verify a UUID file was created
        let entries: Vec<_> = fs::read_dir(temp_dir.path())
            .unwrap()
            .map(|e| e.unwrap().file_name().to_string_lossy().to_string())
            .collect();

        assert_eq!(entries.len(), 2); // Original + renamed
    }

    #[test]
    fn test_parent_require_exists_fails_on_missing_parent() {
        let temp_dir = setup_temp_dir();
        let path = temp_dir.path().join("nonexistent").join("test.txt");

        let options = WriteOptions {
            parent: ParentPolicy::RequireExists,
            collision: CollisionStrategy::Error,
        };

        let result = create_with(
            &path,
            |f| {
                writeln!(f, "content")?;
                Ok::<_, io::Error>(())
            },
            &options,
        );

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::NotFound);
    }

    #[test]
    fn test_parent_create_if_missing_creates_parent() {
        let temp_dir = setup_temp_dir();
        let path = temp_dir.path().join("new_dir").join("test.txt");

        let options = WriteOptions {
            parent: ParentPolicy::CreateIfMissing,
            collision: CollisionStrategy::Error,
        };

        let result = create_with(
            &path,
            |f| {
                writeln!(f, "content")?;
                Ok::<_, io::Error>(())
            },
            &options,
        )
        .unwrap();

        assert!(result.is_some());
        assert!(path.exists());
        assert!(path.parent().unwrap().exists());
    }

    #[test]
    fn test_parent_create_if_missing_with_nested_dirs() {
        let temp_dir = setup_temp_dir();
        let path = temp_dir.path().join("a").join("b").join("c").join("test.txt");

        let options = WriteOptions {
            parent: ParentPolicy::CreateIfMissing,
            collision: CollisionStrategy::Error,
        };

        let result = create_with(
            &path,
            |f| {
                writeln!(f, "nested content")?;
                Ok::<_, io::Error>(())
            },
            &options,
        )
        .unwrap();

        assert!(result.is_some());
        assert!(path.exists());

        let content = fs::read_to_string(&path).unwrap();
        assert_eq!(content, "nested content\n");
    }

    #[test]
    fn test_write_function_called_with_correct_file() {
        let temp_dir = setup_temp_dir();
        let path = temp_dir.path().join("test.txt");

        let options = WriteOptions {
            parent: ParentPolicy::RequireExists,
            collision: CollisionStrategy::Error,
        };

        let result = create_with(
            &path,
            |f| {
                write!(f, "line 1\n")?;
                write!(f, "line 2\n")?;
                write!(f, "line 3\n")?;
                Ok::<_, io::Error>(3)
            },
            &options,
        )
        .unwrap();

        assert_eq!(result, Some(3));

        let content = fs::read_to_string(&path).unwrap();
        assert_eq!(content, "line 1\nline 2\nline 3\n");
    }

    #[test]
    fn test_write_function_error_prevents_file_creation() {
        let temp_dir = setup_temp_dir();
        let path = temp_dir.path().join("test.txt");

        let options = WriteOptions {
            parent: ParentPolicy::RequireExists,
            collision: CollisionStrategy::Error,
        };

        let result = create_with(
            &path,
            |_f| Err::<(), _>(io::Error::new(io::ErrorKind::Other, "write failed")),
            &options,
        );

        assert!(result.is_err());
        // File should not exist due to atomic operation
        assert!(!path.exists());
    }

    #[test]
    fn test_atomicity_on_write_failure() {
        let temp_dir = setup_temp_dir();
        let path = temp_dir.path().join("test.txt");

        let options = WriteOptions {
            parent: ParentPolicy::RequireExists,
            collision: CollisionStrategy::Error,
        };

        // First successful write
        create_with(
            &path,
            |f| {
                writeln!(f, "original")?;
                Ok::<_, io::Error>(())
            },
            &options,
        )
        .unwrap();

        let original_content = fs::read_to_string(&path).unwrap();

        // Second write that fails (with overwrite)
        let options_overwrite = WriteOptions {
            parent: ParentPolicy::RequireExists,
            collision: CollisionStrategy::Overwrite,
        };

        let result = create_with(
            &path,
            |f| {
                writeln!(f, "partial")?;
                Err::<(), _>(io::Error::new(io::ErrorKind::Other, "simulated error"))
            },
            &options_overwrite,
        );

        assert!(result.is_err());

        // Original content should be preserved due to atomicity
        let content = fs::read_to_string(&path).unwrap();
        assert_eq!(content, original_content);
    }

    #[test]
    fn test_return_value_propagation() {
        let temp_dir = setup_temp_dir();
        let path = temp_dir.path().join("test.txt");

        let options = WriteOptions {
            parent: ParentPolicy::RequireExists,
            collision: CollisionStrategy::Error,
        };

        #[derive(Debug, PartialEq)]
        struct CustomReturn {
            bytes_written: usize,
            lines: usize,
        }

        let result = create_with(
            &path,
            |f| {
                writeln!(f, "line 1")?;
                writeln!(f, "line 2")?;
                Ok::<_, io::Error>(CustomReturn { bytes_written: 14, lines: 2 })
            },
            &options,
        )
        .unwrap();

        assert_eq!(result, Some(CustomReturn { bytes_written: 14, lines: 2 }));
    }
}
