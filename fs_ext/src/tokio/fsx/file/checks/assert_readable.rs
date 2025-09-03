use {
    crate::{fsx, tokio::utils::asyncify},
    std::{io, path::Path},
};

pub async fn assert_readable(path: impl AsRef<Path>) -> io::Result<()> {
    let path = path.as_ref().to_owned();
    asyncify(move || fsx::file::assert_readable(path)).await
}

#[cfg(test)]
mod tests {
    use {super::assert_readable, std::io};

    #[tokio::test]
    async fn smoke_assert_readable() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let file_path = dir.path().join("file.txt");

        // create a readable file
        std::fs::write(&file_path, b"hello")?;

        // should succeed since the file is readable
        assert_readable(&file_path).await?;

        Ok(())
    }
}
