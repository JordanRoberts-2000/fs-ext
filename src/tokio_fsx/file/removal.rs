use {
    std::{io, path::Path},
    tokio::{fs, task},
};

pub async fn remove(path: impl AsRef<Path>) -> io::Result<()> {
    let path = path.as_ref();
    fs::remove_file(path).await.map_err(|e| {
        io::Error::new(e.kind(), format!("failed to remove '{}': {}", path.display(), e))
    })
}

pub async fn trash(path: impl AsRef<Path>) -> io::Result<()> {
    let p = path.as_ref().to_path_buf();
    let disp = p.display().to_string();

    let res = task::spawn_blocking(move || trash::delete(&p)).await.map_err(|join_err| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("failed to run trash operation for '{}': {}", disp, join_err),
        )
    })?;

    res.map_err(|trash_err| {
        io::Error::new(io::ErrorKind::Other, format!("Failed to trash '{}': {}", disp, trash_err))
    })
}

pub async fn trash_or_remove(path: impl AsRef<Path>) -> io::Result<()> {
    let p = path.as_ref().to_path_buf();

    match trash(&p).await {
        Ok(()) => Ok(()),
        Err(trash_err) => match remove(&p).await {
            Ok(()) => Ok(()),
            Err(remove_err) => {
                let msg = format!("trash failed: {trash_err}; remove failed: {remove_err}");
                Err(io::Error::new(remove_err.kind(), msg))
            }
        },
    }
}
