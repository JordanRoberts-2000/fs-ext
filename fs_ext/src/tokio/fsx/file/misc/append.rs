use {
    crate::{fsx, tokio::utils::asyncify},
    std::{io, path::Path},
};

pub async fn append(path: impl AsRef<Path>, content: impl AsRef<[u8]>) -> io::Result<()> {
    let path = path.as_ref().to_owned();
    let content = content.as_ref().to_owned();
    asyncify(move || fsx::file::append(path, content)).await
}
