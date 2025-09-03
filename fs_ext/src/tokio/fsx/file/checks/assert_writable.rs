use {
    crate::{fsx, tokio::utils::asyncify},
    std::{io, path::Path},
};

pub async fn assert_writable(path: impl AsRef<Path>) -> io::Result<()> {
    let path = path.as_ref().to_owned();
    asyncify(move || fsx::file::assert_writable(path)).await
}

#[cfg(test)]
mod tests {
    use {super::assert_writable, std::io};

    #[tokio::test]
    async fn smoke_assert_writable() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let file_path = dir.path().join("file.txt");

        // create a file so it exists
        std::fs::write(&file_path, b"hello")?;

        // should succeed since the file is writable
        assert_writable(&file_path).await?;

        Ok(())
    }
}
