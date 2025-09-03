use {
    crate::{fsx, tokio::utils::asyncify},
    std::{io, path::Path},
};

pub async fn size(path: impl AsRef<Path>) -> io::Result<u64> {
    let path = path.as_ref().to_owned();
    asyncify(move || fsx::file::size(path)).await
}
