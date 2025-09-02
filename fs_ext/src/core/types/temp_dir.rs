use {
    crate::IoResultExt,
    std::{
        io,
        path::{Path, PathBuf},
    },
    tempfile::{Builder, TempDir as TfTempDir},
};

#[derive(Debug)]
pub struct TempDir(TfTempDir);

impl TempDir {
    pub fn new() -> io::Result<Self> {
        TfTempDir::new().map(Self)
    }

    pub fn in_dir(dir: impl AsRef<Path>) -> io::Result<Self> {
        let dir = dir.as_ref();
        Builder::new()
            .prefix(".tmp-")
            .tempdir_in(dir)
            .map(Self)
            .with_path_context("failed to create tempdir in", dir)
    }

    pub fn path(&self) -> &Path {
        self.0.path()
    }

    pub fn keep(self) -> PathBuf {
        self.0.keep()
    }

    pub fn close(self) -> io::Result<()> {
        let p = self.0.path().to_path_buf();
        self.0.close().with_path_context("failed to close tempdir", p)
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        std::{fs, io},
        tempfile::tempdir,
    };

    #[test]
    fn new_creates_and_deletes_on_drop() -> io::Result<()> {
        let path = {
            let t = TempDir::new()?;
            let p = t.path().to_path_buf();
            assert!(p.exists(), "temp dir should exist while handle is alive");
            p
        };
        assert!(!path.exists(), "temp dir should be removed on Drop");
        Ok(())
    }

    #[test]
    fn in_dir_places_tempdir_in_provided_dir() -> io::Result<()> {
        let parent = tempdir()?;
        let t = TempDir::in_dir(parent.path())?;
        assert_eq!(t.path().parent().unwrap(), parent.path());
        Ok(())
    }

    #[test]
    fn keep_stops_autodelete_and_returns_path() -> io::Result<()> {
        let t = TempDir::new()?;
        let p = t.keep(); // auto-delete disabled
        assert!(p.exists(), "kept dir should remain after wrapper is dropped");
        // clean up to keep tests tidy
        fs::remove_dir_all(&p)?;
        Ok(())
    }

    #[test]
    fn close_removes_directory_now() -> io::Result<()> {
        let t = TempDir::new()?;
        let p = t.path().to_path_buf();
        // put something inside so we test recursive removal too
        fs::write(p.join("file.txt"), "hello")?;
        t.close()?; // consumes and removes
        assert!(!p.exists(), "dir should be gone after close()");
        Ok(())
    }

    #[test]
    fn in_dir_fails_on_non_directory() {
        let d = tempdir().unwrap();
        let not_a_dir = d.path().join("file.txt");
        fs::write(&not_a_dir, "x").unwrap();

        TempDir::in_dir(&not_a_dir).expect_err("should fail when provided non-directory");
    }
}
