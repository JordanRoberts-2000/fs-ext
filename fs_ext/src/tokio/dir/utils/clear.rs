use {
    crate::{dir, tokio::utils::asyncify},
    std::{io, path::Path},
};

pub async fn clear(path: impl AsRef<Path>) -> io::Result<()> {
    let path = path.as_ref().to_owned();
    asyncify(move || dir::clear(path)).await
}

#[cfg(test)]
mod tests {
    use {super::clear, std::io};

    #[tokio::test]
    async fn smoke_clear() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let file_path = dir.path().join("file.txt");

        // put something in the directory
        std::fs::write(&file_path, b"hello")?;
        assert!(file_path.exists());

        // clear the directory
        clear(dir.path()).await?;

        // directory should now be empty
        assert!(std::fs::read_dir(dir.path())?.next().is_none());

        Ok(())
    }
}
