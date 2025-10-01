use {
    crate::ExtensionFilter,
    std::path::{Path, PathBuf},
};

#[derive(Debug, Clone)]
pub struct DirQuery {
    pub(crate) root: PathBuf,
    pub(crate) include_files: bool,
    pub(crate) include_dirs: bool,
    pub(crate) recursive: bool,
    pub(crate) limit: Option<usize>,
    pub(crate) depth: Option<usize>,
    pub(crate) extension_filter: Option<ExtensionFilter>,
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
            extension_filter: None,
        }
    }
}
