use {
    std::{
        fs::File,
        io,
        path::{Path, PathBuf},
    },
    tempfile::{Builder, NamedTempFile},
};

#[derive(Debug)]
pub struct TempFile(NamedTempFile);

impl TempFile {
    pub fn new() -> io::Result<Self> {
        NamedTempFile::new().map(Self)
    }

    pub fn in_dir(dir: impl AsRef<Path>) -> io::Result<Self> {
        Builder::new().prefix(".").suffix(".tmp").tempfile_in(dir.as_ref()).map(Self)
    }

    pub fn as_file(&self) -> &File {
        self.0.as_file()
    }

    pub fn as_file_mut(&mut self) -> &mut File {
        self.0.as_file_mut()
    }

    pub fn path(&self) -> &Path {
        self.0.path()
    }

    pub fn persist(self, path: impl AsRef<Path>) -> io::Result<File> {
        self.0.persist(path).map_err(|e| e.error)
    }

    pub fn keep(self) -> io::Result<(File, PathBuf)> {
        self.0.keep().map_err(|e| e.error)
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        std::{
            fs,
            io::{self, Write},
        },
        tempfile::tempdir,
    };

    #[test]
    fn new_creates_temp_in_system_tmp_and_is_deleted_on_drop() -> io::Result<()> {
        let path = {
            let t = TempFile::new()?;
            let p = t.path().to_path_buf();
            assert!(p.exists());
            p
        };
        // after t drops, file should be gone
        assert!(!path.exists());
        Ok(())
    }

    #[test]
    fn in_dir_places_file_in_given_dir() -> io::Result<()> {
        let dir = tempdir()?;
        let t = TempFile::in_dir(dir.path())?;
        assert_eq!(t.path().parent().unwrap(), dir.path());
        Ok(())
    }

    #[test]
    fn keep_stops_auto_delete_and_returns_file_and_path() -> io::Result<()> {
        let dir = tempdir()?;
        let t = TempFile::in_dir(dir.path())?;
        let expected = t.path().to_path_buf();

        let (mut f, p) = t.keep()?; // consumes temp; no auto-delete
        assert_eq!(p, expected);

        f.write_all(b"xyz")?;
        drop(f);
        assert!(p.exists(), "kept file should remain after drop");
        assert_eq!(fs::read(&p)?, b"xyz");
        Ok(())
    }

    #[test]
    fn persist_writes_contents_to_destination() -> io::Result<()> {
        let dir = tempdir()?;
        let dest = dir.path().join("data.txt");

        let mut t = TempFile::in_dir(dir.path())?;
        t.as_file_mut().write_all(b"hello")?;
        let _file = t.persist(&dest)?; // consumes t
        assert_eq!(fs::read_to_string(&dest)?, "hello");
        Ok(())
    }

    #[test]
    fn persist_overwrites_existing_file() -> io::Result<()> {
        let dir = tempdir()?;
        let dest = dir.path().join("swap.bin");
        fs::write(&dest, b"old")?;

        let mut t = TempFile::in_dir(dir.path())?;
        t.as_file_mut().write_all(b"new")?;
        t.persist(&dest)?;

        assert_eq!(fs::read(&dest)?, b"new");
        Ok(())
    }

    #[test]
    fn in_dir_errors_if_not_a_directory() {
        let dir = tempdir().unwrap();
        let not_a_dir = dir.path().join("file.txt");
        fs::write(&not_a_dir, "x").unwrap();

        TempFile::in_dir(&not_a_dir).expect_err("in_dir should fail when given a file path");
    }
}
