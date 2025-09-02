use {crate::DirQuery, std::io};

impl DirQuery {
    pub fn count(self) -> io::Result<usize> {
        Ok(self.collect()?.len())
    }
}
