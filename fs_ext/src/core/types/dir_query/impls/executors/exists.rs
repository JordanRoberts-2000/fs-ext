use {crate::DirQuery, std::io};

impl DirQuery {
    pub fn exists(self) -> io::Result<bool> {
        Ok(self.collect()?.len() != 0)
    }
}
