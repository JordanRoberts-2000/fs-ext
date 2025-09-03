use {
    crate::{fsx, tokio::utils::join_err_to_io},
    std::{io, path::Path},
    tokio::task,
};

pub async fn is_writable(path: impl AsRef<Path>) -> io::Result<bool> {
    let path = path.as_ref().to_owned();
    task::spawn_blocking(|| fsx::file::is_writable(path)).await.map_err(join_err_to_io)
}

#[cfg(test)]
mod tests {
    use {super::is_writable, std::io};

    #[tokio::test]
    async fn smoke_is_writable() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let file_path = dir.path().join("file.txt");

        // create a file
        std::fs::write(&file_path, b"hello")?;

        // should be writable
        let result = is_writable(&file_path).await?;
        assert!(result);

        Ok(())
    }
}
