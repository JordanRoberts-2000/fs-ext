use {
    crate::{dir, tokio::utils::asyncify},
    std::{io, path::Path},
};

pub async fn is_empty(path: impl AsRef<Path>) -> io::Result<bool> {
    let path = path.as_ref().to_owned();
    asyncify(move || dir::is_empty(path)).await
}

#[cfg(test)]
mod tests {
    use {super::is_empty, std::io};

    #[tokio::test]
    async fn smoke_is_empty() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let path = dir.path();

        let result = is_empty(path).await?;
        assert!(result);

        Ok(())
    }
}
