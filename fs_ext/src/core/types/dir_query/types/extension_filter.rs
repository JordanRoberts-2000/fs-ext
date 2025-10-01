use {crate::utils::normalize_ext, std::collections::HashSet};

#[derive(Debug, Clone)]
pub enum ExtensionFilter {
    Allow(HashSet<String>),
    Deny(HashSet<String>),
}

impl ExtensionFilter {
    pub fn allow<I, S>(extensions: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        Self::Allow(
            extensions
                .into_iter()
                .map(|s| normalize_ext(s.as_ref()))
                .filter(|s| !s.is_empty())
                .collect(),
        )
    }

    pub fn deny<I, S>(extensions: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        Self::Deny(
            extensions
                .into_iter()
                .map(|s| normalize_ext(s.as_ref()))
                .filter(|s| !s.is_empty())
                .collect(),
        )
    }
}
