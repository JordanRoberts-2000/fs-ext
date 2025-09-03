use {
    crate::{fsx, tokio::utils::asyncify},
    std::{io, path::Path},
    tokio::fs::File,
};

pub async fn ensure(path: impl AsRef<Path>) -> io::Result<File> {
    let path = path.as_ref().to_owned();
    Ok(File::from_std(asyncify(move || fsx::file::ensure(path)).await?))
}

#[cfg(test)]
mod tests {
    use {super::ensure, std::io};

    #[tokio::test]
    async fn smoke_ensure() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let file_path = dir.path().join("file.txt");

        // should succeed even though the file doesn't exist yet
        let file = ensure(&file_path).await?;
        drop(file); // close handle

        // confirm file exists
        assert!(file_path.exists());

        Ok(())
    }
}
