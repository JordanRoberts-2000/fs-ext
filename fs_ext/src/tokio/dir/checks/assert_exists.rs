use {
    crate::{dir, tokio::utils::asyncify},
    std::{io, path::Path},
};

pub async fn assert_exists(path: impl AsRef<Path>) -> io::Result<()> {
    let path = path.as_ref().to_owned();
    asyncify(move || dir::assert_exists(path)).await
}

#[cfg(test)]
mod tests {
    use {super::assert_exists, std::io};

    #[tokio::test]
    async fn smoke_assert_exists() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let path = dir.path();

        assert_exists(path).await?;

        Ok(())
    }
}
