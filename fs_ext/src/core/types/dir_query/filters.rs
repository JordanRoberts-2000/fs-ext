use crate::{DirQuery, utils::normalize_ext};

impl DirQuery {
    pub fn filter_extension(mut self, ext: impl AsRef<str>) -> Self {
        if let Some(e) = normalize_ext(ext.as_ref()) {
            self.allow_exts.insert(e);
        }
        self
    }

    pub fn filter_extensions<I, S>(mut self, exts: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        for ext in exts {
            if let Some(e) = normalize_ext(ext.as_ref()) {
                self.allow_exts.insert(e);
            }
        }
        self
    }

    pub fn exclude_extension(mut self, ext: impl AsRef<str>) -> Self {
        if let Some(e) = normalize_ext(ext.as_ref()) {
            self.deny_exts.insert(e);
        }
        self
    }

    pub fn exclude_extensions<I, S>(mut self, exts: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        for ext in exts {
            if let Some(e) = normalize_ext(ext.as_ref()) {
                self.deny_exts.insert(e);
            }
        }
        self
    }
}
