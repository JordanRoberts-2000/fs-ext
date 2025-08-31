use {crate::tokio::fsx::dir::DirQuery, std::io};

impl DirQuery {
    pub async fn count(self) -> io::Result<usize> {
        Ok(self.collect().await?.len())
    }
}
