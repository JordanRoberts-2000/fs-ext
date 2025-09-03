use {
    crate::{fsx, tokio::utils::asyncify},
    std::{io, path::Path},
    tokio::fs::File,
};

pub async fn ensure(path: impl AsRef<Path>) -> io::Result<File> {
    let path = path.as_ref().to_owned();
    Ok(File::from_std(asyncify(move || fsx::file::ensure(path)).await?))
}
