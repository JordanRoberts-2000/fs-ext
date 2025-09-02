use {
    crate::{fsx, tokio::utils::asyncify},
    std::{io, path::Path},
};

pub async fn assert_not_exists(path: impl AsRef<Path>) -> io::Result<()> {
    let path = path.as_ref().to_owned();
    asyncify(move || fsx::file::assert_not_exists(path)).await
}
