use {
    crate::PathExt,
    std::{fs, io, path::Path},
};

#[cfg_attr(test, fs_ext_test_macros::fs_test(rejects_missing_path, rejects_dir))]
pub fn assert_readable(path: impl AsRef<Path>) -> io::Result<()> {
    _assert_readable(path.as_ref())
}

fn _assert_readable(path: &Path) -> io::Result<()> {
    path.assert_file()?;

    match fs::File::open(path) {
        Ok(_) => Ok(()),
        Err(e) => {
            Err(io::Error::new(e.kind(), format!("File '{}' is not readable: {e}", path.display())))
        }
    }
}

#[cfg(test)]
mod tests {
    use {
        super::assert_readable,
        std::{fs, io},
        tempfile::tempdir,
    };

    #[test]
    fn ok_when_file_is_readable() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("file.txt");
        fs::write(&file_path, "hello").unwrap();

        let res = assert_readable(&file_path);
        assert!(res.is_ok(), "expected Ok(()), got {res:?}");
    }

    #[cfg(unix)]
    #[test]
    fn err_when_file_not_readable() {
        use std::os::unix::fs::PermissionsExt;

        let dir = tempdir().unwrap();
        let file_path = dir.path().join("locked.txt");
        fs::write(&file_path, "secret").unwrap();

        // Remove all read/write/execute permissions
        let mut perms = fs::metadata(&file_path).unwrap().permissions();
        perms.set_mode(0o000);
        fs::set_permissions(&file_path, perms).unwrap();

        let err = assert_readable(&file_path).unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::PermissionDenied);
    }
}
