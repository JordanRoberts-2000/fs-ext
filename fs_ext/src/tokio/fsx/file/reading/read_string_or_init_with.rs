use {
    crate::{fsx, tokio::utils::asyncify},
    std::{io, path::Path},
};

pub async fn read_string_or_init_with<F, C>(
    path: impl AsRef<Path>, contents_fn: F,
) -> io::Result<String>
where
    F: FnOnce() -> C + Send + 'static,
    C: AsRef<[u8]>,
{
    let path = path.as_ref().to_owned();
    asyncify(move || fsx::file::read_string_or_init_with(path, contents_fn)).await
}
