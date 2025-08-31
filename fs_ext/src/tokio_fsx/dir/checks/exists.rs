use {
    crate::{fsx, tokio_fsx::utils::asyncify},
    std::{io, path::Path},
};

pub async fn exists(path: impl AsRef<Path>) -> io::Result<bool> {
    let path = path.as_ref().to_owned();
    asyncify(move || fsx::dir::exists(path)).await
}
