use {
    std::path::{Path, PathBuf},
    tokio::{
        fs::File,
        io::{self, AsyncRead, AsyncReadExt, BufReader},
    },
};

pub type FileByteChunkReader = AsyncByteChunkReader<BufReader<File>>;

pub async fn stream_bytes(
    path: impl AsRef<Path>, chunk_size: usize,
) -> io::Result<FileByteChunkReader> {
    if chunk_size == 0 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "chunk_size must be > 0"));
    }

    let path_buf = path.as_ref().to_owned();
    let file = File::open(&path_buf).await.map_err(|e| {
        io::Error::new(e.kind(), format!("Failed to open file '{}': {}", path_buf.display(), e))
    })?;

    Ok(AsyncByteChunkReader::new(BufReader::new(file), chunk_size, path_buf))
}

pub struct AsyncByteChunkReader<R: AsyncRead + Unpin> {
    reader: R,
    buffer: Vec<u8>,
    path: PathBuf,
}

impl<R: AsyncRead + Unpin> AsyncByteChunkReader<R> {
    fn new(reader: R, chunk_size: usize, path: PathBuf) -> Self {
        Self { reader, buffer: vec![0u8; chunk_size], path }
    }

    pub async fn next_chunk(&mut self) -> Option<io::Result<Vec<u8>>> {
        match self.reader.read(&mut self.buffer).await {
            Ok(0) => None, // EOF
            Ok(n) => Some(Ok(self.buffer[..n].to_vec())),
            Err(e) => Some(Err(io::Error::new(
                e.kind(),
                format!("Failed to read chunk from '{}': {}", self.path.display(), e),
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use {
        super::{FileByteChunkReader, stream_bytes},
        std::io,
        tempfile::tempdir,
        tokio::fs,
    };

    async fn collect_chunks(mut reader: FileByteChunkReader) -> io::Result<Vec<Vec<u8>>> {
        let mut out = Vec::new();
        while let Some(chunk) = reader.next_chunk().await {
            out.push(chunk?);
        }
        Ok(out)
    }

    #[tokio::test]
    async fn chunks_exact_multiple() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("data.bin");
        // 10 bytes total -> 5 + 5
        fs::write(&file, b"abcdefghij").await.unwrap();

        let reader = stream_bytes(&file, 5).await.unwrap();
        let chunks = collect_chunks(reader).await.unwrap();

        assert_eq!(chunks.len(), 2);
        assert_eq!(chunks[0], b"abcde");
        assert_eq!(chunks[1], b"fghij");
    }

    #[tokio::test]
    async fn chunks_with_remainder() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("data.bin");
        // 13 bytes -> 5,5,3
        fs::write(&file, &b"abcdefghijklmn"[..13]).await.unwrap();

        let reader = stream_bytes(&file, 5).await.unwrap();
        let chunks = collect_chunks(reader).await.unwrap();

        assert_eq!(chunks.len(), 3);
        assert_eq!(chunks[0], b"abcde");
        assert_eq!(chunks[1], b"fghij");
        assert_eq!(chunks[2], b"klm");
    }

    #[tokio::test]
    async fn single_chunk_when_chunk_bigger_than_file() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("data.bin");
        fs::write(&file, b"hello!!").await.unwrap(); // 7 bytes

        let reader = stream_bytes(&file, 64).await.unwrap();
        let chunks = collect_chunks(reader).await.unwrap();

        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0], b"hello!!");
    }

    #[tokio::test]
    async fn empty_file_returns_no_chunks() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("empty.bin");
        fs::File::create(&file).await.unwrap();

        let mut reader = stream_bytes(&file, 8).await.unwrap();
        // first call should be EOF -> None
        assert!(reader.next_chunk().await.is_none());
        // subsequent calls should also be None
        assert!(reader.next_chunk().await.is_none());
    }

    #[tokio::test]
    async fn err_when_chunk_size_zero() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("data.bin");
        fs::write(&file, b"data").await.unwrap();

        let err = stream_bytes(&file, 0).await.err().expect("expected error");
        assert_eq!(err.kind(), io::ErrorKind::InvalidInput);
    }
}
