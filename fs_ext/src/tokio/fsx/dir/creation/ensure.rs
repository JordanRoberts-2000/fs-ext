use {
    crate::{fsx, tokio::utils::asyncify},
    std::{io, path::Path},
};

pub async fn ensure(path: impl AsRef<Path>) -> io::Result<()> {
    let path = path.as_ref().to_owned();
    asyncify(move || fsx::dir::ensure(path)).await
}

#[cfg(test)]
mod tests {
    use {super::ensure, std::io};

    #[tokio::test]
    async fn smoke_ensure() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let new_dir = dir.path().join("nested").join("subdir");

        ensure(&new_dir).await?;

        assert!(new_dir.exists());

        Ok(())
    }
}
