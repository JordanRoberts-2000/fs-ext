use {
    crate::{file, tokio::utils::asyncify},
    std::{io, path::Path},
};

pub async fn assert_exists(path: impl AsRef<Path>) -> io::Result<()> {
    let path = path.as_ref().to_owned();
    asyncify(move || file::assert_exists(path)).await
}

#[cfg(test)]
mod tests {
    use {super::assert_exists, std::io};

    #[tokio::test]
    async fn smoke_assert_exists() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let file_path = dir.path().join("file.txt");

        // create a file in the tempdir
        std::fs::write(&file_path, b"hello")?;

        // should not error since the file exists
        assert_exists(&file_path).await?;

        Ok(())
    }
}
