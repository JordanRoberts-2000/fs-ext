use {
    crate::{file, tokio::utils::asyncify},
    std::{io, path::Path},
    tokio::fs::File,
};

pub async fn overwrite(path: impl AsRef<Path>) -> io::Result<File> {
    let path = path.as_ref().to_owned();
    Ok(File::from_std(asyncify(move || file::overwrite(path)).await?))
}

#[cfg(test)]
mod tests {
    use {super::overwrite, std::io, tokio::io::AsyncWriteExt};

    #[tokio::test]
    async fn smoke_overwrite() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let file_path = dir.path().join("file.txt");

        // write initial content
        std::fs::write(&file_path, b"hello")?;

        // open with overwrite, should truncate
        let mut file = overwrite(&file_path).await?;
        file.write_all(b"world").await?;
        file.flush().await?;
        drop(file);

        // file should now contain only "world"
        let content = std::fs::read_to_string(&file_path)?;
        assert_eq!(content, "world");

        Ok(())
    }
}
