use crate::ExtensionFilter;

#[derive(Debug, Clone)]
pub struct DirQueryOptions {
    pub include_files: bool,
    pub include_dirs: bool,
    pub recursive: bool,
    pub limit: Option<usize>,
    pub depth: Option<usize>,
    pub extension_filter: Option<ExtensionFilter>,
}

impl Default for DirQueryOptions {
    fn default() -> Self {
        Self {
            include_files: true,
            include_dirs: true,
            recursive: true,
            limit: None,
            depth: None,
            extension_filter: None,
        }
    }
}

impl DirQueryOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn include_files(mut self, include: bool) -> Self {
        self.include_files = include;
        self
    }

    pub fn include_dirs(mut self, include: bool) -> Self {
        self.include_dirs = include;
        self
    }

    pub fn recursive(mut self, recursive: bool) -> Self {
        self.recursive = recursive;
        self
    }

    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn depth(mut self, depth: usize) -> Self {
        self.depth = Some(depth);
        self
    }

    pub fn extension_filter(mut self, filter: ExtensionFilter) -> Self {
        self.extension_filter = Some(filter);
        self
    }

    pub fn allow_extensions<I, S>(mut self, extensions: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.extension_filter = Some(ExtensionFilter::allow(extensions));
        self
    }

    pub fn deny_extensions<I, S>(mut self, extensions: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.extension_filter = Some(ExtensionFilter::deny(extensions));
        self
    }
}
