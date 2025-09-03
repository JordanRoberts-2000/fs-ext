use {
    crate::{fsx, tokio::utils::asyncify},
    std::{io, path::Path},
};

pub async fn read_string_or_init(
    path: impl AsRef<Path>, contents: impl AsRef<[u8]> + Send + 'static,
) -> io::Result<String> {
    let path = path.as_ref().to_owned();
    asyncify(move || fsx::file::read_string_or_init(path, contents)).await
}
