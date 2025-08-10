use {
    std::{io, path::Path},
    tokio::{
        fs::File,
        io::{AsyncBufReadExt, BufReader},
    },
};

pub async fn stream_lines(path: impl AsRef<Path>) -> io::Result<tokio::io::Lines<BufReader<File>>> {
    let path = path.as_ref();
    let file = File::open(path).await.map_err(|e| {
        io::Error::new(e.kind(), format!("Failed to open file '{}': {}", path.display(), e))
    })?;
    Ok(BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use {super::stream_lines, tempfile::tempdir, tokio::fs};

    #[tokio::test]
    async fn reads_multiple_lines() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("data.txt");
        fs::write(&file, "alpha\nbeta\ngamma\n").await.unwrap();

        let mut lines = stream_lines(&file).await.unwrap();
        let mut out = Vec::new();
        while let Some(line) = lines.next_line().await.unwrap() {
            out.push(line);
        }
        assert_eq!(out, vec!["alpha", "beta", "gamma"]);
    }

    #[tokio::test]
    async fn empty_file_returns_empty_iter() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("empty.txt");
        fs::File::create(&file).await.unwrap();

        let mut lines = stream_lines(&file).await.unwrap();
        let mut count = 0usize;
        while let Some(_line) = lines.next_line().await.unwrap() {
            count += 1;
        }
        assert_eq!(count, 0);
    }

    #[tokio::test]
    async fn includes_last_line_without_trailing_newline() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("no_trailing_nl.txt");
        fs::write(&file, "alpha\nbeta").await.unwrap();

        let mut lines = stream_lines(&file).await.unwrap();
        let mut out = Vec::new();
        while let Some(line) = lines.next_line().await.unwrap() {
            out.push(line);
        }
        assert_eq!(out, vec!["alpha", "beta"]);
    }
}
