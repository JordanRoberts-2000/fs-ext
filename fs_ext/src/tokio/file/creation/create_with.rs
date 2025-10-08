use {
    crate::{WriteOptions, file, tokio::utils::asyncify},
    std::{io, path::Path},
    tokio::fs::File,
};

pub async fn create_with(
    path: impl AsRef<Path>, options: impl AsRef<WriteOptions>,
) -> io::Result<Option<File>> {
    let path = path.as_ref().to_owned();
    let opts = options.as_ref().clone();

    let maybe_std = asyncify(move || file::create_with(&path, &opts)).await?;

    Ok(maybe_std.map(File::from_std))
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{CollisionStrategy, ParentPolicy},
        std::io,
        tempfile::tempdir,
    };

    #[tokio::test]
    async fn create_with_error_strategy_first_ok_then_already_exists() -> io::Result<()> {
        let dir = tempdir()?;
        let dst = dir.path().join("x.txt");

        let opts = WriteOptions {
            parent: ParentPolicy::CreateIfMissing,
            collision: CollisionStrategy::Error,
        };
        // first create: Some(file)
        let f1 = create_with(&dst, &opts).await?;
        assert!(f1.is_some());
        drop(f1);

        // second create: Err(AlreadyExists)
        let err = create_with(&dst, &opts).await.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::AlreadyExists);
        Ok(())
    }

    #[tokio::test]
    async fn create_with_skip_strategy_returns_none_if_exists() -> io::Result<()> {
        let dir = tempdir()?;
        let dst = dir.path().join("y.txt");

        let skip = WriteOptions {
            parent: ParentPolicy::CreateIfMissing,
            collision: CollisionStrategy::Skip,
        };
        // first create succeeds
        let f1 = create_with(&dst, &skip).await?;
        assert!(f1.is_some());
        drop(f1);

        // second create returns None (skipped)
        let f2 = create_with(&dst, &skip).await?;
        assert!(f2.is_none());

        Ok(())
    }
}
