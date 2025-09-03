use {
    crate::{fsx, tokio::utils::asyncify},
    std::{io, path::Path},
    tokio::fs::File,
};

pub async fn write_only(path: impl AsRef<Path>) -> io::Result<File> {
    let path = path.as_ref().to_owned();
    Ok(File::from_std(asyncify(move || fsx::file::open::write_only(path)).await?))
}

pub async fn read_only(path: impl AsRef<Path>) -> io::Result<File> {
    let path = path.as_ref().to_owned();
    Ok(File::from_std(asyncify(move || fsx::file::open::read_only(path)).await?))
}
