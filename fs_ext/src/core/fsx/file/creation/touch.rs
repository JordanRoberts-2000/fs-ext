use {
    filetime::FileTime,
    std::{
        fs::{File, OpenOptions},
        io,
        path::Path,
        time::SystemTime,
    },
};

#[cfg_attr(test, fs_ext_test_macros::fs_test(existing_file_ok, rejects_dir, new_file_ok))]
pub fn touch(path: impl AsRef<Path>) -> io::Result<File> {
    _touch(path.as_ref())
}

fn _touch(path: &Path) -> io::Result<std::fs::File> {
    let file = OpenOptions::new().write(true).create(true).open(path).map_err(|e| {
        io::Error::new(
            e.kind(),
            format!("Failed to open or create file at '{}': {e}", path.display()),
        )
    })?;

    let now = FileTime::from_system_time(SystemTime::now());
    filetime::set_file_times(path, now, now).map_err(|e| {
        io::Error::new(
            e.kind(),
            format!("Failed to update file atime & mtime for '{}': {e}", path.display()),
        )
    })?;

    Ok(file)
}

#[cfg(test)]
mod tests {
    use {
        super::touch,
        std::{
            fs,
            thread::sleep,
            time::{Duration, SystemTime},
        },
        tempfile::tempdir,
    };

    #[test]
    fn creates_file_when_missing() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("new.txt");

        let _file = touch(&file_path).unwrap();

        let meta = fs::metadata(&file_path).unwrap();
        assert!(meta.is_file(), "Expected a file to exist after touch");
    }

    #[test]
    fn updates_mtime_when_exists() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("exists.txt");

        fs::write(&file_path, b"hello").unwrap();
        let mtime_before =
            fs::metadata(&file_path).unwrap().modified().unwrap_or(SystemTime::UNIX_EPOCH);

        // Many filesystems have ~1s mtime resolution; wait to avoid flakes
        sleep(Duration::from_millis(1100));

        let _file = touch(&file_path).unwrap();

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
}
