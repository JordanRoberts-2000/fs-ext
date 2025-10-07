use {
    crate::{PathExt, file},
    std::{error, fs::File, io, path::Path},
};

pub fn create_new<F, T, E>(path: impl AsRef<Path>, write_fn: F) -> io::Result<T>
where
    E: Into<Box<dyn error::Error + Send + Sync>>,
    F: FnOnce(&mut File) -> Result<T, E>,
{
    let path = path.as_ref();
    let parent = path.parent_or_current();

    let mut temp = file::temp_in(parent)?;

    let val =
        write_fn(temp.as_file_mut()).map_err(|e| io::Error::new(io::ErrorKind::Other, e.into()))?;

    temp.persist_new(path)?;
    Ok(val)
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::test_utils::{CustomError, file_count},
        std::{
            fs,
            io::{self, Write},
        },
        tempfile::tempdir,
    };

    #[test]
    fn create_new_writes_new_file() -> io::Result<()> {
        let dir = tempdir()?;
        let dst = dir.path().join("fresh.txt");

        create_new(&dst, |f| -> io::Result<()> {
            f.write_all(b"hello, world")?;
            f.sync_all()?;
            Ok(())
        })?;

        assert_eq!(fs::read_to_string(&dst)?, "hello, world");
        Ok(())
    }

    #[test]
    fn create_new_fails_if_file_already_exists() -> io::Result<()> {
        let dir = tempdir()?;
        let dst = dir.path().join("existing.txt");

        create_new(&dst, |f| -> io::Result<()> {
            f.write_all(b"first write")?;
            f.sync_all()?;
            Ok(())
        })?;

        let err = create_new(&dst, |_f| -> io::Result<()> { Ok(()) }).unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::AlreadyExists);

        // Original content should be preserved
        assert_eq!(fs::read_to_string(&dst)?, "first write");
        Ok(())
    }

    #[test]
    fn create_new_write_fn_converts_to_io_error() -> io::Result<()> {
        let dir = tempdir()?;
        let dst = dir.path().join("will-not-exist.txt");

        let err = create_new(&dst, |_f| -> Result<(), CustomError> {
            Err(CustomError("simulated failure"))
        })
        .unwrap_err();

        assert_eq!(err.kind(), io::ErrorKind::Other);
        assert!(err.to_string().contains("simulated failure"));
        assert!(!dst.exists());
        Ok(())
    }

    #[test]
    fn create_new_returns_value_from_write_fn() -> io::Result<()> {
        let dir = tempdir()?;
        let dst = dir.path().join("return_test.txt");

        let result = create_new(&dst, |f| -> io::Result<usize> {
            let bytes = b"test data";
            f.write_all(bytes)?;
            Ok(bytes.len())
        })?;

        assert_eq!(result, 9);
        assert_eq!(fs::read_to_string(&dst)?, "test data");
        Ok(())
    }

    #[test]
    fn create_new_errors_when_parent_is_missing() {
        let dir = tempdir().unwrap();
        let missing_parent = dir.path().join("missing").join("file.txt");

        let err = create_new(&missing_parent, |_f| -> io::Result<()> { Ok(()) }).unwrap_err();

        assert!(
            matches!(err.kind(), io::ErrorKind::NotFound | io::ErrorKind::Other),
            "expected NotFound/Other when parent is missing, got: {:?}",
            err
        );
        assert!(!missing_parent.exists());
        assert_eq!(file_count(dir.path()), 0);
    }

    #[test]
    fn create_new_works_with_path_with_no_parent() -> io::Result<()> {
        let dir = tempdir()?;

        let original_dir = std::env::current_dir()?;
        std::env::set_current_dir(dir.path())?;

        let result = create_new("simple.txt", |f| -> io::Result<()> {
            f.write_all(b"no parent path")?;
            f.sync_all()?;
            Ok(())
        });

        std::env::set_current_dir(original_dir)?;

        result?;
        assert_eq!(fs::read_to_string(dir.path().join("simple.txt"))?, "no parent path");
        Ok(())
    }
}
