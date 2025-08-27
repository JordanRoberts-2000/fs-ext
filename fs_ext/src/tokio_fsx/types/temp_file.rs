use {
    std::{
        io::{self, SeekFrom},
        path::{Path, PathBuf},
    },
    tempfile::{Builder, NamedTempFile},
    tokio::{fs::File, io::AsyncSeekExt, task},
};

#[derive(Debug)]
pub struct TempFile(NamedTempFile);

impl TempFile {
    pub async fn new() -> io::Result<Self> {
        let t = task::spawn_blocking(NamedTempFile::new).await.map_err(join_err)??;
        Ok(Self(t))
    }

    pub async fn in_dir(dir: impl AsRef<Path> + Send) -> io::Result<Self> {
        let dir = dir.as_ref().to_path_buf();
        let t = task::spawn_blocking(move || {
            Builder::new().prefix(".").suffix(".tmp").tempfile_in(dir)
        })
        .await
        .map_err(join_err)??;
        Ok(Self(t))
    }

    pub fn as_file(&self) -> io::Result<File> {
        let file = self.0.as_file().try_clone()?;
        Ok(File::from_std(file))
    }

    pub fn path(&self) -> &Path {
        self.0.path()
    }

    pub async fn persist(self, path: impl AsRef<Path> + Send) -> io::Result<File> {
        let path = path.as_ref().to_path_buf();
        let file = task::spawn_blocking(move || self.0.persist(&path).map_err(|e| e.error))
            .await
            .map_err(join_err)??;
        Ok(File::from_std(file))
    }

    pub async fn persist_new(self, path: impl AsRef<Path> + Send) -> io::Result<File> {
        let path = path.as_ref().to_path_buf();
        let file =
            task::spawn_blocking(move || self.0.persist_noclobber(&path).map_err(|e| e.error))
                .await
                .map_err(join_err)??;
        Ok(File::from_std(file))
    }

    pub async fn keep(self) -> io::Result<(File, PathBuf)> {
        let (file, path) =
            task::spawn_blocking(move || self.0.keep()).await.map_err(join_err)??;
        Ok((File::from_std(file), path))
    }

    pub async fn copy(&mut self, src: impl AsRef<Path> + Send) -> io::Result<()> {
        let mut source = File::open(src.as_ref()).await?;
        let mut tmp = self.as_file()?;

        tmp.set_len(0).await?;
        tmp.seek(SeekFrom::Start(0)).await?;

        tokio::io::copy(&mut source, &mut tmp).await?;

        tmp.sync_all().await?;

        Ok(())
    }
}

fn join_err(e: tokio::task::JoinError) -> io::Error {
    io::Error::new(io::ErrorKind::Other, format!("blocking task failed: {e}"))
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        std::io,
        tempfile::tempdir,
        tokio::{
            fs,
            io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt},
        },
    };

    #[tokio::test]
    async fn new_creates_temp_in_system_tmp_and_is_deleted_on_drop() -> io::Result<()> {
        let path = {
            let t = TempFile::new().await?;
            let p = t.path().to_path_buf();
            assert!(p.exists(), "temp file should exist while handle is alive");
            p
        };
        // after `t` drops, file should be gone
        assert!(!path.exists(), "temp file should be removed on Drop after scope ends");
        Ok(())
    }

    #[tokio::test]
    async fn in_dir_places_file_in_given_dir() -> io::Result<()> {
        let dir = tempdir()?;
        let t = TempFile::in_dir(dir.path()).await?;
        assert_eq!(
            t.path().parent().unwrap(),
            dir.path(),
            "temp file should be created inside the provided dir"
        );
        Ok(())
    }

    #[tokio::test]
    async fn keep_stops_auto_delete_and_returns_file_and_path() -> io::Result<()> {
        let dir = tempdir()?;
        let t = TempFile::in_dir(dir.path()).await?;
        let expected = t.path().to_path_buf();

        let (mut f, p) = t.keep().await?; // consumes temp; disables auto-delete
        assert_eq!(p, expected, "keep() must return the same path");

        f.write_all(b"xyz").await?;
        f.flush().await?;
        drop(f);

        assert!(p.exists(), "kept file should remain after drop");
        let bytes = fs::read(&p).await?;
        assert_eq!(bytes.as_slice(), b"xyz");
        Ok(())
    }

    #[tokio::test]
    async fn persist_writes_contents_to_destination() -> io::Result<()> {
        let dir = tempdir()?;
        let dest = dir.path().join("data.txt");

        let t = TempFile::in_dir(dir.path()).await?;
        let mut tf = t.as_file()?;
        tf.write_all(b"hello").await?;
        tf.flush().await?;
        drop(tf);

        let _file = t.persist(&dest).await?; // consumes t
        assert_eq!(fs::read_to_string(&dest).await?, "hello");
        Ok(())
    }

    #[tokio::test]
    async fn persist_overwrites_existing_file() -> io::Result<()> {
        let dir = tempdir()?;
        let dest = dir.path().join("swap.bin");
        fs::write(&dest, b"old").await?;

        let t = TempFile::in_dir(dir.path()).await?;
        let mut tf = t.as_file()?;
        tf.write_all(b"new").await?;
        tf.flush().await?;
        drop(tf);

        t.persist(&dest).await?;
        assert_eq!(fs::read(&dest).await?, b"new");
        Ok(())
    }

    #[tokio::test]
    async fn write_then_read_from_tempfile() -> io::Result<()> {
        let t = TempFile::new().await?;
        let mut f = t.as_file()?;
        f.write_all(b"abc").await?;
        f.flush().await?;
        // Rewind before reading
        f.seek(std::io::SeekFrom::Start(0)).await?;
        let mut buf = String::new();
        f.read_to_string(&mut buf).await?;
        assert_eq!(buf, "abc");
        Ok(())
    }

    #[tokio::test]
    async fn in_dir_errors_if_not_a_directory() {
        let dir = tempdir().unwrap();
        let not_a_dir = dir.path().join("file.txt");
        std::fs::write(&not_a_dir, "x").unwrap();

        TempFile::in_dir(&not_a_dir).await.expect_err("in_dir should fail when given a file path");
    }

    #[tokio::test]
    async fn copy_copies_source_and_truncates_temp() -> io::Result<()> {
        let dir = tempdir()?;
        let src = dir.path().join("src.txt");
        fs::write(&src, b"hello").await?;

        let mut t = TempFile::in_dir(dir.path()).await?;

        {
            let mut tf = t.as_file()?;
            tf.write_all(b"AAAAAAAAAAAA").await?;
            tf.flush().await?;
        }

        t.copy(&src).await?;

        let bytes = fs::read(t.path()).await?;
        assert_eq!(bytes.as_slice(), b"hello");
        Ok(())
    }

    #[tokio::test]
    async fn copy_then_append_requires_seek_to_end() -> io::Result<()> {
        let dir = tempdir()?;
        let src = dir.path().join("src.txt");
        fs::write(&src, b"base").await?;

        let mut t = TempFile::in_dir(dir.path()).await?;
        t.copy(&src).await?;

        let mut tf = t.as_file()?;
        tf.seek(std::io::SeekFrom::End(0)).await?;
        tf.write_all(b"+more").await?;
        tf.flush().await?;

        let bytes = fs::read(t.path()).await?;
        assert_eq!(bytes.as_slice(), b"base+more");
        Ok(())
    }

    #[tokio::test]
    async fn copy_errors_when_source_missing() {
        let dir = tempdir().unwrap();
        let missing = dir.path().join("nope.txt");

        let mut t = TempFile::in_dir(dir.path()).await.unwrap();
        let err = t.copy(&missing).await.unwrap_err();

        assert_eq!(err.kind(), io::ErrorKind::NotFound);
    }
}
