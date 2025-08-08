use std::{fs, io, path::Path, time::SystemTime};

pub fn touch(path: impl AsRef<Path>) -> io::Result<bool> {
    _touch(path.as_ref())
}

fn _touch(path: &Path) -> io::Result<bool> {
    match fs::OpenOptions::new().write(true).create_new(true).open(path) {
        Ok(_) => Ok(true), // File created

        Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {
            let meta = fs::metadata(path).map_err(|e| {
                io::Error::new(e.kind(), format!("Failed to inspect '{}': {e}", path.display()))
            })?;

            if !meta.is_file() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!(
                        "Path '{}' exists but is not a file (file type: {:?})",
                        path.display(),
                        meta.file_type()
                    ),
                ));
            }

            // Update modification time
            let now = filetime::FileTime::from_system_time(SystemTime::now());
            filetime::set_file_times(path, now, now)?;
            Ok(false) // File exists already
        }

        Err(e) => Err(io::Error::new(
            e.kind(),
            format!("Failed to open/create '{}': {e}", path.display()),
        )),
    }
}

#[cfg(test)]
mod tests {
    use {
        super::touch,
        std::{
            fs, io,
            thread::sleep,
            time::{Duration, SystemTime},
        },
        tempfile::tempdir,
    };

    #[test]
    fn creates_file_when_missing() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("new.txt");

        let created = touch(&file_path).unwrap();
        assert!(created, "Expected touch to create a missing file");

        let meta = fs::metadata(&file_path).unwrap();
        assert!(meta.is_file(), "Expected a file to exist after touch");
    }

    #[test]
    fn returns_false_and_updates_mtime_when_exists() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("exists.txt");

        fs::write(&file_path, b"hello").unwrap();
        let mtime_before =
            fs::metadata(&file_path).unwrap().modified().unwrap_or(SystemTime::UNIX_EPOCH);

        // Many filesystems have ~1s mtime resolution; wait to avoid flakes
        sleep(Duration::from_millis(1100));

        let created = touch(&file_path).unwrap();
        assert!(!created, "Expected touch to return false when file already exists");

        let mtime_after =
            fs::metadata(&file_path).unwrap().modified().unwrap_or(SystemTime::UNIX_EPOCH);

        assert!(
            mtime_after > mtime_before,
            "Expected mtime to be updated. before={mtime_before:?}, after={mtime_after:?}"
        );
    }

    #[test]
    fn preserves_existing_contents() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("keep.txt");

        fs::write(&file_path, b"original").unwrap();
        let _ = touch(&file_path).unwrap();

        let contents = fs::read(&file_path).unwrap();
        assert_eq!(contents, b"original", "Touch must not alter file contents");
    }

    #[test]
    fn errors_if_path_is_directory() {
        let dir = tempdir().unwrap();
        let subdir_path = dir.path().join("a_dir");

        fs::create_dir(&subdir_path).unwrap();

        let result = touch(&subdir_path);
        assert!(result.is_err(), "Touch should error on a directory path");

        let err = result.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidInput);
    }
}
