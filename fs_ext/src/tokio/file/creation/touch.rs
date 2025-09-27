use {
    crate::{file, tokio::utils::asyncify},
    std::{io, path::Path},
    tokio::fs::File,
};

pub async fn touch(path: impl AsRef<Path>) -> io::Result<File> {
    let path = path.as_ref().to_owned();
    Ok(File::from_std(asyncify(move || file::touch(path)).await?))
}

#[cfg(test)]
mod tests {
    use {super::touch, std::io};

    #[tokio::test]
    async fn smoke_touch() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let file_path = dir.path().join("file.txt");

        // should create the file if it doesn't exist
        let file = touch(&file_path).await?;
        drop(file);

        assert!(file_path.exists());

        // calling again should not error (idempotent)
        touch(&file_path).await?;

        Ok(())
    }
}
