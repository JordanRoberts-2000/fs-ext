use {crate::tokio::DirQuery, std::path::Path};

pub fn entries(path: impl AsRef<Path>) -> DirQuery {
    DirQuery::new(path)
}

pub fn files(path: impl AsRef<Path>) -> DirQuery {
    DirQuery::new(path).include_dirs(false).include_files(true)
}

pub fn subdirs(path: impl AsRef<Path>) -> DirQuery {
    DirQuery::new(path).include_dirs(true).include_files(false)
}
