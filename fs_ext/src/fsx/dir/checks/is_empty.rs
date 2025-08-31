use {
    crate::IoResultExt,
    std::{fs, io, path::Path},
};

#[cfg_attr(test, fs_ext_test_macros::fs_test(rejects_missing_path, rejects_file, existing_dir_ok))]
pub fn is_empty(path: impl AsRef<Path>) -> io::Result<bool> {
    let path = path.as_ref();
    let mut entries = fs::read_dir(&path).with_path_context("Failed to read directory ", &path)?;
    Ok(entries.next().is_none())
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        std::{fs, io},
        tempfile::tempdir,
    };

    #[test]
    fn existing_dir_ok_when_empty() -> io::Result<()> {
        let d = tempdir()?;
        assert_eq!(is_empty(d.path())?, true);
        Ok(())
    }

    #[test]
    fn nonempty_dir_with_file_is_false() -> io::Result<()> {
        let d = tempdir()?;
        let f = d.path().join("a.txt");
        fs::write(&f, b"x")?;
        assert_eq!(is_empty(d.path())?, false);
        Ok(())
    }

    #[test]
    fn nonempty_dir_with_subdir_is_false() -> io::Result<()> {
        let d = tempdir()?;
        fs::create_dir(d.path().join("sub"))?;
        assert!(!is_empty(d.path())?);
        Ok(())
    }
}
