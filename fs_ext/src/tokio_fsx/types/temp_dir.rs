use {
    std::{
        io,
        path::{Path, PathBuf},
    },
    tempfile::{Builder, TempDir as TfTempDir},
    tokio::task,
};

#[derive(Debug)]
pub struct TempDir(TfTempDir);

impl TempDir {
    pub async fn new() -> io::Result<Self> {
        let td = task::spawn_blocking(TfTempDir::new).await.map_err(join_err)??;
        Ok(Self(td))
    }

    pub async fn in_dir(dir: impl AsRef<Path>) -> io::Result<Self> {
        let dir = dir.as_ref().to_path_buf();
        let td = task::spawn_blocking(move || Builder::new().prefix(".tmp-").tempdir_in(dir))
            .await
            .map_err(join_err)??;
        Ok(Self(td))
    }

    pub fn path(&self) -> &Path {
        self.0.path()
    }

    pub fn keep(self) -> PathBuf {
        self.0.keep()
    }

    pub async fn close(self) -> io::Result<()> {
        task::spawn_blocking(move || self.0.close()).await.map_err(join_err)?
    }
}

fn join_err(e: tokio::task::JoinError) -> io::Error {
    io::Error::new(io::ErrorKind::Other, format!("blocking task failed: {e}"))
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        tempfile::tempdir,
        tokio::{fs, io::AsyncWriteExt},
    };

    #[tokio::test]
    async fn new_creates_and_deletes_on_drop() -> io::Result<()> {
        let path = {
            let t = TempDir::new().await?;
            let p = t.path().to_path_buf();
            assert!(p.exists(), "temp dir should exist while handle is alive");
            p
        };
        assert!(!path.exists(), "temp dir should be removed on Drop after scope ends");
        Ok(())
    }

    #[tokio::test]
    async fn in_dir_places_tempdir_in_provided_dir() -> io::Result<()> {
        let parent = tempdir()?;
        let t = TempDir::in_dir(parent.path()).await?;
        assert_eq!(t.path().parent().unwrap(), parent.path());
        Ok(())
    }

    #[tokio::test]
    async fn keep_stops_autodelete_and_returns_path() -> io::Result<()> {
        let t = TempDir::new().await?;
        let p = t.keep(); // auto-delete disabled
        assert!(p.exists(), "kept dir should remain after wrapper is dropped");

        // clean up to keep tests tidy
        fs::remove_dir_all(&p).await?;
        Ok(())
    }

    #[tokio::test]
    async fn close_removes_directory_now() -> io::Result<()> {
        let t = TempDir::new().await?;
        let p = t.path().to_path_buf();

        let mut f = fs::File::create(p.join("file.txt")).await?;
        f.write_all(b"hello").await?;
        f.sync_all().await?;
        drop(f);

        t.close().await?; // consumes and removes
        assert!(!p.exists(), "dir should be gone after close()");
        Ok(())
    }

    #[tokio::test]
    async fn in_dir_fails_on_non_directory() {
        let d = tempdir().unwrap();
        let not_a_dir = d.path().join("file.txt");
        std::fs::write(&not_a_dir, "x").unwrap();

        TempDir::in_dir(&not_a_dir).await.expect_err("should fail when provided non-directory");
    }
}
