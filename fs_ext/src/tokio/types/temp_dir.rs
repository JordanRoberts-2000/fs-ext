use {
    crate::{TempDir as SyncTempDir, tokio::utils::join_err_to_io},
    std::{
        io,
        path::{Path, PathBuf},
    },
    tokio::task,
};

#[derive(Debug)]
pub struct TempDir {
    inner: SyncTempDir,
}

impl TempDir {
    pub async fn new() -> io::Result<Self> {
        let sync_tempdir =
            task::spawn_blocking(SyncTempDir::new).await.map_err(join_err_to_io)??;
        Ok(Self { inner: sync_tempdir })
    }

    pub async fn in_dir(path: impl AsRef<Path>) -> io::Result<Self> {
        let path = path.as_ref().to_owned();

        let sync_tempdir = task::spawn_blocking(move || SyncTempDir::in_dir(&path))
            .await
            .map_err(join_err_to_io)??;
        Ok(Self { inner: sync_tempdir })
    }

    pub fn path(&self) -> &Path {
        self.inner.path()
    }

    pub fn keep(self) -> PathBuf {
        self.inner.keep()
    }

    pub async fn close(self) -> io::Result<()> {
        task::spawn_blocking(move || self.inner.close()).await.map_err(join_err_to_io)?
    }
}

#[cfg(test)]
mod tests {
    use {super::*, tokio::fs as tokio_fs};

    #[tokio::test]
    async fn test_tempdir_new() {
        let tempdir = TempDir::new().await.expect("Failed to create temp dir");

        let path = tempdir.path();
        assert!(path.exists(), "Temp dir path should exist");
        assert!(path.is_dir(), "Temp dir path should be a directory");
    }

    #[tokio::test]
    async fn test_tempdir_in_dir() {
        let base_temp_dir = std::env::temp_dir();
        let tempdir = TempDir::in_dir(&base_temp_dir)
            .await
            .expect("Failed to create temp dir in specified dir");

        let path = tempdir.path();
        assert!(path.exists(), "Temp dir should exist");
        assert!(path.is_dir(), "Temp dir should be a directory");

        // Check that it's actually in the specified parent directory
        let parent = path.parent().expect("Temp dir should have a parent");
        assert_eq!(parent, base_temp_dir, "Temp dir should be in specified parent directory");
    }

    #[tokio::test]
    async fn test_path_consistency() {
        let tempdir = TempDir::new().await.expect("Failed to create temp dir");
        let path1 = tempdir.path();
        let path2 = tempdir.path();

        // Path should be consistent between calls
        assert_eq!(path1, path2, "Path should be consistent between calls");
        assert!(path1.exists(), "Path should exist");
        assert!(path1.is_dir(), "Path should be a directory");
    }

    #[tokio::test]
    async fn test_keep() {
        let tempdir = TempDir::new().await.expect("Failed to create temp dir");
        let original_path = tempdir.path().to_owned();

        // Create a test file in the directory
        let test_file_path = original_path.join("keep_test.txt");
        tokio_fs::write(&test_file_path, b"keep test data")
            .await
            .expect("Failed to write test file");

        // Keep the directory
        let kept_path = tempdir.keep();

        // Verify the directory and its contents still exist
        assert_eq!(kept_path, original_path, "Kept path should match original path");
        assert!(kept_path.exists(), "Kept directory should still exist");
        assert!(test_file_path.exists(), "Test file in kept directory should still exist");

        // Verify content is preserved
        let content = tokio_fs::read(&test_file_path).await.expect("Failed to read kept file");
        assert_eq!(content, b"keep test data", "Kept file should preserve content");

        // Manual cleanup since keep() prevents automatic cleanup
        tokio_fs::remove_dir_all(&kept_path).await.ok();
    }

    #[tokio::test]
    async fn test_close() {
        let tempdir = TempDir::new().await.expect("Failed to create temp dir");
        let path = tempdir.path().to_owned();

        // Create a test file to verify cleanup
        let test_file_path = path.join("close_test.txt");
        tokio_fs::write(&test_file_path, b"close test data")
            .await
            .expect("Failed to write test file");

        assert!(path.exists(), "Temp dir should exist before close");
        assert!(test_file_path.exists(), "Test file should exist before close");

        // Close the temp directory
        tempdir.close().await.expect("Failed to close temp dir");

        // Directory should be cleaned up
        assert!(!path.exists(), "Temp dir should not exist after close");
        assert!(!test_file_path.exists(), "Test file should not exist after close");
    }
}
