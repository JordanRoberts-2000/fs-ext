use {crate::fsx::dir::DirQuery, std::io};

impl DirQuery {
    pub fn exist(self) -> io::Result<bool> {
        Ok(self.collect()?.len() != 0)
    }
}
