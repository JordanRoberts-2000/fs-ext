use {crate::tokio::fsx::dir::DirQuery, std::io};

impl DirQuery {
    pub async fn exist(self) -> io::Result<bool> {
        Ok(self.collect().await?.len() != 0)
    }
}
