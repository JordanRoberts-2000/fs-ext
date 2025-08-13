use {
    crate::tokio::fsx::TempFile,
    std::{io, path::Path},
};

pub async fn temp_in(dir: impl AsRef<Path> + Send + 'static) -> io::Result<TempFile> {
    TempFile::in_dir(dir).await
}
