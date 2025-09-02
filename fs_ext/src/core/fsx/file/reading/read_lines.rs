use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

pub fn read_lines(path: impl AsRef<Path>) -> io::Result<Vec<String>> {
    _read_lines(path.as_ref())
}

fn _read_lines(path: &Path) -> io::Result<Vec<String>> {
    let file = File::open(path).map_err(|e| {
        io::Error::new(e.kind(), format!("Failed to open file '{}': {e}", path.display()))
    })?;

    let reader = BufReader::new(file);
    reader.lines().collect::<Result<Vec<_>, _>>().map_err(|e| {
        io::Error::new(e.kind(), format!("Failed to read lines from '{}': {e}", path.display()))
    })
}

#[cfg(test)]
mod tests {
    use {
        super::read_lines,
        std::{fs, io},
        tempfile::tempdir,
    };

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

    #[test]
    fn err_for_missing_path() {
        let dir = tempdir().unwrap();
        let missing = dir.path().join("nope.txt");

        let err = read_lines(&missing).unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::NotFound);
    }
}
