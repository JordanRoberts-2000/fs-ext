use {
    crate::{file, tokio::utils::asyncify},
    std::{io, path::Path},
};

pub async fn append(path: impl AsRef<Path>, content: impl AsRef<[u8]>) -> io::Result<()> {
    let path = path.as_ref().to_owned();
    let content = content.as_ref().to_owned();
    asyncify(move || file::append(path, content)).await
}

#[cfg(test)]
mod tests {
    use {super::append, std::io};

    #[tokio::test]
    async fn smoke_append() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let file_path = dir.path().join("file.txt");

        // write initial content
        std::fs::write(&file_path, b"hello")?;

        // append some content
        append(&file_path, b" world").await?;

        // file should now contain "hello world"
        let content = std::fs::read_to_string(&file_path)?;
        assert_eq!(content, "hello world");

        Ok(())
    }
}
