use {
    crate::fsx,
    std::{io, path::Path},
    walkdir::WalkDir,
};

#[cfg_attr(test, fs_ext_test_macros::fs_test(rejects_file, rejects_missing_path, existing_dir_ok))]
pub fn size(path: impl AsRef<Path>) -> io::Result<u64> {
    _size(path.as_ref())
}

fn _size(path: &Path) -> io::Result<u64> {
    let mut total: u64 = 0;

    fsx::dir::assert_exists(path)?;

    for entry in WalkDir::new(path) {
        let entry = entry.map_err(|e| {
            let kind = e.io_error().map(|ioe| ioe.kind()).unwrap_or(io::ErrorKind::Other);
            let msg = match e.path() {
                Some(p) => format!("walk error at '{}': {}", p.display(), e),
                None => format!("walk error: {}", e),
            };
            io::Error::new(kind, msg)
        })?;

        if entry.file_type().is_file() {
            let len = entry.metadata().map(|m| m.len()).map_err(|meta_err| {
                io::Error::new(
                    meta_err.io_error().map(|ioe| ioe.kind()).unwrap_or(io::ErrorKind::Other),
                    format!(
                        "metadata error under '{}' at '{}': {}",
                        path.display(),
                        entry.path().display(),
                        meta_err
                    ),
                )
            })?;
            total += len;
        }
    }

    Ok(total)
}

#[cfg(test)]
mod tests {
    use {
        super::size,
        std::{fs, io},
        tempfile::tempdir,
    };

    #[test]
    fn empty_dir_is_zero() -> io::Result<()> {
        let d = tempdir()?;
        assert_eq!(size(d.path())?, 0);
        Ok(())
    }

    #[test]
    fn counts_nested_files() -> io::Result<()> {
        let d = tempdir()?;
        let sub = d.path().join("sub");
        fs::create_dir_all(&sub)?;

        let a = d.path().join("a.bin");
        let b = sub.join("b.txt");
        fs::write(&a, vec![0u8; 10])?;
        fs::write(&b, vec![0u8; 5])?;

        assert_eq!(size(d.path())?, 15);
        Ok(())
    }
}
