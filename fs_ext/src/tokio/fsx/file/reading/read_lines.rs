use {
    crate::{fsx, tokio::utils::asyncify},
    std::{io, path::Path},
};

pub async fn read_lines(path: impl AsRef<Path>) -> io::Result<Vec<String>> {
    let path = path.as_ref().to_owned();
    asyncify(move || fsx::file::read_lines(path)).await
}

#[cfg(test)]
mod tests {
    use {super::read_lines, std::io};

    #[tokio::test]
    async fn smoke_read_lines() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let file_path = dir.path().join("file.txt");

        // write multiple lines
        std::fs::write(&file_path, "hello\nworld\n")?;

        // read them back
        let lines = read_lines(&file_path).await?;
        assert_eq!(lines, vec!["hello", "world"]);

        Ok(())
    }
}
