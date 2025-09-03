use {
    crate::{fsx, tokio::utils::asyncify},
    std::{io, path::Path},
};

pub async fn assert_not_exists(path: impl AsRef<Path>) -> io::Result<()> {
    let path = path.as_ref().to_owned();
    asyncify(move || fsx::file::assert_not_exists(path)).await
}

#[cfg(test)]
mod tests {
    use {super::assert_not_exists, std::io};

    #[tokio::test]
    async fn smoke_assert_not_exists() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let missing_file = dir.path().join("nope.txt");

        // should succeed since file does not exist
        assert_not_exists(&missing_file).await?;

        Ok(())
    }
}
