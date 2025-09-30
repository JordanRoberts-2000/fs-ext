use {
    crate::file,
    std::{error, fs::File, io, path::Path},
};

pub fn create_new<F, T, E>(path: impl AsRef<Path>, write_fn: F) -> io::Result<T>
where
    E: Into<Box<dyn error::Error + Send + Sync>>,
    F: FnOnce(&mut File) -> Result<T, E>,
{
    let path = path.as_ref();
    let parent = path.parent().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, format!("'{}' has no parent", path.display()))
    })?;

    let mut temp = file::temp_in(parent)?;

    let val =
        write_fn(temp.as_file_mut()).map_err(|e| io::Error::new(io::ErrorKind::Other, e.into()))?;

    temp.persist_new(path)?;
    Ok(val)
}

pub fn overwrite<F, T, E>(path: impl AsRef<Path>, write_fn: F) -> io::Result<T>
where
    E: Into<Box<dyn error::Error + Send + Sync>>,
    F: FnOnce(&mut File) -> Result<T, E>,
{
    let path = path.as_ref();
    let parent = path.parent().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, format!("'{}' has no parent", path.display()))
    })?;

    let mut temp = file::temp_in(parent)?;

    let val =
        write_fn(temp.as_file_mut()).map_err(|e| io::Error::new(io::ErrorKind::Other, e.into()))?;

    temp.persist(path)?;
    Ok(val)
}

pub fn update<F, T, E>(path: impl AsRef<Path>, write_fn: F) -> io::Result<T>
where
    E: Into<Box<dyn error::Error + Send + Sync>>,
    F: FnOnce(&mut File) -> Result<T, E>,
{
    let path = path.as_ref();
    let parent = path.parent().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, format!("'{}' has no parent", path.display()))
    })?;

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
        std::{
            fmt, fs,
            io::{self, Seek, SeekFrom, Write},
        },
        tempfile::tempdir,
    };

    #[derive(Debug)]
    struct MyCustomError(&'static str);

    impl fmt::Display for MyCustomError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "MyCustomError: {}", self.0)
        }
    }
    impl error::Error for MyCustomError {}

    #[test]
    fn create_new_propagates_custom_error_and_does_not_create_dst() -> io::Result<()> {
        let dir = tempdir()?;
        let dst = dir.path().join("will-not-exist.txt");

        let err = create_new(&dst, |_f| -> Result<(), MyCustomError> {
            Err(MyCustomError("write failed"))
        })
        .unwrap_err();

        assert_eq!(err.kind(), io::ErrorKind::Other);
        let inner_msg = err.get_ref().map(ToString::to_string).unwrap_or_default();
        assert!(inner_msg.contains("write failed"));

        assert!(!dst.exists());
        Ok(())
    }

    #[test]
    fn create_writes_new_file_and_fails_if_exists() -> io::Result<()> {
        let dir = tempdir()?;
        let dst = dir.path().join("fresh.txt");

        create_new(&dst, |f| -> io::Result<()> {
            f.write_all(b"hello, world")?;
            f.sync_all()?;
            Ok(())
        })?;
        assert_eq!(fs::read_to_string(&dst)?, "hello, world");

        let err = create_new(&dst, |_f| -> io::Result<()> { Ok(()) }).unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::AlreadyExists);

        Ok(())
    }

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
    fn update_loads_then_applies_closure_and_persists() -> io::Result<()> {
        let dir = tempdir()?;
        let dst = dir.path().join("notes.txt");

        fs::write(&dst, b"base")?;

        update(&dst, |f| -> io::Result<()> {
            f.seek(SeekFrom::End(0))?;
            f.write_all(b"+more")?;
            Ok(())
        })?;

        assert_eq!(fs::read(&dst)?, b"base+more");
        Ok(())
    }
}
