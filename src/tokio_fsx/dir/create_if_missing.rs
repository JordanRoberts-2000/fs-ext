use {
    std::{io, path::Path},
    tokio::fs,
};

pub async fn async_dir_create_if_missing(path: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(path).await
}
