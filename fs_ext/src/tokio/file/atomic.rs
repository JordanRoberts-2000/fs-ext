use {
    crate::{
        CollisionStrategy, WriteOptions,
        tokio::file::{self, atomic},
    },
    std::{future::Future, io, path::Path},
    tokio::{fs::File, task},
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

    let temp = file::temp_in(parent).await?;

    write_fn(temp.as_file()?).await?;

    temp.persist_new(path).await?;
    Ok(())
}

pub async fn create_with<F, Fut>(
    path: impl AsRef<Path>, write_fn: F, options: impl AsRef<WriteOptions>,
) -> io::Result<Option<()>>
where
    F: Fn(File) -> Fut + Send + 'static,
    Fut: Future<Output = io::Result<()>> + Send,
{
    let path = path.as_ref().to_path_buf();
    let options = options.as_ref().clone();

    task::spawn_blocking({
        let path = path.clone();
        let options = options.clone();
        move || -> io::Result<()> {
            if let Some(parent) = path.parent() {
                options.parent.ensure(parent)?;
            }
            Ok(())
        }
    })
    .await
    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))??;

    match &options.collision {
        CollisionStrategy::Error => atomic::create_new(&path, &write_fn).await.map(Some),
        CollisionStrategy::Skip => match atomic::create_new(&path, &write_fn).await {
            Ok(()) => Ok(Some(())),
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => Ok(None),
            Err(e) => Err(e),
        },
        CollisionStrategy::Overwrite => atomic::overwrite(&path, &write_fn).await.map(Some),
        CollisionStrategy::Rename(rename_opts) => {
            let rename_opts = rename_opts.clone(); // Assumes RenameOptions implements Clone
            match atomic::create_new(&path, &write_fn).await {
                Ok(()) => Ok(Some(())),
                Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {
                    let unique_path = task::spawn_blocking({
                        let path = path.clone();
                        move || rename_opts.generate_unique_path(&path)
                    })
                    .await
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))??;

                    atomic::create_new(&unique_path, &write_fn).await.map(Some)
                }
                Err(e) => Err(e),
            }
        }
    }
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

    let temp = file::temp_in(parent).await?;

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

    let mut temp = file::temp_in(parent).await?;

    temp.copy_from(path).await?;

    update_fn(temp.as_file()?).await?;

    temp.persist(path).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{ParentPolicy, RenameOptions},
        std::{
            fs,
            io::{self, SeekFrom},
        },
        tempfile::{TempDir, tempdir},
        tokio::io::{AsyncSeekExt, AsyncWriteExt},
    };

    fn setup_temp_dir() -> TempDir {
        tempfile::tempdir().unwrap()
    }

    #[tokio::test]
    async fn test_creates_new_file() {
        let temp_dir = setup_temp_dir();
        let path = temp_dir.path().join("test.txt");

        let options = WriteOptions {
            parent: ParentPolicy::RequireExists,
            collision: CollisionStrategy::Error,
        };

        let result = create_with(
            &path,
            |mut f| async move {
                f.write_all(b"hello").await?;
                Ok(())
            },
            &options,
        )
        .await;

        assert!(result.is_ok());
        assert!(path.exists());
    }

    #[tokio::test]
    async fn test_error_strategy_fails_on_existing() {
        let temp_dir = setup_temp_dir();
        let path = temp_dir.path().join("test.txt");

        fs::write(&path, "existing").unwrap();

        let options = WriteOptions {
            parent: ParentPolicy::RequireExists,
            collision: CollisionStrategy::Error,
        };

        let result = create_with(
            &path,
            |mut f| async move {
                f.write_all(b"new").await?;
                Ok(())
            },
            &options,
        )
        .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_skip_strategy_returns_none() {
        let temp_dir = setup_temp_dir();
        let path = temp_dir.path().join("test.txt");

        fs::write(&path, "existing").unwrap();

        let options = WriteOptions {
            parent: ParentPolicy::RequireExists,
            collision: CollisionStrategy::Skip,
        };

        let result = create_with(
            &path,
            |mut f| async move {
                f.write_all(b"new").await?;
                Ok(())
            },
            &options,
        )
        .await
        .unwrap();

        assert_eq!(result, None);
        assert_eq!(fs::read_to_string(&path).unwrap(), "existing");
    }

    #[tokio::test]
    async fn test_overwrite_strategy_replaces_file() {
        let temp_dir = setup_temp_dir();
        let path = temp_dir.path().join("test.txt");

        fs::write(&path, "old content").unwrap();

        let options = WriteOptions {
            parent: ParentPolicy::RequireExists,
            collision: CollisionStrategy::Overwrite,
        };

        let result = create_with(
            &path,
            |mut f| async move {
                f.write_all(b"new").await?;
                Ok(())
            },
            &options,
        )
        .await
        .unwrap();

        assert_eq!(result, Some(()));
        assert_eq!(fs::read_to_string(&path).unwrap(), "new");
    }

    #[tokio::test]
    async fn test_rename_counter_creates_renamed_file() {
        let temp_dir = setup_temp_dir();
        let path = temp_dir.path().join("test.txt");

        fs::write(&path, "original").unwrap();

        let options = WriteOptions {
            parent: ParentPolicy::RequireExists,
            collision: CollisionStrategy::Rename(RenameOptions::Counter),
        };

        let result = create_with(
            &path,
            |mut f| async move {
                f.write_all(b"renamed").await?;
                Ok(())
            },
            &options,
        )
        .await
        .unwrap();

        assert_eq!(result, Some(()));

        let renamed = temp_dir.path().join("test_1.txt");
        assert!(renamed.exists());
        assert_eq!(fs::read_to_string(&renamed).unwrap(), "renamed");
    }

    #[tokio::test]
    async fn test_create_missing_parent() {
        let temp_dir = setup_temp_dir();
        let path = temp_dir.path().join("new_dir").join("test.txt");

        let options = WriteOptions {
            parent: ParentPolicy::CreateIfMissing,
            collision: CollisionStrategy::Error,
        };

        let result = create_with(
            &path,
            |mut f| async move {
                f.write_all(b"content").await?;
                Ok(())
            },
            &options,
        )
        .await
        .unwrap();

        assert_eq!(result, Some(()));
        assert!(path.exists());
        assert!(path.parent().unwrap().exists());
    }

    #[tokio::test]
    async fn test_require_parent_fails_when_missing() {
        let temp_dir = setup_temp_dir();
        let path = temp_dir.path().join("nonexistent").join("test.txt");

        let options = WriteOptions {
            parent: ParentPolicy::RequireExists,
            collision: CollisionStrategy::Error,
        };

        let result = create_with(
            &path,
            |mut f| async move {
                f.write_all(b"content").await?;
                Ok(())
            },
            &options,
        )
        .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_rename_timestamp_creates_unique_file() {
        let temp_dir = setup_temp_dir();
        let path = temp_dir.path().join("test.txt");

        fs::write(&path, "original").unwrap();

        let options = WriteOptions {
            parent: ParentPolicy::RequireExists,
            collision: CollisionStrategy::Rename(RenameOptions::Timestamp),
        };

        let result = create_with(
            &path,
            |mut f| async move {
                f.write_all(b"timestamped").await?;
                Ok(())
            },
            &options,
        )
        .await
        .unwrap();

        assert_eq!(result, Some(()));

        // Should have 2 files: original + timestamped
        let count = fs::read_dir(temp_dir.path()).unwrap().count();
        assert_eq!(count, 2);
    }

    #[tokio::test]
    async fn test_write_function_called() {
        let temp_dir = setup_temp_dir();
        let path = temp_dir.path().join("test.txt");

        let options = WriteOptions {
            parent: ParentPolicy::RequireExists,
            collision: CollisionStrategy::Error,
        };

        create_with(
            &path,
            |mut f| async move {
                f.write_all(b"line 1\n").await?;
                f.write_all(b"line 2\n").await?;
                Ok(())
            },
            &options,
        )
        .await
        .unwrap();

        let content = fs::read_to_string(&path).unwrap();
        assert_eq!(content, "line 1\nline 2\n");
    }

    #[tokio::test]
    async fn test_atomicity_on_write_failure() {
        let temp_dir = setup_temp_dir();
        let path = temp_dir.path().join("test.txt");

        fs::write(&path, "original").unwrap();

        let options = WriteOptions {
            parent: ParentPolicy::RequireExists,
            collision: CollisionStrategy::Overwrite,
        };

        let result = create_with(
            &path,
            |mut f| async move {
                f.write_all(b"partial").await?;
                Err(std::io::Error::new(std::io::ErrorKind::Other, "simulated error"))
            },
            &options,
        )
        .await;

        assert!(result.is_err());

        // Original content should be preserved
        let content = fs::read_to_string(&path).unwrap();
        assert_eq!(content, "original");
    }

    #[tokio::test]
    async fn create_writes_new_file_and_fails_if_exists() -> io::Result<()> {
        let dir = tempdir()?;
        let dst = dir.path().join("fresh.txt");

        create_new(&dst, |mut f| async move {
            f.write_all(b"hello, world").await?;
            Ok(())
        })
        .await?;
        assert_eq!(fs::read_to_string(&dst)?, "hello, world");

        let err = create_new(&dst, |_f| async { Ok(()) }).await.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::AlreadyExists);

        Ok(())
    }

    #[tokio::test]
    async fn overwrite_replaces_existing_content() -> io::Result<()> {
        let dir = tempdir()?;
        let dst = dir.path().join("data.bin");

        fs::write(&dst, b"old content that is longer")?;

        overwrite(&dst, |mut f| async move {
            f.write_all(b"new").await?;
            Ok(())
        })
        .await?;

        let bytes = fs::read(&dst)?;
        assert_eq!(bytes.as_slice(), b"new");

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn update_loads_then_applies_closure_and_persists() -> io::Result<()> {
        let dir = tempdir()?;
        let dst = dir.path().join("notes.txt");

        fs::write(&dst, b"base")?;

        update(&dst, |mut f| async move {
            f.seek(SeekFrom::End(0)).await?;
            f.write_all(b"+more").await?;
            Ok(())
        })
        .await?;

        assert_eq!(fs::read(&dst)?, b"base+more");
        Ok(())
    }
}
