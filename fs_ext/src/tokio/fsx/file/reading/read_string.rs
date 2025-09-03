use {
    crate::{fsx, tokio::utils::asyncify},
    std::{io, path::Path},
};

pub async fn read_string(path: impl AsRef<Path>) -> io::Result<String> {
    let path = path.as_ref().to_owned();
    asyncify(move || fsx::file::read_string(path)).await
}

#[cfg(test)]
mod tests {
    use {super::read_string, std::io};

    #[tokio::test]
    async fn smoke_read_string() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let file_path = dir.path().join("file.txt");

        // write some text
        std::fs::write(&file_path, "hello world")?;

        // read it back
        let s = read_string(&file_path).await?;
        assert_eq!(s, "hello world");

        Ok(())
    }
}
