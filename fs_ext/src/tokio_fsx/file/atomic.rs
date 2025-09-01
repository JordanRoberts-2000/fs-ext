use {
    crate::tokio::fsx,
    std::{future::Future, io, path::Path},
    tokio::fs::File,
};

pub async fn create_new<F, Fut>(path: impl AsRef<Path>, write_fn: F) -> io::Result<()>
where
    F: FnOnce(File) -> Fut,
    Fut: Future<Output = io::Result<()>>,
{
    let path = path.as_ref();
    let parent = path.parent().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, format!("'{}' has no parent", path.display()))
    })?;

    let temp = fsx::file::temp_in(parent).await?;

    write_fn(temp.as_file()?).await?;

    temp.persist_new(path).await?;
    Ok(())
}

pub async fn overwrite<F, Fut>(path: impl AsRef<Path>, write_fn: F) -> io::Result<()>
where
    F: FnOnce(File) -> Fut,
    Fut: Future<Output = io::Result<()>>,
{
    let path = path.as_ref();
    let parent = path.parent().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, format!("'{}' has no parent", path.display()))
    })?;

    let temp = fsx::file::temp_in(parent).await?;

    write_fn(temp.as_file()?).await?;

    temp.persist(path).await?;
    Ok(())
}

pub async fn update<F, Fut>(path: impl AsRef<Path>, update_fn: F) -> io::Result<()>
where
    F: FnOnce(File) -> Fut,
    Fut: Future<Output = io::Result<()>>,
{
    let path = path.as_ref();
    let parent = path.parent().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, format!("'{}' has no parent", path.display()))
    })?;

    let mut temp = fsx::file::temp_in(parent).await?;

    temp.copy_from(path).await?;

    update_fn(temp.as_file()?).await?;

    temp.persist(path).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        std::io::{self, SeekFrom},
        tempfile::tempdir,
        tokio::{
            fs,
            io::{AsyncSeekExt, AsyncWriteExt},
        },
    };

    #[tokio::test]
    async fn create_writes_new_file_and_fails_if_exists() -> io::Result<()> {
        let dir = tempdir()?;
        let dst = dir.path().join("fresh.txt");

        create_new(&dst, |mut f| async move {
            f.write_all(b"hello, world").await?;
            Ok(())
        })
        .await?;
        assert_eq!(fs::read_to_string(&dst).await?, "hello, world");

        let err = create_new(&dst, |_f| async { Ok(()) }).await.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::AlreadyExists);

        Ok(())
    }

    #[tokio::test]
    async fn overwrite_replaces_existing_content() -> io::Result<()> {
        let dir = tempdir()?;
        let dst = dir.path().join("data.bin");

        fs::write(&dst, b"old content that is longer").await?;

        overwrite(&dst, |mut f| async move {
            f.write_all(b"new").await?;
            Ok(())
        })
        .await?;

        let bytes = fs::read(&dst).await?;
        assert_eq!(bytes.as_slice(), b"new");

        Ok(())
    }

    #[tokio::test]
    async fn update_loads_then_applies_closure_and_persists() -> io::Result<()> {
        let dir = tempdir()?;
        let dst = dir.path().join("notes.txt");

        fs::write(&dst, b"base").await?;

        update(&dst, |mut f| async move {
            f.seek(SeekFrom::End(0)).await?;
            f.write_all(b"+more").await?;
            Ok(())
        })
        .await?;

        assert_eq!(fs::read(&dst).await?, b"base+more");
        Ok(())
    }
}
