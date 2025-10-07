use {
    crate::{PathExt, file},
    std::{error, fs::File, io, path::Path},
};

pub fn update<F, T, E>(path: impl AsRef<Path>, write_fn: F) -> io::Result<T>
where
    E: Into<Box<dyn error::Error + Send + Sync>>,
    F: FnOnce(&mut File) -> Result<T, E>,
{
    let path = path.as_ref();
    let parent = path.parent_or_current();

    let mut temp = file::temp_in(parent)?;

    temp.copy_from(path)?;

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
            io::{self, Read, Seek, SeekFrom, Write},
        },
        tempfile::tempdir,
    };

    #[test]
    fn update_modifies_existing_content() -> io::Result<()> {
        let dir = tempdir()?;
        let dst = dir.path().join("data.txt");

        fs::write(&dst, b"hello")?;

        update(&dst, |f| -> io::Result<()> {
            f.seek(SeekFrom::End(0))?;
            f.write_all(b" world")?;
            Ok(())
        })?;

        assert_eq!(fs::read_to_string(&dst)?, "hello world");
        Ok(())
    }

    #[test]
    fn update_can_read_and_modify() -> io::Result<()> {
        let dir = tempdir()?;
        let dst = dir.path().join("counter.txt");

        fs::write(&dst, b"42")?;

        update(&dst, |f| -> io::Result<()> {
            f.seek(SeekFrom::Start(0))?;
            let mut contents = String::new();
            f.read_to_string(&mut contents)?;
            let num: i32 = contents.trim().parse().unwrap();

            f.seek(SeekFrom::Start(0))?;
            f.set_len(0)?;
            write!(f, "{}", num + 1)?;
            Ok(())
        })?;

        assert_eq!(fs::read_to_string(&dst)?, "43");
        Ok(())
    }

    #[test]
    fn update_fails_if_file_does_not_exist() {
        let dir = tempdir().unwrap();
        let dst = dir.path().join("nonexistent.txt");

        let err = update(&dst, |_f| -> io::Result<()> { Ok(()) }).unwrap_err();

        assert_eq!(err.kind(), io::ErrorKind::NotFound);
        assert!(!dst.exists());
    }

    #[test]
    fn update_returns_value_from_write_fn() -> io::Result<()> {
        let dir = tempdir()?;
        let dst = dir.path().join("return_test.txt");

        fs::write(&dst, b"test")?;

        let result = update(&dst, |f| -> io::Result<u64> {
            let len = f.metadata()?.len();
            f.seek(SeekFrom::End(0))?;
            f.write_all(b" data")?;
            Ok(len)
        })?;

        assert_eq!(result, 4);
        assert_eq!(fs::read_to_string(&dst)?, "test data");
        Ok(())
    }

    #[test]
    fn update_converts_custom_error_to_io_error() -> io::Result<()> {
        let dir = tempdir()?;
        let dst = dir.path().join("error_test.txt");

        fs::write(&dst, b"original")?;

        let err =
            update(&dst, |_f| -> Result<(), CustomError> { Err(CustomError("update failed")) })
                .unwrap_err();

        assert_eq!(err.kind(), io::ErrorKind::Other);
        assert!(err.to_string().contains("update failed"));
        assert_eq!(fs::read_to_string(&dst)?, "original");
        Ok(())
    }

    #[test]
    fn update_preserves_original_on_write_error() -> io::Result<()> {
        let dir = tempdir()?;
        let dst = dir.path().join("preserve_test.txt");

        fs::write(&dst, b"original content")?;

        let _err = update(&dst, |f| -> io::Result<()> {
            f.seek(SeekFrom::End(0))?;
            f.write_all(b" partial")?;
            Err(io::Error::new(io::ErrorKind::Other, "write failed"))
        })
        .unwrap_err();

        // Original file should still exist with original content (atomicity)
        assert_eq!(fs::read_to_string(&dst)?, "original content");
        Ok(())
    }

    #[test]
    fn update_errors_when_parent_is_missing() -> io::Result<()> {
        let dir = tempdir()?;
        let missing_parent = dir.path().join("missing").join("file.txt");

        let err = update(&missing_parent, |_f| -> io::Result<()> { Ok(()) }).unwrap_err();

        assert_eq!(err.kind(), io::ErrorKind::NotFound);
        assert!(!missing_parent.exists());
        assert_eq!(file_count(dir.path()), 0);
        Ok(())
    }

    #[test]
    fn update_works_with_path_with_no_parent() -> io::Result<()> {
        let dir = tempdir()?;

        let original_dir = std::env::current_dir()?;
        std::env::set_current_dir(dir.path())?;

        fs::write("simple.txt", b"before")?;

        let result = update("simple.txt", |f| -> io::Result<()> {
            f.seek(SeekFrom::End(0))?;
            f.write_all(b" after")?;
            Ok(())
        });

        std::env::set_current_dir(original_dir)?;

        result?;
        assert_eq!(fs::read_to_string(dir.path().join("simple.txt"))?, "before after");
        Ok(())
    }
}
