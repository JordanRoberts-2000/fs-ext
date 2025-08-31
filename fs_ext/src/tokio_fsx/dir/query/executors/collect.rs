use {
    crate::{tokio::fsx::dir::DirQuery, utils::normalize_ext},
    std::{
        io,
        path::{Path, PathBuf},
    },
    walkdir::WalkDir,
};

impl DirQuery {
    pub async fn collect(self) -> io::Result<Vec<PathBuf>> {
        tokio::task::spawn_blocking(move || self.sync_collect())
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("join error: {e}")))?
    }

    fn sync_collect(self) -> io::Result<Vec<PathBuf>> {
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
                self.matches_extension_filter(entry_path)
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

    fn matches_extension_filter(&self, path: &Path) -> bool {
        let ext = path.extension().and_then(|e| e.to_str()).and_then(|e| normalize_ext(e));

        match ext {
            Some(ext) => {
                // If there are allowed extensions, the file must have one of them
                if !self.allow_exts.is_empty() && !self.allow_exts.contains(&ext) {
                    return false;
                }

                // If there are denied extensions, the file must not have one of them
                if !self.deny_exts.is_empty() && self.deny_exts.contains(&ext) {
                    return false;
                }

                true
            }
            None => {
                // Files without extensions are included only if no allow filter is set
                self.allow_exts.is_empty() && self.deny_exts.is_empty()
            }
        }
    }
}
