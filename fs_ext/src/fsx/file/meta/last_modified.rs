use std::{fs, io, path::Path, time::SystemTime};

pub fn last_modified(path: impl AsRef<Path>) -> io::Result<SystemTime> {
    _last_modified(path.as_ref())
}

fn _last_modified(path: &Path) -> io::Result<SystemTime> {
    fs::metadata(path).and_then(|meta| meta.modified()).map_err(|e| {
        io::Error::new(
            e.kind(),
            format!("Failed to get last modified time for '{}': {e}", path.display()),
        )
    })
}

#[cfg(test)]
mod tests {
    use {
        super::last_modified,
        std::{fs, io},
        tempfile::tempdir,
    };

    #[test]
    fn returns_time_for_existing_file() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("file.txt");
        fs::write(&file, "hello").unwrap();

        last_modified(&file).unwrap();
    }

    #[test]
    fn err_for_missing_path() {
        let dir = tempdir().unwrap();
        let missing = dir.path().join("nope.txt");

        let err = last_modified(&missing).unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::NotFound);
    }
}
