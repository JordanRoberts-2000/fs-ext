use {
    crate::{fsx, tokio::utils::asyncify},
    std::{io, path::Path},
};

pub async fn size(path: impl AsRef<Path>) -> io::Result<u128> {
    let path = path.as_ref().to_owned();
    asyncify(move || fsx::dir::size(path)).await
}

#[cfg(test)]
mod tests {
    use {super::size, std::io};

    #[tokio::test]
    async fn smoke_size() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let path = dir.path();

        let result = size(path).await?;
        assert_eq!(result, 0);

        Ok(())
    }
}
