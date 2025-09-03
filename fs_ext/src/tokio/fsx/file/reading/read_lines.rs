use {
    crate::{fsx, tokio::utils::asyncify},
    std::{io, path::Path},
};

pub async fn read_lines(path: impl AsRef<Path>) -> io::Result<Vec<String>> {
    let path = path.as_ref().to_owned();
    asyncify(move || fsx::file::read_lines(path)).await
}
