use {
    crate::{dir, tokio::utils::asyncify},
    std::{io, path::Path},
};

pub async fn create_new(path: impl AsRef<Path>) -> io::Result<()> {
    let path = path.as_ref().to_owned();
    asyncify(move || dir::create_new(path)).await
}

#[cfg(test)]
mod tests {
    use {super::create_new, std::io};

    #[tokio::test]
    async fn smoke_create_new() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let new_dir = dir.path().join("subdir");

        create_new(&new_dir).await?;

        assert!(new_dir.exists());

        Ok(())
    }
}
