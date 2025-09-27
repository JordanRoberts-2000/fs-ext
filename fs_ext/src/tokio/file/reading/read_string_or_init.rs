use {
    crate::{file, tokio::utils::asyncify},
    std::{io, path::Path},
};

pub async fn read_string_or_init(
    path: impl AsRef<Path>, contents: impl AsRef<[u8]> + Send + 'static,
) -> io::Result<String> {
    let path = path.as_ref().to_owned();
    asyncify(move || file::read_string_or_init(path, contents)).await
}

#[cfg(test)]
mod tests {
    use {super::read_string_or_init, std::io};

    #[tokio::test]
    async fn smoke_read_string_or_init() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let file_path = dir.path().join("file.txt");

        // first call should initialize the file with "hello"
        let s1 = read_string_or_init(&file_path, b"hello").await?;
        assert_eq!(s1, "hello");

        // second call should just read back the existing contents
        let s2 = read_string_or_init(&file_path, b"world").await?;
        assert_eq!(s2, "hello"); // should not overwrite

        Ok(())
    }
}
