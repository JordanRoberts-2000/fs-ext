use {
    crate::{fsx, tokio::utils::asyncify},
    std::{io, path::Path},
};

pub async fn read_string_or_init_with<F, C>(
    path: impl AsRef<Path>, contents_fn: F,
) -> io::Result<String>
where
    F: FnOnce() -> C + Send + 'static,
    C: AsRef<[u8]>,
{
    let path = path.as_ref().to_owned();
    asyncify(move || fsx::file::read_string_or_init_with(path, contents_fn)).await
}

#[cfg(test)]
mod tests {
    use {super::read_string_or_init_with, std::io};

    #[tokio::test]
    async fn smoke_read_string_or_init_with() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let file_path = dir.path().join("file.txt");

        // first call should initialize the file with the closure's value
        let s1 = read_string_or_init_with(&file_path, || "hello").await?;
        assert_eq!(s1, "hello");

        // second call should return the existing content, not call closure again
        let s2 = read_string_or_init_with(&file_path, || "world").await?;
        assert_eq!(s2, "hello");

        Ok(())
    }
}
