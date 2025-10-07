use std::{fs, path::Path};

pub fn file_count(dir: &Path) -> usize {
    fs::read_dir(dir).map(|entries| entries.filter_map(Result::ok).count()).unwrap_or(0)
}
