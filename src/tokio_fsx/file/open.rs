use {
    std::{io, path::Path},
    tokio::fs::{File, OpenOptions},
};

pub async fn write_only(path: impl AsRef<Path>) -> io::Result<File> {
    OpenOptions::new().write(true).open(path).await
}

pub async fn read_only(path: impl AsRef<Path>) -> io::Result<File> {
    OpenOptions::new().read(true).open(path).await
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        std::io,
        tempfile::tempdir,
        tokio::{
            fs,
            io::{AsyncReadExt, AsyncWriteExt},
        },
    };

    #[tokio::test]
    async fn read_only_errors_if_missing() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("missing.txt");

        let err = read_only(&path).await.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::NotFound);
    }

    #[tokio::test]
    async fn write_only_errors_if_missing() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("missing.txt");

        let err = write_only(&path).await.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::NotFound);
    }

    #[tokio::test]
    async fn read_only_can_read_but_cannot_write() -> io::Result<()> {
        let dir = tempdir()?;
        let path = dir.path().join("data.txt");
        fs::write(&path, b"hello").await?;

        let mut f = read_only(&path).await?;

        let mut s = String::new();
        f.read_to_string(&mut s).await?;
        assert_eq!(s, "hello");

        f.write_all(b"!").await?;

        assert!(f.flush().await.is_err(), "write unexpectedly succeeded");
        Ok(())
    }

    #[tokio::test]
    async fn write_only_can_write_but_cannot_read() -> io::Result<()> {
        let dir = tempdir()?;
        let path = dir.path().join("data.txt");
        fs::write(&path, b"old").await?;

        let mut f = write_only(&path).await?;
        f.write_all(b"new").await?;
        f.flush().await?;

        let mut s = String::new();
        assert!(f.read_to_string(&mut s).await.is_err(), "read unexpectedly succeeded");

        drop(f);

        let on_disk = fs::read_to_string(&path).await?;
        assert_eq!(on_disk, "new");

        Ok(())
    }
}
