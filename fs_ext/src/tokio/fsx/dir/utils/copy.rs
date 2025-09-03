use {
    crate::{fsx, tokio::utils::asyncify},
    std::{io, path::Path},
};

pub async fn copy(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    let src = src.as_ref().to_owned();
    let dst = dst.as_ref().to_owned();
    asyncify(move || fsx::dir::copy(src, dst)).await
}

#[cfg(test)]
mod tests {
    use {super::copy, std::io};

    #[tokio::test]
    async fn smoke_copy() -> io::Result<()> {
        let src = tempfile::tempdir()?;
        let dst = tempfile::tempdir()?;

        // put a file in the source dir
        let src_file = src.path().join("file.txt");
        std::fs::write(&src_file, b"hello")?;

        // copy whole directory
        copy(src.path(), dst.path()).await?;

        // confirm file exists in destination with same content
        let dst_file = dst.path().join("file.txt");
        assert!(dst_file.exists());
        let content = std::fs::read_to_string(&dst_file)?;
        assert_eq!(content, "hello");

        Ok(())
    }
}
