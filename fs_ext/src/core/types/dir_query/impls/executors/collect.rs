use {
    crate::DirQuery,
    std::{io, path::PathBuf},
    walkdir::WalkDir,
};

impl DirQuery {
    pub fn collect(self) -> io::Result<Vec<PathBuf>> {
        let mut entries = WalkDir::new(&self.root).min_depth(1);

        if !self.recursive {
            entries = entries.max_depth(1);
        } else if let Some(max_depth) = self.depth {
            entries = entries.max_depth(max_depth);
        }

        let mut results = Vec::new();

        for entry in entries {
            let entry = entry.map_err(|e| {
                let kind = e.io_error().map(|ioe| ioe.kind()).unwrap_or(io::ErrorKind::Other);
                let msg = match e.path() {
                    Some(p) => format!("walk error at '{}': {}", p.display(), e),
                    None => format!("walk error: {}", e),
                };
                io::Error::new(kind, msg)
            })?;

            let entry_path = entry.path();

            let metadata = entry.metadata().map_err(|meta_err| {
                io::Error::new(
                    meta_err.io_error().map(|ioe| ioe.kind()).unwrap_or(io::ErrorKind::Other),
                    format!(
                        "metadata error under '{}' at '{}': {}",
                        self.root.display(),
                        entry_path.display(),
                        meta_err
                    ),
                )
            })?;

            let is_dir = metadata.is_dir();
            let is_file = metadata.is_file();

            let should_include = if is_dir && self.include_dirs {
                true
            } else if is_file && self.include_files {
                self.is_extension_allowed(entry_path)
            } else {
                false
            };

            if should_include {
                results.push(entry_path.to_path_buf());

                // Check limit
                if let Some(limit) = self.limit {
                    if results.len() >= limit {
                        break;
                    }
                }
            }
        }

        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        std::{collections::BTreeSet, fs, io::Write, path::Path},
        tempfile::tempdir,
    };

    fn touch(path: &Path) {
        let mut f = std::fs::File::create(path).unwrap();
        f.write_all(b"x").unwrap();
    }

    fn as_set<I: IntoIterator<Item = PathBuf>>(it: I) -> BTreeSet<PathBuf> {
        it.into_iter().collect()
    }

    #[test]
    fn skips_root_entry() -> io::Result<()> {
        let d = tempdir()?;
        let root = d.path();
        let sub = root.join("sub");
        fs::create_dir(&sub)?;
        touch(&root.join("a.txt"));

        let got = DirQuery::new(root).collect()?;
        assert!(!got.iter().any(|p| p == root), "root should not be yielded");
        Ok(())
    }

    #[test]
    fn files_non_recursive_only_lists_direct_children() -> io::Result<()> {
        let d = tempdir()?;
        let root = d.path();
        fs::create_dir(root.join("sub"))?;
        touch(&root.join("a.txt"));
        touch(&root.join("b.md"));
        touch(&root.join("sub/c.txt"));

        let q = DirQuery::new(root).include_dirs(false).include_files(true).recursive(false);

        let got = q.collect()?;
        assert_eq!(
            as_set(got),
            as_set([root.join("a.txt"), root.join("b.md")]),
            "non-recursive should not include subdir files"
        );
        Ok(())
    }

    #[test]
    fn recursive_with_depth_respects_max_depth_for_dirs() -> io::Result<()> {
        let d = tempdir()?;
        let root = d.path();
        let a = root.join("a");
        let b = a.join("b");
        fs::create_dir_all(&b)?;
        // depth(1) means: root depth=0; children depth=1; grandchildren depth=2 (excluded)
        let q =
            DirQuery::new(root).include_dirs(true).include_files(false).recursive(true).depth(1);

        let got = q.collect()?;
        assert!(got.contains(&a), "first-level dir should be included");
        assert!(!got.contains(&b), "second-level dir should be excluded by depth");
        Ok(())
    }

    #[test]
    fn limit_is_enforced_early() -> io::Result<()> {
        let d = tempdir()?;
        let root = d.path();
        for i in 0..5 {
            touch(&root.join(format!("f{i}.txt")));
        }
        for i in 0..5 {
            fs::create_dir(root.join(format!("d{i}")))?;
        }

        let q = DirQuery::new(root).include_dirs(true).include_files(true).limit(3);

        let got = q.collect()?;
        assert_eq!(got.len(), 3, "limit should cap the number of results");
        Ok(())
    }

    #[test]
    fn missing_root_produces_error() {
        let d = tempdir().unwrap();
        let missing = d.path().join("does_not_exist");
        let err = DirQuery::new(&missing).collect().unwrap_err();
        // Error text comes from walkdir mapping; check it mentions "walk error"
        assert!(err.to_string().contains("walk error"), "expected mapped walk error, got: {err}");
    }

    #[test]
    fn allow_extensions_only_includes_allowed() -> io::Result<()> {
        let d = tempdir()?;
        let root = d.path();
        touch(&root.join("a.rs"));
        touch(&root.join("b.txt"));
        touch(&root.join("c.RS")); // case-insensitive after normalize_ext

        let q =
            DirQuery::new(root).include_dirs(false).include_files(true).allow_extensions(["rs"]);

        let got = q.collect()?;
        assert_eq!(
            as_set(got),
            as_set([root.join("a.rs"), root.join("c.RS")]),
            "only .rs files should pass when allow list is set"
        );
        Ok(())
    }

    #[test]
    fn deny_extensions_excludes_denied() -> io::Result<()> {
        let d = tempdir()?;
        let root = d.path();
        touch(&root.join("a.txt"));
        touch(&root.join("b.tmp"));

        let q =
            DirQuery::new(root).include_dirs(false).include_files(true).deny_extensions(["tmp"]);

        let got = q.collect()?;
        assert_eq!(
            as_set(got),
            as_set([root.join("a.txt")]),
            "denied extensions should be excluded"
        );
        Ok(())
    }

    #[test]
    fn no_extension_filter_includes_all_files() -> io::Result<()> {
        let d = tempdir()?;
        let root = d.path();
        touch(&root.join("a.rs"));
        touch(&root.join("b.txt"));
        touch(&root.join("c.tmp"));

        let q = DirQuery::new(root).include_dirs(false).include_files(true);

        let got = q.collect()?;
        assert_eq!(
            as_set(got),
            as_set([root.join("a.rs"), root.join("b.txt"), root.join("c.tmp")]),
            "all files should be included when no filter is set"
        );
        Ok(())
    }

    #[test]
    fn allow_extensions_excludes_files_without_extensions() -> io::Result<()> {
        let d = tempdir()?;
        let root = d.path();
        touch(&root.join("main.rs"));
        touch(&root.join("Makefile"));
        touch(&root.join("README"));

        let q =
            DirQuery::new(root).include_dirs(false).include_files(true).allow_extensions(["rs"]);

        let got = q.collect()?;
        assert_eq!(
            as_set(got),
            as_set([root.join("main.rs")]),
            "allow filter should exclude files without extensions"
        );
        Ok(())
    }

    #[test]
    fn deny_extensions_includes_files_without_extensions() -> io::Result<()> {
        let d = tempdir()?;
        let root = d.path();
        touch(&root.join("debug.log"));
        touch(&root.join("Makefile"));
        touch(&root.join("README"));

        let q =
            DirQuery::new(root).include_dirs(false).include_files(true).deny_extensions(["log"]);

        let got = q.collect()?;
        assert_eq!(
            as_set(got),
            as_set([root.join("Makefile"), root.join("README")]),
            "deny filter should include files without extensions"
        );
        Ok(())
    }

    #[test]
    fn extension_filter_with_multiple_extensions() -> io::Result<()> {
        let d = tempdir()?;
        let root = d.path();
        touch(&root.join("main.rs"));
        touch(&root.join("Cargo.toml"));
        touch(&root.join("README.md"));
        touch(&root.join("script.py"));

        let q = DirQuery::new(root)
            .include_dirs(false)
            .include_files(true)
            .allow_extensions(["rs", "toml", "md"]);

        let got = q.collect()?;
        assert_eq!(
            as_set(got),
            as_set([root.join("main.rs"), root.join("Cargo.toml"), root.join("README.md")]),
            "multiple allowed extensions should all be included"
        );
        Ok(())
    }
}
