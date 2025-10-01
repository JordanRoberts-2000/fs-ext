use {
    crate::{DirQuery, ExtensionFilter, utils::normalize_ext},
    std::path::Path,
};

impl DirQuery {
    pub(crate) fn is_extension_allowed(&self, path: &Path) -> bool {
        let Some(filter) = &self.extension_filter else {
            return true;
        };

        let ext = path.extension().and_then(|e| e.to_str()).map(|e| normalize_ext(e));

        match (ext, filter) {
            (Some(ext), ExtensionFilter::Allow(allowed)) => allowed.contains(&ext),
            (Some(ext), ExtensionFilter::Deny(denied)) => !denied.contains(&ext),

            (None, ExtensionFilter::Allow(_)) => false, // Allow list = must have a listed extension
            (None, ExtensionFilter::Deny(_)) => true, // Deny list = files without extensions are ok
        }
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        std::{
            collections::HashSet,
            path::{Path, PathBuf},
        },
    };

    fn test_query() -> DirQuery {
        DirQuery {
            root: PathBuf::from("/test"),
            include_files: true,
            include_dirs: false,
            recursive: false,
            limit: None,
            depth: None,
            extension_filter: None,
        }
    }

    #[test]
    fn no_filter_allows_everything() {
        let query = test_query();

        assert!(query.is_extension_allowed(Path::new("file.rs")));
        assert!(query.is_extension_allowed(Path::new("file.toml")));
        assert!(query.is_extension_allowed(Path::new("file.xyz")));
        assert!(query.is_extension_allowed(Path::new("Makefile")));
        assert!(query.is_extension_allowed(Path::new(".gitignore")));
    }

    #[test]
    fn allow_filter_accepts_matching_extensions() {
        let mut query = test_query();
        query.extension_filter =
            Some(ExtensionFilter::Allow(["rs", "toml"].iter().map(|s| s.to_string()).collect()));

        assert!(query.is_extension_allowed(Path::new("main.rs")));
        assert!(query.is_extension_allowed(Path::new("Cargo.toml")));
        assert!(query.is_extension_allowed(Path::new("path/to/file.rs")));
    }

    #[test]
    fn allow_filter_rejects_non_matching_extensions() {
        let mut query = test_query();
        query.extension_filter =
            Some(ExtensionFilter::Allow(["rs", "toml"].iter().map(|s| s.to_string()).collect()));

        assert!(!query.is_extension_allowed(Path::new("file.txt")));
        assert!(!query.is_extension_allowed(Path::new("script.py")));
        assert!(!query.is_extension_allowed(Path::new("data.json")));
    }

    #[test]
    fn allow_filter_rejects_files_without_extensions() {
        let mut query = test_query();
        query.extension_filter =
            Some(ExtensionFilter::Allow(["rs", "toml"].iter().map(|s| s.to_string()).collect()));

        assert!(!query.is_extension_allowed(Path::new("Makefile")));
        assert!(!query.is_extension_allowed(Path::new("README")));
        assert!(!query.is_extension_allowed(Path::new(".gitignore")));
    }

    #[test]
    fn deny_filter_rejects_matching_extensions() {
        let mut query = test_query();
        query.extension_filter = Some(ExtensionFilter::Deny(
            ["tmp", "log", "bak"].iter().map(|s| s.to_string()).collect(),
        ));

        assert!(!query.is_extension_allowed(Path::new("file.tmp")));
        assert!(!query.is_extension_allowed(Path::new("debug.log")));
        assert!(!query.is_extension_allowed(Path::new("backup.bak")));
    }

    #[test]
    fn deny_filter_accepts_non_matching_extensions() {
        let mut query = test_query();
        query.extension_filter = Some(ExtensionFilter::Deny(
            ["tmp", "log", "bak"].iter().map(|s| s.to_string()).collect(),
        ));

        assert!(query.is_extension_allowed(Path::new("main.rs")));
        assert!(query.is_extension_allowed(Path::new("data.json")));
        assert!(query.is_extension_allowed(Path::new("script.py")));
    }

    #[test]
    fn deny_filter_accepts_files_without_extensions() {
        let mut query = test_query();
        query.extension_filter = Some(ExtensionFilter::Deny(
            ["tmp", "log", "bak"].iter().map(|s| s.to_string()).collect(),
        ));

        assert!(query.is_extension_allowed(Path::new("Makefile")));
        assert!(query.is_extension_allowed(Path::new("README")));
        assert!(query.is_extension_allowed(Path::new(".gitignore")));
    }

    #[test]
    fn case_sensitivity_handled_by_normalize() {
        let mut query = test_query();
        query.extension_filter =
            Some(ExtensionFilter::Allow(["rs"].iter().map(|s| s.to_string()).collect()));

        // These should all work if normalize_ext converts to lowercase
        assert!(query.is_extension_allowed(Path::new("file.rs")));
        assert!(query.is_extension_allowed(Path::new("file.RS")));
        assert!(query.is_extension_allowed(Path::new("file.Rs")));
    }

    #[test]
    fn empty_allow_list_rejects_everything() {
        let mut query = test_query();
        query.extension_filter = Some(ExtensionFilter::Allow(HashSet::new()));

        assert!(!query.is_extension_allowed(Path::new("file.rs")));
        assert!(!query.is_extension_allowed(Path::new("file.txt")));
        assert!(!query.is_extension_allowed(Path::new("Makefile")));
    }

    #[test]
    fn empty_deny_list_allows_everything() {
        let mut query = test_query();
        query.extension_filter = Some(ExtensionFilter::Deny(HashSet::new()));

        assert!(query.is_extension_allowed(Path::new("file.rs")));
        assert!(query.is_extension_allowed(Path::new("file.txt")));
        assert!(query.is_extension_allowed(Path::new("Makefile")));
    }

    #[test]
    fn paths_with_multiple_dots() {
        let mut query = test_query();
        query.extension_filter =
            Some(ExtensionFilter::Allow(["gz"].iter().map(|s| s.to_string()).collect()));

        // Should only check the final extension
        assert!(query.is_extension_allowed(Path::new("archive.tar.gz")));
        assert!(!query.is_extension_allowed(Path::new("archive.tar")));
    }

    #[test]
    fn hidden_files_with_extensions() {
        let mut query = test_query();
        query.extension_filter =
            Some(ExtensionFilter::Allow(["toml"].iter().map(|s| s.to_string()).collect()));

        assert!(query.is_extension_allowed(Path::new(".cargo.toml")));
        assert!(!query.is_extension_allowed(Path::new(".gitignore")));
    }
}
