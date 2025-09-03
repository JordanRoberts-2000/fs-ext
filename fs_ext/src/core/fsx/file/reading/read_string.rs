use {
    crate::IoResultExt,
    std::{fs, io, path::Path},
};

#[cfg_attr(test, fs_ext_test_macros::fs_test(rejects_missing_path, rejects_dir))]
pub fn read_string(path: impl AsRef<Path>) -> io::Result<String> {
    _read_string(path.as_ref())
}

fn _read_string(path: &Path) -> io::Result<String> {
    fs::read_to_string(path).with_path_context("failed to read file as string", path)
}

#[cfg(test)]
mod tests {
    use {super::read_string, std::fs, tempfile::tempdir};

    #[test]
    fn returns_string_for_existing_file() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("data.txt");
        fs::write(&file, "hello").unwrap();

        let s = read_string(&file).unwrap();
        assert_eq!(s, "hello");
    }
}
