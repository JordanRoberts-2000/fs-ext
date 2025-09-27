use {
    crate::{file, tokio::utils::asyncify},
    std::{io, path::Path},
    tokio::fs::File,
};

pub async fn create_new(path: impl AsRef<Path>) -> io::Result<File> {
    let path = path.as_ref().to_owned();
    Ok(File::from_std(asyncify(move || file::create_new(path)).await?))
}

#[cfg(test)]
mod tests {
    use {super::create_new, std::io};

    #[tokio::test]
    async fn smoke_create_new() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let file_path = dir.path().join("file.txt");

        // should succeed creating a brand new file
        let file = create_new(&file_path).await?;
        drop(file); // close handle

        // confirm file exists
        assert!(file_path.exists());

        Ok(())
    }
}
