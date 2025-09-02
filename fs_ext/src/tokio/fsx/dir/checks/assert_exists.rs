use {
    crate::{fsx, tokio::utils::asyncify},
    std::{io, path::Path},
};

pub async fn assert_exists(path: impl AsRef<Path>) -> io::Result<()> {
    let path = path.as_ref().to_owned();
    asyncify(move || fsx::dir::assert_exists(path)).await
}
