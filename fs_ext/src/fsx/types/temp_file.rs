use {
    crate::IoResultExt,
    std::{
        fs::File,
        io::{self, Seek, SeekFrom},
        path::{Path, PathBuf},
    },
    tempfile::{Builder, NamedTempFile},
};

#[derive(Debug)]
pub struct TempFile(pub(crate) NamedTempFile);

impl TempFile {
    pub fn new() -> io::Result<Self> {
        NamedTempFile::new().map(Self)
    }

    pub fn in_dir(dir: impl AsRef<Path>) -> io::Result<Self> {
        let dir = dir.as_ref();
        Builder::new()
            .prefix(".")
            .suffix(".tmp")
            .tempfile_in(dir)
            .map(Self)
            .with_path_context("failed to create tempfile in", dir)
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

    pub fn persist_new(self, path: impl AsRef<Path>) -> io::Result<File> {
        let dst = path.as_ref().to_owned();
        let src = self.path().to_owned();
        self.0.persist_noclobber(&dst).map_err(|e| e.error).with_paths_context(
            "failed to persist tempfile (noclobber)",
            &src,
            &dst,
        )
    }

    pub fn persist(self, path: impl AsRef<Path>) -> io::Result<File> {
        let dst = path.as_ref().to_owned();
        let src = self.path().to_owned();
        self.0.persist(&dst).map_err(|e| e.error).with_paths_context(
            "failed to persist tempfile",
            &src,
            &dst,
        )
    }

    pub fn keep(self) -> io::Result<(File, PathBuf)> {
        let src = self.path().to_owned();
        self.0.keep().map_err(|e| e.error).with_path_context("failed to keep tempfile", &src)
    }

    pub fn copy_from(&mut self, path: impl AsRef<Path>) -> io::Result<()> {
        let src = path.as_ref().to_owned();
        let dst = self.path().to_owned();

        let mut source =
            File::open(&src).with_path_context("failed to open source for copy", &src)?;

        // Truncate + rewind destination with context
        let tmp = self.as_file_mut();
        tmp.set_len(0).with_path_context("failed to truncate tempfile", &dst)?;
        tmp.seek(SeekFrom::Start(0)).with_path_context("failed to seek tempfile to start", &dst)?;

        io::copy(&mut source, tmp).with_paths_context("failed to copy", &src, &dst)?;

        tmp.sync_all().with_path_context("failed to fsync tempfile", &dst)?;

        Ok(())
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

    #[test]
    fn copy_copies_source_and_truncates_temp() -> io::Result<()> {
        let dir = tempdir()?;
        let src = dir.path().join("src.txt");

        fs::write(&src, b"hello")?;

        let mut t = TempFile::in_dir(dir.path())?;
        t.as_file_mut().write_all(b"AAAAAAAAAAAA")?;

        t.copy_from(&src)?;

        assert_eq!(fs::read(t.path())?, b"hello");
        Ok(())
    }

    #[test]
    fn copy_leaves_cursor_at_end_allowing_append() -> io::Result<()> {
        let dir = tempdir()?;
        let src = dir.path().join("src.txt");
        fs::write(&src, b"base")?;

        let mut t = TempFile::in_dir(dir.path())?;
        t.copy_from(&src)?;

        t.as_file_mut().write_all(b"+more")?;

        assert_eq!(fs::read(t.path())?, b"base+more");
        Ok(())
    }

    #[test]
    fn copy_errors_when_source_missing() {
        let dir = tempdir().unwrap();
        let missing = dir.path().join("nope.txt");

        let mut t = TempFile::in_dir(dir.path()).unwrap();
        let err = t.copy_from(&missing).unwrap_err();

        assert_eq!(err.kind(), io::ErrorKind::NotFound);
    }
}
