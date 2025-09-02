use std::{fs, io, path::Path, time::SystemTime};

pub fn created(path: impl AsRef<Path>) -> io::Result<SystemTime> {
    _created(path.as_ref())
}

fn _created(path: &Path) -> io::Result<SystemTime> {
    fs::metadata(path).and_then(|meta| meta.created()).map_err(|e| {
        io::Error::new(
            e.kind(),
            format!("Failed to get creation time for '{}': {e}", path.display()),
        )
    })
}

#[cfg(test)]
mod tests {
    use {
        super::created,
        std::{fs, io},
        tempfile::tempdir,
    };

    #[test]
    fn returns_creation_time_for_existing_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("file.txt");
        fs::write(&file_path, "hello").unwrap();

        created(&file_path).unwrap();
    }

    #[test]
    fn returns_error_for_missing_file() {
        let dir = tempdir().unwrap();
        let missing = dir.path().join("nope.txt");

        let err = created(&missing).unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::NotFound);
    }
}
