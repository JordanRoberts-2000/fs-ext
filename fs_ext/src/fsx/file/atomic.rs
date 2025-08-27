use {
    crate::fsx,
    std::{fs::File, io, path::Path},
};

pub fn create_new<F>(path: impl AsRef<Path>, write_fn: F) -> io::Result<()>
where
    F: FnOnce(&mut File) -> io::Result<()>,
{
    let path = path.as_ref();
    let parent = path.parent().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, format!("'{}' has no parent", path.display()))
    })?;

    let mut temp = fsx::file::temp_in(parent)?;

    write_fn(temp.as_file_mut())?;

    temp.persist_new(path)?;
    Ok(())
}

pub fn overwrite<F>(path: impl AsRef<Path>, write_fn: F) -> io::Result<()>
where
    F: FnOnce(&mut File) -> io::Result<()>,
{
    let path = path.as_ref();
    let parent = path.parent().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, format!("'{}' has no parent", path.display()))
    })?;

    let mut temp = fsx::file::temp_in(parent)?;

    write_fn(temp.as_file_mut())?;

    temp.persist(path)?;
    Ok(())
}

pub fn update<F>(path: impl AsRef<Path>, update_fn: F) -> io::Result<()>
where
    F: FnOnce(&mut File) -> io::Result<()>,
{
    let path = path.as_ref();
    let parent = path.parent().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, format!("'{}' has no parent", path.display()))
    })?;

    let mut temp = fsx::file::temp_in(parent)?;

    temp.copy(path)?;

    update_fn(temp.as_file_mut())?;

    temp.persist(path)?;
    Ok(())
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

        create_new(&dst, |f| {
            f.write_all(b"hello, world")?;
            Ok(())
        })?;
        assert_eq!(fs::read_to_string(&dst)?, "hello, world");

        let err = create_new(&dst, |_f| Ok(())).unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::AlreadyExists);

        Ok(())
    }

    #[test]
    fn overwrite_replaces_existing_content() -> io::Result<()> {
        let dir = tempdir()?;
        let dst = dir.path().join("data.bin");

        fs::write(&dst, b"old content that is longer")?;

        overwrite(&dst, |f| {
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

        update(&dst, |f| {
            f.seek(SeekFrom::End(0))?;
            f.write_all(b"+more")?;
            Ok(())
        })?;

        assert_eq!(fs::read(&dst)?, b"base+more");
        Ok(())
    }
}
