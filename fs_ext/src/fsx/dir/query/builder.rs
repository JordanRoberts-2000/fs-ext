use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone)]
pub struct DirQuery {
    pub(crate) root: PathBuf,
    pub(crate) include_files: bool,
    pub(crate) include_dirs: bool,
    pub(crate) recursive: bool,
    pub(crate) limit: Option<usize>,
    pub(crate) depth: Option<usize>,
    pub(crate) allow_exts: HashSet<String>,
    pub(crate) deny_exts: HashSet<String>,
}

impl DirQuery {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            root: path.as_ref().to_path_buf(),
            include_files: true,
            include_dirs: true,
            recursive: true,
            limit: None,
            depth: None,
            allow_exts: HashSet::new(),
            deny_exts: HashSet::new(),
        }
    }

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
