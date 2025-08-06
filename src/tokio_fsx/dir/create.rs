use {
    std::{io, path::Path},
    tokio::fs,
};

pub async fn create(path: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(path).await
}
