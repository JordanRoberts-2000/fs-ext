use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

pub fn stream_lines(
    path: impl AsRef<Path>,
) -> io::Result<impl Iterator<Item = io::Result<String>>> {
    let file = File::open(path.as_ref()).map_err(|e| {
        io::Error::new(
            e.kind(),
            format!("Failed to open file '{}': {}", path.as_ref().display(), e),
        )
    })?;
    Ok(BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use {super::stream_lines, std::fs, tempfile::tempdir};

    #[test]
    fn reads_multiple_lines() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("data.txt");
        fs::write(&file, "alpha\nbeta\ngamma\n").unwrap();

        let lines: Result<Vec<_>, _> = stream_lines(&file).unwrap().collect();
        assert_eq!(lines.unwrap(), vec!["alpha", "beta", "gamma"]);
    }

    #[test]
    fn empty_file_returns_empty_iter() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("empty.txt");
        fs::File::create(&file).unwrap();

        let lines: Result<Vec<_>, _> = stream_lines(&file).unwrap().collect();
        assert!(lines.unwrap().is_empty());
    }

    #[test]
    fn includes_last_line_without_trailing_newline() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("no_trailing_nl.txt");
        fs::write(&file, "alpha\nbeta").unwrap();

        let lines: Result<Vec<_>, _> = stream_lines(&file).unwrap().collect();
        assert_eq!(lines.unwrap(), vec!["alpha", "beta"]);
    }
}
