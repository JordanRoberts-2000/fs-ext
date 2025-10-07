use {
    crate::{TempFile as SyncTempFile, tokio::utils::join_err_to_io},
    std::{
        io,
        path::{Path, PathBuf},
    },
    tokio::{fs::File as TokioFile, task},
};

#[derive(Debug)]
pub struct TempFile {
    pub(crate) inner: SyncTempFile,
}

impl TempFile {
    pub async fn new() -> io::Result<Self> {
        let sync_tempfile =
            task::spawn_blocking(SyncTempFile::new).await.map_err(join_err_to_io)??;
        Ok(Self { inner: sync_tempfile })
    }

    pub async fn in_dir(dir: impl AsRef<Path>) -> io::Result<Self> {
        let dir = dir.as_ref().to_owned();

        let sync_tempfile =
            task::spawn_blocking(|| SyncTempFile::in_dir(dir)).await.map_err(join_err_to_io)??;
        Ok(Self { inner: sync_tempfile })
    }

    pub fn as_file(&self) -> io::Result<TokioFile> {
        let file = self.inner.as_file().try_clone()?;
        Ok(TokioFile::from_std(file))
    }

    pub fn path(&self) -> &Path {
        self.inner.path()
    }

    pub async fn persist(self, path: impl AsRef<Path>) -> io::Result<TokioFile> {
        let path = path.as_ref().to_owned();

        let file = task::spawn_blocking(move || self.inner.persist(&path))
            .await
            .map_err(join_err_to_io)??;
        Ok(TokioFile::from_std(file))
    }

    pub async fn persist_new(self, path: impl AsRef<Path>) -> io::Result<TokioFile> {
        let path = path.as_ref().to_owned();

        let file = task::spawn_blocking(move || self.inner.persist_new(&path))
            .await
            .map_err(join_err_to_io)??;
        Ok(TokioFile::from_std(file))
    }

    pub async fn keep(self) -> io::Result<(TokioFile, PathBuf)> {
        let (file, path) =
            task::spawn_blocking(move || self.inner.keep()).await.map_err(join_err_to_io)??;
        Ok((TokioFile::from_std(file), path))
    }

    pub async fn copy_from(&mut self, path: impl AsRef<Path>) -> io::Result<()> {
        let path = path.as_ref().to_owned();

        task::block_in_place(|| self.inner.copy_from(&path))
    }
}

#[cfg(test)]
mod tests {
    use {super::*, std::fs, tokio::io::AsyncWriteExt};

    #[tokio::test]
    async fn test_tempfile_new() {
        let tempfile = TempFile::new().await.expect("Failed to create temp file");

        let path = tempfile.path();
        assert!(path.exists(), "Temp file path should exist");
        assert!(path.is_file(), "Temp file path should be a file");
    }

    #[tokio::test]
    async fn test_tempfile_in_dir() {
        let temp_dir = std::env::temp_dir();
        let tempfile =
            TempFile::in_dir(&temp_dir).await.expect("Failed to create temp file in dir");

        let path = tempfile.path();
        assert!(path.exists(), "Temp file should exist");
        assert!(path.parent().unwrap() == temp_dir, "Temp file should be in specified directory");
    }

    #[tokio::test]
    async fn test_as_file() {
        let tempfile = TempFile::new().await.expect("Failed to create temp file");
        let mut file = tempfile.as_file().expect("Failed to get file handle");

        // Write some data
        let test_data = b"Hello, World!";
        file.write_all(test_data).await.expect("Failed to write to file");
        drop(file); // Close the file handle

        // Verify by reading from filesystem
        let content = tokio::fs::read(tempfile.path()).await.expect("Failed to read file");
        assert_eq!(content, test_data, "Written data should match read data");
    }

    #[tokio::test]
    async fn test_persist() {
        let tempfile = TempFile::new().await.expect("Failed to create temp file");
        let original_path = tempfile.path().to_owned();

        // Write some test data first
        let mut file = tempfile.as_file().expect("Failed to get file handle");
        let test_data = b"persist test data";
        file.write_all(test_data).await.expect("Failed to write test data");
        drop(file); // Close file before persisting

        // Create a target path
        let target_path = std::env::temp_dir().join("test_persist.txt");
        if target_path.exists() {
            fs::remove_file(&target_path).ok(); // Clean up if exists
        }

        let _persisted_file = tempfile.persist(&target_path).await.expect("Failed to persist file");

        // Verify the file was moved
        assert!(!original_path.exists(), "Original temp file should not exist after persist");
        assert!(target_path.exists(), "Persisted file should exist");

        // Verify content
        let content = fs::read(&target_path).expect("Failed to read persisted file");
        assert_eq!(&content, test_data, "Persisted file should contain original data");

        // Cleanup
        fs::remove_file(&target_path).ok();
    }

    #[tokio::test]
    async fn test_persist_new() {
        let tempfile = TempFile::new().await.expect("Failed to create temp file");

        // Write some test data
        let mut file = tempfile.as_file().expect("Failed to get file handle");
        let test_data = b"persist_new test data";
        file.write_all(test_data).await.expect("Failed to write test data");
        drop(file);

        let target_path = std::env::temp_dir().join("test_persist_new.txt");
        if target_path.exists() {
            fs::remove_file(&target_path).ok();
        }

        let _persisted_file =
            tempfile.persist_new(&target_path).await.expect("Failed to persist_new file");

        assert!(target_path.exists(), "persist_new file should exist");

        let content = fs::read(&target_path).expect("Failed to read persist_new file");
        assert_eq!(&content, test_data, "persist_new file should contain original data");

        // Cleanup
        fs::remove_file(&target_path).ok();
    }

    #[tokio::test]
    async fn test_keep() {
        let tempfile = TempFile::new().await.expect("Failed to create temp file");

        // Write some test data
        let mut file = tempfile.as_file().expect("Failed to get file handle");
        let test_data = b"keep test data";
        file.write_all(test_data).await.expect("Failed to write test data");
        drop(file);

        let (_kept_file, kept_path) = tempfile.keep().await.expect("Failed to keep file");

        // Verify file still exists and has correct content
        assert!(kept_path.exists(), "Kept file should exist");
        let content = fs::read(&kept_path).expect("Failed to read kept file");
        assert_eq!(&content, test_data, "Kept file should contain original data");

        // Cleanup (since keep() prevents automatic cleanup)
        fs::remove_file(&kept_path).ok();
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_copy_from() -> io::Result<()> {
        use std::fs;

        let mut tempfile = TempFile::new().await?;

        // Put *different* content in the temp file first (so we can verify overwrite)
        {
            let mut f = tempfile.as_file()?;
            f.write_all(b"original").await?;
            f.flush().await?;
        } // drop handle

        // Prepare a SOURCE file on disk
        let src_path = std::env::temp_dir().join("test_copy_from_src.txt");
        if src_path.exists() {
            let _ = fs::remove_file(&src_path);
        }
        fs::write(&src_path, b"copy test data")?;

        // Copy from source path INTO the temp file
        tempfile.copy_from(&src_path).await?;

        // Verify: temp file exists and matches the source contents
        assert!(tempfile.path().exists(), "temp file should still exist");
        assert!(src_path.exists(), "source file should exist");

        let temp_bytes = fs::read(tempfile.path())?;
        let src_bytes = fs::read(&src_path)?;
        assert_eq!(temp_bytes, src_bytes, "temp should match source after copy_from");
        assert_eq!(temp_bytes, b"copy test data");

        // Cleanup the source file
        let _ = fs::remove_file(&src_path);
        Ok(())
    }
}
