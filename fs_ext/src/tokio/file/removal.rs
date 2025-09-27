use {
    crate::{file, tokio::utils::asyncify},
    std::{io, path::Path},
};

pub async fn remove(path: impl AsRef<Path>) -> io::Result<()> {
    let path = path.as_ref().to_owned();
    asyncify(move || file::remove(path)).await
}

pub async fn trash(path: impl AsRef<Path>) -> io::Result<()> {
    let path = path.as_ref().to_owned();
    asyncify(move || file::trash(path)).await
}

pub async fn trash_or_remove(path: impl AsRef<Path>) -> io::Result<()> {
    let path = path.as_ref().to_owned();
    asyncify(move || file::trash_or_remove(path)).await
}
