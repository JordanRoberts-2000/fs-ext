use {
    crate::{fsx, tokio::utils::asyncify},
    std::{io, path::Path},
};

pub async fn size(path: impl AsRef<Path>) -> io::Result<u128> {
    let path = path.as_ref().to_owned();
    asyncify(move || fsx::dir::size(path)).await
}
