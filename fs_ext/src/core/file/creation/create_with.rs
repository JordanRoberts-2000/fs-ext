use {
    crate::{CollisionStrategy, WriteOptions, file},
    std::{fs::File, io, path::Path},
};

pub fn create_with(
    path: impl AsRef<Path>, options: impl AsRef<WriteOptions>,
) -> io::Result<Option<File>> {
    _create_with(path.as_ref(), options.as_ref())
}

fn _create_with(path: &Path, options: &WriteOptions) -> io::Result<Option<File>> {
    if let Some(parent) = path.parent() {
        options.parent.ensure(parent)?;
    }

    match &options.collision {
        CollisionStrategy::Error => file::create_new(path).map(Some),

        CollisionStrategy::Skip => match file::create_new(path) {
            Ok(f) => Ok(Some(f)),
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => Ok(None),
            Err(e) => Err(e),
        },

        CollisionStrategy::Overwrite => file::overwrite(path).map(Some),

        CollisionStrategy::Rename(rename_opts) => match file::create_new(path) {
            Ok(f) => Ok(Some(f)),
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {
                let unique_path = rename_opts.generate_unique_path(path)?;
                file::create_new(&unique_path).map(Some)
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
            fs::{self, File},
            io::Write,
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

        let result = create_with(&path, &options).unwrap();
        assert!(result.is_some());
        assert!(path.exists());
    }

    #[test]
    fn test_error_strategy_fails_on_existing_file() {
        let temp_dir = setup_temp_dir();
        let path = temp_dir.path().join("test.txt");

        File::create(&path).unwrap();

        let options = WriteOptions {
            parent: ParentPolicy::RequireExists,
            collision: CollisionStrategy::Error,
        };

        let result = create_with(&path, &options);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::AlreadyExists);
    }

    #[test]
    fn test_skip_strategy_creates_new_file() {
        let temp_dir = setup_temp_dir();
        let path = temp_dir.path().join("test.txt");

        let options = WriteOptions {
            parent: ParentPolicy::RequireExists,
            collision: CollisionStrategy::Skip,
        };

        let result = create_with(&path, &options).unwrap();
        assert!(result.is_some());
        assert!(path.exists());
    }

    #[test]
    fn test_skip_strategy_skips_existing_file() {
        let temp_dir = setup_temp_dir();
        let path = temp_dir.path().join("test.txt");

        // Create and write to the file first
        let mut file = File::create(&path).unwrap();
        writeln!(file, "original content").unwrap();
        drop(file);

        let options = WriteOptions {
            parent: ParentPolicy::RequireExists,
            collision: CollisionStrategy::Skip,
        };

        let result = create_with(&path, &options).unwrap();
        assert!(result.is_none());

        // Verify original content is unchanged
        let content = fs::read_to_string(&path).unwrap();
        assert_eq!(content, "original content\n");
    }

    #[test]
    fn test_overwrite_strategy_creates_new_file() {
        let temp_dir = setup_temp_dir();
        let path = temp_dir.path().join("test.txt");

        let options = WriteOptions {
            parent: ParentPolicy::RequireExists,
            collision: CollisionStrategy::Overwrite,
        };

        let result = create_with(&path, &options).unwrap();
        assert!(result.is_some());
        assert!(path.exists());
    }

    #[test]
    fn test_overwrite_strategy_overwrites_existing_file() {
        let temp_dir = setup_temp_dir();
        let path = temp_dir.path().join("test.txt");

        // Create and write to the file first
        let mut file = File::create(&path).unwrap();
        writeln!(file, "original content").unwrap();
        drop(file);

        let options = WriteOptions {
            parent: ParentPolicy::RequireExists,
            collision: CollisionStrategy::Overwrite,
        };

        let mut result = create_with(&path, &options).unwrap().unwrap();
        writeln!(result, "new content").unwrap();
        drop(result);

        // Verify content was overwritten
        let content = fs::read_to_string(&path).unwrap();
        assert_eq!(content, "new content\n");
    }

    #[test]
    fn test_rename_counter_creates_new_file() {
        let temp_dir = setup_temp_dir();
        let path = temp_dir.path().join("test.txt");

        let options = WriteOptions {
            parent: ParentPolicy::RequireExists,
            collision: CollisionStrategy::Rename(RenameOptions::Counter),
        };

        let result = create_with(&path, &options).unwrap();
        assert!(result.is_some());
        assert!(path.exists());
    }

    #[test]
    fn test_rename_counter_renames_on_collision() {
        let temp_dir = setup_temp_dir();
        let path = temp_dir.path().join("test.txt");

        // Create the original file
        File::create(&path).unwrap();

        let options = WriteOptions {
            parent: ParentPolicy::RequireExists,
            collision: CollisionStrategy::Rename(RenameOptions::Counter),
        };

        let result = create_with(&path, &options).unwrap();
        assert!(result.is_some());

        // Verify the renamed file exists
        let renamed_path = temp_dir.path().join("test_1.txt");
        assert!(renamed_path.exists());
        assert!(path.exists()); // Original still exists
    }

    #[test]
    fn test_rename_counter_increments_correctly() {
        let temp_dir = setup_temp_dir();
        let path = temp_dir.path().join("test.txt");

        // Create test.txt and test_1.txt
        File::create(&path).unwrap();
        File::create(temp_dir.path().join("test_1.txt")).unwrap();

        let options = WriteOptions {
            parent: ParentPolicy::RequireExists,
            collision: CollisionStrategy::Rename(RenameOptions::Counter),
        };

        let result = create_with(&path, &options).unwrap();
        assert!(result.is_some());

        // Should create test_2.txt
        let renamed_path = temp_dir.path().join("test_2.txt");
        assert!(renamed_path.exists());
    }

    #[test]
    fn test_rename_timestamp_creates_unique_file() {
        let temp_dir = setup_temp_dir();
        let path = temp_dir.path().join("test.txt");

        // Create the original file
        File::create(&path).unwrap();

        let options = WriteOptions {
            parent: ParentPolicy::RequireExists,
            collision: CollisionStrategy::Rename(RenameOptions::Timestamp),
        };

        let result = create_with(&path, &options).unwrap();
        assert!(result.is_some());

        // Verify a timestamped file was created
        let entries: Vec<_> = fs::read_dir(temp_dir.path())
            .unwrap()
            .map(|e| e.unwrap().file_name().to_string_lossy().to_string())
            .collect();

        assert_eq!(entries.len(), 2); // Original + renamed
        assert!(entries.iter().any(|name| name.starts_with("test_") && name.ends_with(".txt")));
    }

    #[test]
    fn test_rename_uuid_creates_unique_file() {
        let temp_dir = setup_temp_dir();
        let path = temp_dir.path().join("test.txt");

        // Create the original file
        File::create(&path).unwrap();

        let options = WriteOptions {
            parent: ParentPolicy::RequireExists,
            collision: CollisionStrategy::Rename(RenameOptions::Uuid),
        };

        let result = create_with(&path, &options).unwrap();
        assert!(result.is_some());

        // Verify a UUID file was created
        let entries: Vec<_> = fs::read_dir(temp_dir.path())
            .unwrap()
            .map(|e| e.unwrap().file_name().to_string_lossy().to_string())
            .collect();

        assert_eq!(entries.len(), 2); // Original + renamed
        assert!(entries.iter().any(|name| name.starts_with("test_") && name.ends_with(".txt")));
    }

    #[test]
    fn test_parent_require_exists_fails_on_missing_parent() {
        let temp_dir = setup_temp_dir();
        let path = temp_dir.path().join("nonexistent").join("test.txt");

        let options = WriteOptions {
            parent: ParentPolicy::RequireExists,
            collision: CollisionStrategy::Error,
        };

        let result = create_with(&path, &options);
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

        let result = create_with(&path, &options).unwrap();
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

        let result = create_with(&path, &options).unwrap();
        assert!(result.is_some());
        assert!(path.exists());
    }

    #[test]
    fn test_file_can_be_written_to() {
        let temp_dir = setup_temp_dir();
        let path = temp_dir.path().join("test.txt");

        let options = WriteOptions {
            parent: ParentPolicy::RequireExists,
            collision: CollisionStrategy::Error,
        };

        let mut file = create_with(&path, &options).unwrap().unwrap();
        writeln!(file, "test content").unwrap();
        drop(file);

        let content = fs::read_to_string(&path).unwrap();
        assert_eq!(content, "test content\n");
    }
}
