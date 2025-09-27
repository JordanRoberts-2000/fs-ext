use {
    crate::{file, tokio::utils::asyncify},
    std::{io, path::Path},
    tokio::fs::File,
};

pub async fn open(path: impl AsRef<Path>) -> io::Result<File> {
    let path = path.as_ref().to_owned();
    Ok(File::from_std(asyncify(move || file::open(path)).await?))
}

#[cfg(test)]
mod tests {
    use {super::open, std::io, tokio::io::AsyncReadExt};

    #[tokio::test]
    async fn smoke_open() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let file_path = dir.path().join("file.txt");

        // write some initial content
        std::fs::write(&file_path, b"hello")?;

        // open the file with your async wrapper
        let mut file = open(&file_path).await?;

        // read back the content
        let mut buf = String::new();
        file.read_to_string(&mut buf).await?;
        assert_eq!(buf, "hello");

        Ok(())
    }
}
