use {
    crate::{fsx, tokio::utils::asyncify},
    std::{io, path::Path},
};

pub async fn is_empty(path: impl AsRef<Path>) -> io::Result<bool> {
    let path = path.as_ref().to_owned();
    asyncify(move || fsx::file::is_empty(path)).await
}
