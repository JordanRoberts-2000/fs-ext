use {
    crate::tokio::fsx::TempFile,
    std::{io, path::Path},
};

pub async fn temp_in(dir: impl AsRef<Path> + Send) -> io::Result<TempFile> {
    TempFile::in_dir(dir).await
}

pub async fn temp() -> io::Result<TempFile> {
    TempFile::new().await
}
