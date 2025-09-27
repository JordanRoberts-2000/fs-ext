use {
    crate::{file, tokio::utils::asyncify},
    std::{fs::FileType, io, path::Path},
};

pub async fn file_type(path: impl AsRef<Path>) -> io::Result<FileType> {
    let path = path.as_ref().to_owned();
    asyncify(move || file::meta::file_type(path)).await
}

#[cfg(test)]
mod tests {
    use {super::file_type, std::io};

    #[tokio::test]
    async fn smoke_file_type() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let file_path = dir.path().join("file.txt");

        // create a file
        std::fs::write(&file_path, b"hello")?;

        // should return a file type of "file"
        let ft = file_type(&file_path).await?;
        assert!(ft.is_file());

        Ok(())
    }
}
