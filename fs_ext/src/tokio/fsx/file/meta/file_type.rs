use {
    crate::{fsx, tokio::utils::asyncify},
    std::{fs::FileType, io, path::Path},
};

pub async fn file_type(path: impl AsRef<Path>) -> io::Result<FileType> {
    let path = path.as_ref().to_owned();
    asyncify(move || fsx::file::meta::file_type(path)).await
}
