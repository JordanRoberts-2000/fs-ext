use std::{fs, io, path::Path};

pub fn existing_dir_ok<F, T>(f: F) -> io::Result<()>
where
    F: FnOnce(&Path) -> io::Result<T>,
{
    let tmp = tempfile::tempdir()?;
    let dir = tmp.path().join("existing_dir");
    fs::create_dir(&dir)?;

    f(&dir).map_err(|e| {
        io::Error::new(
            e.kind(),
            format!("expected success when passed EXISTING directory '{}': {}", dir.display(), e),
        )
    })?;

    Ok(())
}
