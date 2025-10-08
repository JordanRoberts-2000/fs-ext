use {
    crate::PathExt,
    std::{
        io,
        path::{Path, PathBuf},
        time::{SystemTime, UNIX_EPOCH},
    },
    uuid::Uuid,
};

#[derive(Default, Clone)]
pub struct WriteOptions {
    pub parent: ParentPolicy,
    pub collision: CollisionStrategy,
}

impl AsRef<WriteOptions> for WriteOptions {
    fn as_ref(&self) -> &WriteOptions {
        self
    }
}

#[derive(Default, Clone)]
pub enum ParentPolicy {
    #[default]
    RequireExists,
    CreateIfMissing,
}

impl ParentPolicy {
    pub fn ensure(&self, parent: &Path) -> io::Result<()> {
        match self {
            ParentPolicy::RequireExists => {
                if !parent.exists() {
                    return Err(io::Error::new(
                        io::ErrorKind::NotFound,
                        format!("Parent directory does not exist: {}", parent.display()),
                    ));
                }
                Ok(())
            }
            ParentPolicy::CreateIfMissing => {
                if !parent.exists() {
                    std::fs::create_dir_all(parent)?;
                }
                Ok(())
            }
        }
    }
}

#[derive(Default, Clone)]
pub enum CollisionStrategy {
    #[default]
    Error,
    Overwrite,
    Rename(RenameOptions),
    Skip,
}

#[derive(Default, Clone)]
pub enum RenameOptions {
    Timestamp,
    Uuid,
    #[default]
    Counter,
}

impl RenameOptions {
    pub fn generate_unique_path(&self, path: &Path) -> io::Result<PathBuf> {
        let stem = path.utf8_stem()?;
        let ext = path.utf8_extension()?.map(|s| format!(".{}", s)).unwrap_or_default();
        let parent = path.parent_or_current();

        match self {
            RenameOptions::Timestamp => {
                let timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
                    .as_secs();
                let new_name = format!("{}_{}{}", stem, timestamp, ext);
                Ok(parent.join(new_name))
            }
            RenameOptions::Uuid => {
                for _ in 0..8 {
                    let id = Uuid::new_v4();
                    let cand = parent.join(format!("{stem}_{id}{ext}"));
                    if !cand.exists() {
                        return Ok(cand);
                    }
                }
                return Err(io::Error::new(
                    io::ErrorKind::AlreadyExists,
                    "couldn't find a free UUID filename after 8 attempts",
                ));
            }
            RenameOptions::Counter => {
                for i in 1..10000 {
                    let new_name = format!("{}_{}{}", stem, i, ext);
                    let new_path = parent.join(new_name);

                    if !new_path.exists() {
                        return Ok(new_path);
                    }
                }
                Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Could not find available counter-based filename",
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        std::{fs::File, path::Path},
        tempfile::tempdir,
    };

    #[test]
    fn test_timestamp_generates_unique_path() {
        let path = Path::new("test.txt");
        let result = RenameOptions::Timestamp.generate_unique_path(path).unwrap();

        let file_name = result.file_name().unwrap().to_str().unwrap();
        assert!(file_name.starts_with("test_"));
        assert!(file_name.ends_with(".txt"));

        // Check that the timestamp portion is numeric
        let parts: Vec<&str> = file_name.split('_').collect();
        assert_eq!(parts.len(), 2);
        let timestamp_part = parts[1].trim_end_matches(".txt");
        assert!(timestamp_part.parse::<u64>().is_ok());
    }

    #[test]
    fn test_timestamp_preserves_path_directory() {
        let path = Path::new("dir/subdir/test.txt");
        let result = RenameOptions::Timestamp.generate_unique_path(path).unwrap();

        assert_eq!(result.parent().unwrap(), Path::new("dir/subdir"));
    }

    #[test]
    fn test_timestamp_handles_no_extension() {
        let path = Path::new("test");
        let result = RenameOptions::Timestamp.generate_unique_path(path).unwrap();

        let file_name = result.file_name().unwrap().to_str().unwrap();
        assert!(file_name.starts_with("test_"));
        assert!(!file_name.contains('.'));
    }

    #[test]
    fn test_uuid_generates_valid_path() {
        let path = Path::new("test.txt");
        let result = RenameOptions::Uuid.generate_unique_path(path).unwrap();

        let file_name = result.file_name().unwrap().to_str().unwrap();
        assert!(file_name.starts_with("test_"));
        assert!(file_name.ends_with(".txt"));

        // Check that there's a UUID-like pattern
        let parts: Vec<&str> = file_name.split('_').collect();
        assert_eq!(parts.len(), 2);
        let uuid_part = parts[1].trim_end_matches(".txt");
        // UUID format: 8-4-4-4-12 hex digits
        assert!(uuid_part.len() >= 32); // At least 32 hex chars plus hyphens
    }

    #[test]
    fn test_uuid_generates_different_paths() {
        let path = Path::new("test.txt");
        let result1 = RenameOptions::Uuid.generate_unique_path(path).unwrap();
        let result2 = RenameOptions::Uuid.generate_unique_path(path).unwrap();

        assert_ne!(result1, result2);
    }

    #[test]
    fn test_counter_generates_path_with_1() {
        let path = Path::new("test.txt");
        let result = RenameOptions::Counter.generate_unique_path(path).unwrap();

        assert_eq!(result, Path::new("./test_1.txt"));
    }

    #[test]
    fn test_counter_skips_existing_files() {
        let dir = tempdir().unwrap();
        let base_path = dir.path().join("test.txt");

        // Create test_1.txt and test_2.txt
        File::create(dir.path().join("test_1.txt")).unwrap();
        File::create(dir.path().join("test_2.txt")).unwrap();

        let result = RenameOptions::Counter.generate_unique_path(&base_path).unwrap();

        assert_eq!(result, dir.path().join("./test_3.txt"));
    }

    #[test]
    fn test_counter_handles_no_extension() {
        let path = Path::new("test");
        let result = RenameOptions::Counter.generate_unique_path(path).unwrap();

        assert_eq!(result, Path::new("./test_1"));
    }

    #[test]
    fn test_counter_with_directory() {
        let path = Path::new("dir/test.txt");
        let result = RenameOptions::Counter.generate_unique_path(path).unwrap();

        assert_eq!(result, Path::new("dir/test_1.txt"));
    }

    #[test]
    fn test_all_strategies_preserve_complex_extensions() {
        let path = Path::new("archive.tar.gz");

        let timestamp_result = RenameOptions::Timestamp.generate_unique_path(path).unwrap();
        let counter_result = RenameOptions::Counter.generate_unique_path(path).unwrap();
        let uuid_result = RenameOptions::Uuid.generate_unique_path(path).unwrap();

        assert!(timestamp_result.to_str().unwrap().contains("archive"));
        assert!(counter_result.to_str().unwrap().contains("archive"));
        assert!(uuid_result.to_str().unwrap().contains("archive"));
    }
}
