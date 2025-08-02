use {
    crate::utils::create_file_or_dir_async,
    std::{io, path::Path},
    tokio::fs,
};

pub async fn async_create_if_missing(path: impl AsRef<Path>) -> io::Result<()> {
    _async_create_if_missing(path.as_ref()).await
}

async fn _async_create_if_missing(path: &Path) -> io::Result<()> {
    match fs::metadata(path).await {
        Ok(_) => Ok(()),
        Err(e) if e.kind() == io::ErrorKind::NotFound => create_file_or_dir_async(path).await,
        Err(e) => Err(e),
    }
}
