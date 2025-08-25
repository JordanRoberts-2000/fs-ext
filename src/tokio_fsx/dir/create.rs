use {
    std::{io, path::Path},
    tokio::fs,
};

pub async fn ensure(path: impl AsRef<Path>) -> io::Result<()> {
    let p = path.as_ref();
    fs::create_dir_all(p).await.map_err(|e| {
        io::Error::new(e.kind(), format!("failed to create directory at '{}': {e}", p.display()))
    })
}

pub async fn create_new(path: impl AsRef<Path>) -> io::Result<()> {
    let p = path.as_ref();
    fs::create_dir(p).await.map_err(|e| {
        io::Error::new(
            e.kind(),
            format!("failed to create new directory at '{}': {e}", p.display()),
        )
    })
}
