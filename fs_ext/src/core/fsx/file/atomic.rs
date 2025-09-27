use {
    crate::fsx,
    std::{fs::File, io, path::Path},
};

pub fn create_new<F, T, E>(path: impl AsRef<Path>, write_fn: F) -> io::Result<T>
where
    E: Into<io::Error>,
    F: FnOnce(&mut File) -> Result<T, E>,
{
    let path = path.as_ref();
    let parent = path.parent().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, format!("'{}' has no parent", path.display()))
    })?;

    let mut temp = fsx::file::temp_in(parent)?;

    let val = write_fn(temp.as_file_mut()).map_err(Into::into)?;

    temp.persist_new(path)?;
    Ok(val)
}

pub fn overwrite<F, T, E>(path: impl AsRef<Path>, write_fn: F) -> io::Result<T>
where
    E: Into<io::Error>,
    F: FnOnce(&mut File) -> Result<T, E>,
{
    let path = path.as_ref();
    let parent = path.parent().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, format!("'{}' has no parent", path.display()))
    })?;

    let mut temp = fsx::file::temp_in(parent)?;

    let val = write_fn(temp.as_file_mut()).map_err(Into::into)?;

    temp.persist(path)?;
    Ok(val)
}

pub fn update<F, T, E>(path: impl AsRef<Path>, write_fn: F) -> io::Result<T>
where
    E: Into<io::Error>,
    F: FnOnce(&mut File) -> Result<T, E>,
{
    let path = path.as_ref();
    let parent = path.parent().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, format!("'{}' has no parent", path.display()))
    })?;

    let mut temp = fsx::file::temp_in(parent)?;

    temp.copy_from(path)?;

    let val = write_fn(temp.as_file_mut()).map_err(Into::into)?;

    temp.persist(path)?;
    Ok(val)
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        std::{
            fs,
            io::{self, Seek, SeekFrom, Write},
        },
        tempfile::tempdir,
    };

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
