use {
    crate::IoResultExt,
    std::{
        fs::File,
        io::{self, BufRead, BufReader},
        path::Path,
    },
};

#[cfg_attr(test, fs_ext_test_macros::fs_test(rejects_missing_path, rejects_dir))]
pub fn read_lines(path: impl AsRef<Path>) -> io::Result<Vec<String>> {
    _read_lines(path.as_ref())
}

fn _read_lines(path: &Path) -> io::Result<Vec<String>> {
    let file = File::open(path).with_path_context("failed to open file", path)?;

    let reader = BufReader::new(file);
    reader
        .lines()
        .collect::<Result<Vec<_>, _>>()
        .with_path_context("failed to read lines from", path)
}

#[cfg(test)]
mod tests {
    use {super::read_lines, std::fs, tempfile::tempdir};

    #[test]
    fn reads_multiple_lines() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("data.txt");
        fs::write(&file, "alpha\nbeta\ngamma\n").unwrap();

        let lines = read_lines(&file).unwrap();
        assert_eq!(lines, vec!["alpha", "beta", "gamma"]);
    }

    #[test]
    fn empty_file_returns_empty_vec() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("empty.txt");
        fs::File::create(&file).unwrap();

        let lines = read_lines(&file).unwrap();
        assert!(lines.is_empty());
    }
}
