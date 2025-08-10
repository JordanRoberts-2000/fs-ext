use {
    std::{io, path::Path},
    tokio::{
        fs::File,
        io::{AsyncBufReadExt, BufReader},
    },
};

pub async fn read_lines(path: impl AsRef<Path>) -> io::Result<Vec<String>> {
    _read_lines(path.as_ref()).await
}

async fn _read_lines(path: &Path) -> io::Result<Vec<String>> {
    let file = File::open(path).await.map_err(|e| {
        io::Error::new(e.kind(), format!("Failed to open file '{}': {e}", path.display()))
    })?;

    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut out = Vec::new();

    while let Some(line) = lines.next_line().await.map_err(|e| {
        io::Error::new(e.kind(), format!("Failed to read lines from '{}': {e}", path.display()))
    })? {
        out.push(line);
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use {super::read_lines, std::io, tempfile::tempdir, tokio::fs};

    #[tokio::test]
    async fn reads_multiple_lines() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("data.txt");
        fs::write(&file, "alpha\nbeta\ngamma\n").await.unwrap();

        let lines = read_lines(&file).await.unwrap();
        assert_eq!(lines, vec!["alpha", "beta", "gamma"]);
    }

    #[tokio::test]
    async fn empty_file_returns_empty_vec() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("empty.txt");
        fs::File::create(&file).await.unwrap();

        let lines = read_lines(&file).await.unwrap();
        assert!(lines.is_empty());
    }

    #[tokio::test]
    async fn err_for_missing_path() {
        let dir = tempdir().unwrap();
        let missing = dir.path().join("nope.txt");

        let err = read_lines(&missing).await.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::NotFound);
    }
}
