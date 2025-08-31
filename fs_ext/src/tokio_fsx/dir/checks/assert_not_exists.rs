use {
    crate::{fsx, tokio_fsx::utils::asyncify},
    std::{io, path::Path},
};

pub async fn assert_not_exists(path: impl AsRef<Path>) -> io::Result<()> {
    let path = path.as_ref().to_owned();
    asyncify(move || fsx::dir::assert_not_exists(path)).await
}
