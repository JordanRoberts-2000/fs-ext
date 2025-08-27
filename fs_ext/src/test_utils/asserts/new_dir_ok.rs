use std::{io, path::Path};

pub fn new_dir_ok<F, T>(f: F) -> io::Result<()>
where
    F: FnOnce(&Path) -> io::Result<T>,
{
    let tmp = tempfile::tempdir()?;
    let dir = tmp.path().join("new_dir_should_be_created");
    assert!(!dir.exists(), "test setup error: '{}' already exists", dir.display());

    f(&dir).map_err(|e| {
        io::Error::new(
            e.kind(),
            format!("expected success when passing NEW directory path '{}': {}", dir.display(), e),
        )
    })?;

    Ok(())
}
