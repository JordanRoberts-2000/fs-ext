use {
    crate::{fsx, tokio::utils::asyncify},
    std::{io, path::Path},
};

pub async fn read_bytes(path: impl AsRef<Path>) -> io::Result<Vec<u8>> {
    let path = path.as_ref().to_owned();
    asyncify(move || fsx::file::read_bytes(path)).await
}

#[cfg(test)]
mod tests {
    use {super::read_bytes, std::io};

    #[tokio::test]
    async fn smoke_read_bytes() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let file_path = dir.path().join("file.txt");

        // write some bytes
        std::fs::write(&file_path, b"hello")?;

        // read them back
        let bytes = read_bytes(&file_path).await?;
        assert_eq!(bytes, b"hello");

        Ok(())
    }
}
