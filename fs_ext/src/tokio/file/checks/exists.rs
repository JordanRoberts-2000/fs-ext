use {
    crate::{file, tokio::utils::asyncify},
    std::{io, path::Path},
};

pub async fn exists(path: impl AsRef<Path>) -> io::Result<bool> {
    let path = path.as_ref().to_owned();
    asyncify(move || file::exists(path)).await
}

#[cfg(test)]
mod tests {
    use {super::exists, std::io};

    #[tokio::test]
    async fn smoke_exists() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let file_path = dir.path().join("file.txt");

        // file does not exist yet
        let result = exists(&file_path).await?;
        assert!(!result);

        // create the file
        std::fs::write(&file_path, b"hello")?;

        // now it should exist
        let result = exists(&file_path).await?;
        assert!(result);

        Ok(())
    }
}
