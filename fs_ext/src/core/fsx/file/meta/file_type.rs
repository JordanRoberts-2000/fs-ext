use {
    crate::IoResultExt,
    std::{fs, fs::FileType, io, path::Path},
};

#[cfg_attr(test, fs_ext_test_macros::fs_test(rejects_missing_path, rejects_dir))]
pub fn file_type(path: impl AsRef<Path>) -> io::Result<FileType> {
    _file_type(path.as_ref())
}

fn _file_type(path: &Path) -> io::Result<FileType> {
    let meta = fs::metadata(path).with_path_context("Failed to get metadata", path)?;

    if !meta.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Path '{}' is not a regular file", path.display()),
        ));
    }

    Ok(meta.file_type())
}

#[cfg(test)]
mod tests {
    use {super::file_type, std::fs, tempfile::tempdir};

    #[test]
    fn returns_filetype_for_existing_file() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("a.txt");
        fs::write(&file, "hi").unwrap();

        let ft = file_type(&file).unwrap();
        assert!(ft.is_file(), "expected is_file() to be true");
    }
}
