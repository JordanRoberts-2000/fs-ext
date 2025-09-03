use {
    crate::{fsx, tokio::utils::asyncify},
    std::{io, path::Path},
};

pub async fn size(path: impl AsRef<Path>) -> io::Result<u64> {
    let path = path.as_ref().to_owned();
    asyncify(move || fsx::file::size(path)).await
}

#[cfg(test)]
mod tests {
    use {super::size, std::io};

    #[tokio::test]
    async fn smoke_size() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let file_path = dir.path().join("file.txt");

        // write some bytes to the file
        std::fs::write(&file_path, b"hello")?;

        // should return correct size (5 bytes)
        let result = size(&file_path).await?;
        assert_eq!(result, 5);

        Ok(())
    }
}
