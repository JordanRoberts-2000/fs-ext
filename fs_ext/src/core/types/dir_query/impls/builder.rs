use crate::DirQuery;

impl DirQuery {
    pub fn include_files(mut self, bool: bool) -> Self {
        self.include_files = bool;
        self
    }

    pub fn include_dirs(mut self, bool: bool) -> Self {
        self.include_dirs = bool;
        self
    }

    pub fn recursive(mut self, bool: bool) -> Self {
        self.recursive = bool;
        self
    }

    pub fn limit(mut self, n: usize) -> Self {
        self.limit = Some(n);
        self
    }

    pub fn depth(mut self, n: usize) -> Self {
        self.depth = Some(n);
        self
    }
}
