use {
    crate::{fsx, tokio::utils::asyncify},
    std::{io, path::Path},
};

pub async fn exists(path: impl AsRef<Path>) -> io::Result<bool> {
    let path = path.as_ref().to_owned();
    asyncify(move || fsx::dir::exists(path)).await
}

#[cfg(test)]
mod tests {
    use {super::exists, std::io};

    #[tokio::test]
    async fn smoke_exists() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let path = dir.path();

        let result = exists(path).await?;
        assert!(result);

        Ok(())
    }
}
