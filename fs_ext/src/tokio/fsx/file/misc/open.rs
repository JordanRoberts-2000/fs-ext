use {
    std::{io, path::Path},
    tokio::fs::{File, OpenOptions},
};

pub async fn open(path: impl AsRef<Path>) -> io::Result<File> {
    OpenOptions::new().read(true).write(true).open(path).await
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        std::io::SeekFrom,
        tempfile::tempdir,
        tokio::{
            fs,
            io::{AsyncSeekExt, AsyncWriteExt},
        },
    };

    #[tokio::test]
    async fn opens_existing_file_read_write_and_can_modify() -> io::Result<()> {
        let dir = tempdir()?;
        let path = dir.path().join("data.txt");

        fs::write(&path, "hello").await?;

        let mut f = open(&path).await?;
        f.seek(SeekFrom::End(0)).await?;
        f.write_all(b"!").await?;
        f.flush().await?;
        drop(f);

        let s = fs::read_to_string(&path).await?;
        assert_eq!(s, "hello!");
        Ok(())
    }

    #[tokio::test]
    async fn does_not_create_missing_file() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("missing.txt");

        let err = open(&path).await.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::NotFound);
        assert!(!path.exists(), "file should not be created");
    }

    #[tokio::test]
    async fn error_when_target_is_directory() {
        let dir = tempdir().unwrap();
        let target = dir.path();

        let err = open(target).await.unwrap_err();
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
    async fn error_when_file_is_readonly() -> io::Result<()> {
        let dir = tempdir()?;
        let path = dir.path().join("ro.txt");

        fs::write(&path, "locked").await?;

        let mut perms = std::fs::metadata(&path)?.permissions();
        perms.set_readonly(true);
        std::fs::set_permissions(&path, perms)?;

        let res = open(&path).await;
        assert!(res.is_err(), "open should fail for read-only file with write(true)");

        if let Err(e) = res {
            assert!(
                matches!(e.kind(), io::ErrorKind::PermissionDenied | io::ErrorKind::Other),
                "unexpected error kind: {:?}",
                e.kind()
            );
        }
        Ok(())
    }
}
