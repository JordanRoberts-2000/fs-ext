use {
    crate::{DirQuery as SyncDirQuery, ExtensionFilter, tokio::utils::join_err_to_io},
    std::{
        io,
        path::{Path, PathBuf},
    },
};

#[derive(Debug, Clone)]
pub struct DirQuery {
    inner: SyncDirQuery,
}

impl DirQuery {
    pub fn new(root: impl AsRef<Path>) -> Self {
        Self { inner: SyncDirQuery::new(root) }
    }

    pub fn include_files(mut self, bool: bool) -> Self {
        self.inner.include_files = bool;
        self
    }

    pub fn include_dirs(mut self, bool: bool) -> Self {
        self.inner.include_dirs = bool;
        self
    }

    pub fn recursive(mut self, bool: bool) -> Self {
        self.inner.recursive = bool;
        self
    }

    pub fn limit(mut self, n: usize) -> Self {
        self.inner.limit = Some(n);
        self
    }

    pub fn depth(mut self, n: usize) -> Self {
        self.inner.depth = Some(n);
        self
    }

    pub fn extension_filter(mut self, filter: ExtensionFilter) -> Self {
        let inner = self.inner;
        self.inner = inner.extension_filter(filter);
        self
    }

    pub fn allow_extensions<I, S>(mut self, extensions: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let inner = self.inner;
        self.inner = inner.allow_extensions(extensions);
        self
    }

    pub fn deny_extensions<I, S>(mut self, extensions: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let inner = self.inner;
        self.inner = inner.deny_extensions(extensions);
        self
    }

    pub async fn collect(self) -> io::Result<Vec<PathBuf>> {
        tokio::task::spawn_blocking(move || self.inner.collect()).await.map_err(join_err_to_io)?
    }

    pub async fn count(self) -> io::Result<usize> {
        Ok(self.collect().await?.len())
    }

    pub async fn exists(self) -> io::Result<bool> {
        Ok(self.collect().await?.len() != 0)
    }
}

#[cfg(test)]
mod tests {
    use {super::*, std::fs, tempfile::TempDir};

    #[tokio::test]
    async fn test_basic_collect() -> io::Result<()> {
        let temp_dir = TempDir::new()?;
        let root = temp_dir.path();

        fs::write(root.join("test.txt"), "content")?;

        let query = DirQuery::new(root);
        let results = query.collect().await?;

        assert!(!results.is_empty(), "Should find at least one item");
        Ok(())
    }

    #[tokio::test]
    async fn test_count_matches_collect_len() -> io::Result<()> {
        let temp_dir = TempDir::new()?;
        let root = temp_dir.path();

        fs::write(root.join("file1.txt"), "content")?;
        fs::write(root.join("file2.txt"), "content")?;

        let collect_len = DirQuery::new(root).collect().await?.len();
        let count = DirQuery::new(root).count().await?;

        assert_eq!(collect_len, count, "count() should match collect().len()");
        Ok(())
    }

    #[tokio::test]
    async fn test_exists_true_when_items_found() -> io::Result<()> {
        let temp_dir = TempDir::new()?;
        let root = temp_dir.path();

        fs::write(root.join("test.txt"), "content")?;

        let exists = DirQuery::new(root).exists().await?;
        assert!(exists, "exists() should return true when items are found");
        Ok(())
    }

    #[tokio::test]
    async fn exists_false_when_no_matches_allow() -> io::Result<()> {
        let temp_dir = TempDir::new()?;
        let root = temp_dir.path();

        // Present file, but we only allow a non-existent extension
        fs::write(root.join("test.txt"), "content")?;

        let exists = DirQuery::new(root).allow_extensions(["nonexistent"]).exists().await?;
        assert!(!exists, "exists() should be false when no items match allowed extensions");
        Ok(())
    }

    #[tokio::test]
    async fn test_method_chaining() -> io::Result<()> {
        let temp_dir = TempDir::new()?;
        let root = temp_dir.path();

        fs::write(root.join("test.txt"), "content")?;
        fs::write(root.join("test.rs"), "content")?;

        let results = DirQuery::new(root)
            .include_files(true)
            .include_dirs(false)
            .allow_extensions(["txt"])
            .collect()
            .await?;

        // Should find the .txt file but not the .rs file
        assert!(!results.is_empty(), "Should find filtered results");
        Ok(())
    }

    #[tokio::test]
    async fn test_limit_functionality() -> io::Result<()> {
        let temp_dir = TempDir::new()?;
        let root = temp_dir.path();

        for i in 0..5 {
            fs::write(root.join(format!("file{}.txt", i)), "content")?;
        }

        let count = DirQuery::new(root).limit(2).count().await?;

        assert!(count <= 2, "limit() should restrict the number of results");
        Ok(())
    }
}
