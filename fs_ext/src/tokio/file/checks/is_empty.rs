use {
    crate::{file, tokio::utils::asyncify},
    std::{io, path::Path},
};

pub async fn is_empty(path: impl AsRef<Path>) -> io::Result<bool> {
    let path = path.as_ref().to_owned();
    asyncify(move || file::is_empty(path)).await
}

#[cfg(test)]
mod tests {
    use {super::is_empty, std::io};

    #[tokio::test]
    async fn smoke_is_empty() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let file_path = dir.path().join("file.txt");

        // create an empty file
        std::fs::File::create(&file_path)?;

        // should be empty
        let result = is_empty(&file_path).await?;
        assert!(result);

        // write some content
        std::fs::write(&file_path, b"hello")?;

        // should no longer be empty
        let result = is_empty(&file_path).await?;
        assert!(!result);

        Ok(())
    }
}
