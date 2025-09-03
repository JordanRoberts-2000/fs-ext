use {
    crate::{fsx, tokio::utils::asyncify},
    std::{io, path::Path, time::SystemTime},
};

pub async fn created(path: impl AsRef<Path>) -> io::Result<SystemTime> {
    let path = path.as_ref().to_owned();
    asyncify(move || fsx::file::meta::created(path)).await
}
