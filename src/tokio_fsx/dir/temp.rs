use {
    crate::tokio::fsx::TempDir,
    std::{io, path::Path},
};

pub async fn temp() -> io::Result<TempDir> {
    TempDir::new().await
}

pub async fn temp_in(dir: impl AsRef<Path> + Send) -> io::Result<TempDir> {
    TempDir::in_dir(dir).await
}
