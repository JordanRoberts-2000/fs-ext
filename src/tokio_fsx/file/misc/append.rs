use {
    std::{io, path::Path},
    tokio::{fs::OpenOptions, io::AsyncWriteExt},
};

pub async fn append(path: impl AsRef<Path>, content: impl AsRef<[u8]>) -> io::Result<()> {
    let mut f = OpenOptions::new().append(true).open(path).await?;

    f.write_all(content.as_ref()).await?;
    f.flush().await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use {super::*, tempfile::tempdir, tokio::fs};

    #[tokio::test]
    async fn appends_to_existing_file_without_truncation() -> io::Result<()> {
        let dir = tempdir()?;
        let path = dir.path().join("log.txt");

        fs::write(&path, "one").await?;
        append(&path, "two").await?;
        append(&path, "three").await?;

        let s = fs::read_to_string(&path).await?;
        assert_eq!(s, "onetwothree");
        Ok(())
    }

    #[tokio::test]
    async fn empty_content_is_noop() -> io::Result<()> {
        let dir = tempdir()?;
        let path = dir.path().join("noop.txt");

        fs::write(&path, "keep-me").await?;
        append(&path, &[] as &[u8]).await?;

        let s = fs::read_to_string(&path).await?;
        assert_eq!(s, "keep-me");
        Ok(())
    }

    #[tokio::test]
    async fn does_not_create_missing_file() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("missing.txt");

        let err = append(&path, "data").await.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::NotFound);

        assert!(!path.exists());
    }

    #[tokio::test]
    async fn error_when_target_is_directory() {
        let dir = tempdir().unwrap();
        let target = dir.path();

        let err = append(target, "data").await.unwrap_err();
        assert!(
            matches!(
                err.kind(),
                io::ErrorKind::IsADirectory
                    | io::ErrorKind::PermissionDenied
                    | io::ErrorKind::Other
            ),
            "unexpected error kind: {:?}",
            err.kind()
        );
    }

    #[tokio::test]
    async fn appends_binary_bytes() -> io::Result<()> {
        let dir = tempdir()?;
        let path = dir.path().join("bytes.bin");

        fs::write(&path, [0xDE, 0xAD]).await?;
        append(&path, &[0xBE, 0xEF]).await?;

        let bytes = fs::read(&path).await?;
        assert_eq!(bytes, vec![0xDE, 0xAD, 0xBE, 0xEF]);
        Ok(())
    }
}
