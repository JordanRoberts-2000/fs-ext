use std::{io, path::Path};

pub fn new_file_ok<F, T>(f: F) -> io::Result<()>
where
    F: FnOnce(&Path) -> io::Result<T>,
{
    let tmp = tempfile::tempdir()?;
    let file = tmp.path().join("new_file_should_be_created.bin");
    assert!(!file.exists(), "test setup error: '{}' already exists", file.display());

    f(&file).map_err(|e| {
        io::Error::new(
            e.kind(),
            format!("expected success when passing NEW file path '{}': {}", file.display(), e),
        )
    })?;

    Ok(())
}
