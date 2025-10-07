use {
    crate::{PathExt, file},
    std::{error, fs::File, io, path::Path},
};

pub fn overwrite<F, T, E>(path: impl AsRef<Path>, write_fn: F) -> io::Result<T>
where
    E: Into<Box<dyn error::Error + Send + Sync>>,
    F: FnOnce(&mut File) -> Result<T, E>,
{
    let path = path.as_ref();
    let parent = path.parent_or_current();

    let mut temp = file::temp_in(parent)?;

    let val =
        write_fn(temp.as_file_mut()).map_err(|e| io::Error::new(io::ErrorKind::Other, e.into()))?;

    temp.persist(path)?;
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
    fn overwrite_replaces_existing_content() -> io::Result<()> {
        let dir = tempdir()?;
        let dst = dir.path().join("data.bin");

        fs::write(&dst, b"old content that is longer")?;

        overwrite(&dst, |f| -> io::Result<()> {
            f.write_all(b"new")?;
            Ok(())
        })?;

        let bytes = fs::read(&dst)?;
        assert_eq!(bytes.as_slice(), b"new");
        Ok(())
    }

    #[test]
    fn overwrite_creates_file_if_not_exists() -> io::Result<()> {
        let dir = tempdir()?;
        let dst = dir.path().join("new_file.txt");

        assert!(!dst.exists());

        overwrite(&dst, |f| -> io::Result<()> {
            f.write_all(b"created from scratch")?;
            f.sync_all()?;
            Ok(())
        })?;

        assert!(dst.exists());
        assert_eq!(fs::read_to_string(&dst)?, "created from scratch");
        Ok(())
    }

    #[test]
    fn overwrite_returns_value_from_write_fn() -> io::Result<()> {
        let dir = tempdir()?;
        let dst = dir.path().join("return_test.txt");

        let result = overwrite(&dst, |f| -> io::Result<usize> {
            let bytes = b"test data";
            f.write_all(bytes)?;
            Ok(bytes.len())
        })?;

        assert_eq!(result, 9);
        assert_eq!(fs::read_to_string(&dst)?, "test data");
        Ok(())
    }

    #[test]
    fn overwrite_converts_custom_error_to_io_error() -> io::Result<()> {
        let dir = tempdir()?;
        let dst = dir.path().join("error_test.txt");

        let err = overwrite(&dst, |_f| -> Result<(), CustomError> {
            Err(CustomError("simulated failure"))
        })
        .unwrap_err();

        assert_eq!(err.kind(), io::ErrorKind::Other);
        assert!(err.to_string().contains("simulated failure"));
        assert!(!dst.exists());
        Ok(())
    }

    #[test]
    fn overwrite_preserves_original_on_write_error() -> io::Result<()> {
        let dir = tempdir()?;
        let dst = dir.path().join("preserve_test.txt");

        fs::write(&dst, b"original content")?;

        let _err = overwrite(&dst, |_f| -> io::Result<()> {
            Err(io::Error::new(io::ErrorKind::Other, "write failed"))
        })
        .unwrap_err();

        // Original file should still exist with original content
        assert_eq!(fs::read_to_string(&dst)?, "original content");
        Ok(())
    }

    #[test]
    fn overwrite_errors_when_parent_is_missing() {
        let dir = tempdir().unwrap();
        let missing_parent = dir.path().join("missing").join("file.txt");

        let err = overwrite(&missing_parent, |_f| -> io::Result<()> { Ok(()) }).unwrap_err();

        assert!(
            matches!(err.kind(), io::ErrorKind::NotFound | io::ErrorKind::Other),
            "expected NotFound/Other when parent is missing, got: {:?}",
            err
        );
        assert!(!missing_parent.exists());
        assert_eq!(file_count(dir.path()), 0);
    }

    #[test]
    fn overwrite_works_with_path_with_no_parent() -> io::Result<()> {
        let dir = tempdir()?;

        let original_dir = std::env::current_dir()?;
        std::env::set_current_dir(dir.path())?;

        let result = overwrite("simple.txt", |f| -> io::Result<()> {
            f.write_all(b"no parent path")?;
            f.sync_all()?;
            Ok(())
        });

        std::env::set_current_dir(original_dir)?;

        result?;
        assert_eq!(fs::read_to_string(dir.path().join("simple.txt"))?, "no parent path");
        Ok(())
    }

    #[test]
    fn overwrite_handles_empty_file() -> io::Result<()> {
        let dir = tempdir()?;
        let dst = dir.path().join("empty.txt");

        fs::write(&dst, b"some content")?;

        overwrite(&dst, |f| -> io::Result<()> {
            // Write nothing, creating an empty file
            f.sync_all()?;
            Ok(())
        })?;

        let bytes = fs::read(&dst)?;
        assert_eq!(bytes.len(), 0);
        Ok(())
    }
}
