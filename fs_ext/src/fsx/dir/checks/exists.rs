use {
    crate::IoResultExt,
    std::{fs, io, path::Path},
};

#[cfg_attr(test, fs_ext_test_macros::fs_test(existing_dir_ok))]
pub fn exists(path: impl AsRef<Path>) -> io::Result<bool> {
    _exists(path.as_ref())
}

fn _exists(path: &Path) -> io::Result<bool> {
    match fs::metadata(path) {
        Ok(meta) => Ok(meta.is_dir()),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(false),
        Err(e) => Err(e).with_path_context("Failed to access", path),
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        std::{fs, io},
        tempfile::tempdir,
    };

    #[test]
    fn returns_false_for_missing_path() {
        let d = tempdir().unwrap();
        let missing = d.path().join("does_not_exist");
        let res = exists(&missing).unwrap();
        assert!(!res, "missing path should return Ok(false)");
    }

    #[test]
    fn returns_false_for_file() -> io::Result<()> {
        let d = tempdir()?;
        let f = d.path().join("file.txt");
        fs::write(&f, "x")?;
        assert_eq!(exists(&f)?, false, "file should return Ok(false)");
        Ok(())
    }
}
