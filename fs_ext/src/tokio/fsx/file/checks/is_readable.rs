use {
    crate::{fsx, tokio::utils::join_err_to_io},
    std::{io, path::Path},
    tokio::task,
};

pub async fn is_readable(path: impl AsRef<Path>) -> io::Result<bool> {
    let path = path.as_ref().to_owned();
    task::spawn_blocking(|| fsx::file::is_readable(path)).await.map_err(join_err_to_io)
}

#[cfg(test)]
mod tests {
    use {super::is_readable, std::io};

    #[tokio::test]
    async fn smoke_is_readable() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let file_path = dir.path().join("file.txt");

        // create a file
        std::fs::write(&file_path, b"hello")?;

        // should be readable
        let result = is_readable(&file_path).await?;
        assert!(result);

        Ok(())
    }
}
