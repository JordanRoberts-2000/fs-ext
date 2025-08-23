use std::{
    fs::{File, OpenOptions},
    io,
    path::Path,
};

pub fn open(path: impl AsRef<Path>) -> io::Result<File> {
    OpenOptions::new().write(true).read(true).open(path)
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        std::{
            fs,
            io::{Seek, SeekFrom, Write},
        },
        tempfile::tempdir,
    };

    #[test]
    fn opens_existing_file_read_write_and_can_modify() -> io::Result<()> {
        let dir = tempdir()?;
        let path = dir.path().join("data.txt");

        fs::write(&path, "hello")?;

        let mut f = open(&path)?;

        f.seek(SeekFrom::End(0))?;
        f.write_all(b"!")?;
        f.flush()?;
        drop(f);

        let s = fs::read_to_string(&path)?;
        assert_eq!(s, "hello!");
        Ok(())
    }

    #[test]
    fn does_not_create_missing_file() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("missing.txt");

        let err = open(&path).unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::NotFound);
        assert!(!path.exists(), "file should not be created");
    }

    #[test]
    fn error_when_target_is_directory() {
        let dir = tempdir().unwrap();
        let target = dir.path();

        let err = open(target).unwrap_err();
        assert!(
            matches!(
                err.kind(),
                io::ErrorKind::IsADirectory
                    | io::ErrorKind::PermissionDenied
                    | io::ErrorKind::Other
            ),
            "unexpected error kind: {:?}",
            err.kind()
        );
    }

    #[test]
    fn error_when_file_is_readonly() -> io::Result<()> {
        let dir = tempdir()?;
        let path = dir.path().join("ro.txt");

        fs::write(&path, "locked")?;

        let mut perms = fs::metadata(&path)?.permissions();
        perms.set_readonly(true);
        fs::set_permissions(&path, perms)?;

        let res = open(&path);
        assert!(res.is_err(), "open should fail for read-only file when write(true)");

        if let Err(e) = res {
            assert!(
                matches!(e.kind(), io::ErrorKind::PermissionDenied | io::ErrorKind::Other),
                "unexpected error kind: {:?}",
                e.kind()
            );
        }
        Ok(())
    }
}
