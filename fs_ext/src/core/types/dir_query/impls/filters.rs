use crate::{DirQuery, ExtensionFilter};

impl DirQuery {
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
