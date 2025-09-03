use {
    crate::{fsx, tokio::utils::join_err_to_io},
    std::{io, path::Path},
    tokio::task,
};

pub async fn is_writable(path: impl AsRef<Path>) -> io::Result<bool> {
    let path = path.as_ref().to_owned();
    task::spawn_blocking(|| fsx::file::is_readable(path)).await.map_err(join_err_to_io)
}
