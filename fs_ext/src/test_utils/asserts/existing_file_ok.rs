use std::{fs, io, path::Path};

pub fn existing_file_ok<F, T>(f: F) -> io::Result<()>
where
    F: FnOnce(&Path) -> io::Result<T>,
{
    let tmp = tempfile::tempdir()?;
    let file = tmp.path().join("existing_file.bin");
    fs::write(&file, b"x")?;

    f(&file).map_err(|e| {
        io::Error::new(
            e.kind(),
            format!("expected success when passed EXISTING file '{}': {}", file.display(), e),
        )
    })?;

    Ok(())
}
