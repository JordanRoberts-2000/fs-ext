use {
    crate::{fsx, tokio::utils::asyncify},
    std::{io, path::Path},
};

pub async fn copy(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    let src = src.as_ref().to_owned();
    let dst = dst.as_ref().to_owned();
    asyncify(move || fsx::dir::copy(src, dst)).await
}
