use std::{fs, io, path::Path};

pub fn assert_writable(path: impl AsRef<Path>) -> io::Result<()> {
    _assert_writable(path.as_ref())
}

fn _assert_writable(path: &Path) -> io::Result<()> {
    match fs::OpenOptions::new().write(true).open(path) {
        Ok(_) => Ok(()),
        Err(e) => {
            Err(io::Error::new(e.kind(), format!("File '{}' is not writable: {e}", path.display())))
        }
    }
}

#[cfg(test)]
mod tests {
    use {
        super::assert_writable,
        std::{fs, io},
        tempfile::tempdir,
    };

    #[test]
    fn ok_when_file_is_writable() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("writable.txt");
        fs::write(&file_path, "hello").unwrap();

        let res = assert_writable(&file_path);
        assert!(res.is_ok(), "expected Ok(()), got {res:?}");
    }

    #[test]
    fn err_when_file_missing() {
        let dir = tempdir().unwrap();
        let missing = dir.path().join("missing.txt");

        let err = assert_writable(&missing).unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::NotFound);
        let msg = err.to_string();
        assert!(msg.contains("is not writable"), "msg={msg}");
        assert!(msg.contains(missing.to_string_lossy().as_ref()), "msg={msg}");
    }

    #[cfg(unix)]
    #[test]
    fn err_when_file_not_writable() {
        use std::os::unix::fs::PermissionsExt;

        let dir = tempdir().unwrap();
        let file_path = dir.path().join("locked.txt");
        fs::write(&file_path, "secret").unwrap();

        // Remove all write permissions
        let mut perms = fs::metadata(&file_path).unwrap().permissions();
        perms.set_mode(0o444); // read-only
        fs::set_permissions(&file_path, perms).unwrap();

        let err = assert_writable(&file_path).unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::PermissionDenied);
        let msg = err.to_string();
        assert!(msg.contains("is not writable"), "msg={msg}");
    }
}
