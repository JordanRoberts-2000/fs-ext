use {
    crate::{fsx, tokio_fsx::utils::asyncify},
    std::{io, path::Path},
};

pub async fn copy_dir_contents(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    let src = src.as_ref().to_owned();
    let dst = dst.as_ref().to_owned();
    asyncify(move || fsx::dir::copy_dir_contents(src, dst)).await
}
