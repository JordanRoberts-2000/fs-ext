use {
    crate::{fsx, tokio::utils::asyncify},
    std::{io, path::Path},
};

pub async fn create_new(path: impl AsRef<Path>) -> io::Result<()> {
    let path = path.as_ref().to_owned();
    asyncify(move || fsx::dir::create_new(path)).await
}
