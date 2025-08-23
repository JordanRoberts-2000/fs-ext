use std::{
    fs::{File, OpenOptions},
    io,
    path::Path,
};

pub fn write_only(path: impl AsRef<Path>) -> io::Result<File> {
    OpenOptions::new().write(true).open(path)
}

pub fn read_only(path: impl AsRef<Path>) -> io::Result<File> {
    OpenOptions::new().read(true).open(path)
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        std::io::{self, Read, Write},
        tempfile::tempdir,
    };

    #[test]
    fn read_only_errors_if_missing() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("missing.txt");

        let err = read_only(&path).unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::NotFound);
    }

    #[test]
    fn write_only_errors_if_missing() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("missing.txt");

        let err = write_only(&path).unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::NotFound);
    }

    #[test]
    fn read_only_can_read_but_cannot_write() -> io::Result<()> {
        let dir = tempdir()?;
        let path = dir.path().join("data.txt");
        std::fs::write(&path, b"hello")?;

        let mut f = read_only(&path)?;

        let mut s = String::new();
        f.read_to_string(&mut s)?;
        assert_eq!(s, "hello");

        assert!(f.write_all(b"!").is_err(), "read unexpectedly succeeded");

        Ok(())
    }

    #[test]
    fn write_only_can_write_but_cannot_read() -> io::Result<()> {
        let dir = tempdir()?;
        let path = dir.path().join("data.txt");
        std::fs::write(&path, b"old")?;

        let mut f = write_only(&path)?;
        f.write_all(b"new")?;
        f.flush()?;

        let mut s = String::new();
        assert!(f.read_to_string(&mut s).is_err(), "read unexpectedly succeeded");

        drop(f);

        let on_disk = std::fs::read_to_string(&path)?;
        assert_eq!(on_disk, "new");

        Ok(())
    }
}
