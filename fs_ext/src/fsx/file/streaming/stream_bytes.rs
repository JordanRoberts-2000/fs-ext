use std::{
    fs::File,
    io::{self, BufReader, Read},
    path::Path,
};

pub fn stream_bytes(
    path: impl AsRef<Path>, chunk_size: usize,
) -> io::Result<impl Iterator<Item = io::Result<Vec<u8>>>> {
    if chunk_size == 0 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "chunk_size must be > 0"));
    }

    let path_buf = path.as_ref().to_owned();
    let file = File::open(&path_buf).map_err(|e| {
        io::Error::new(e.kind(), format!("Failed to open file '{}': {}", path_buf.display(), e))
    })?;

    Ok(ByteChunkIterator::new(BufReader::new(file), chunk_size, path_buf))
}

struct ByteChunkIterator<R: Read> {
    reader: R,
    buffer: Vec<u8>,
    path: std::path::PathBuf,
}

impl<R: Read> ByteChunkIterator<R> {
    fn new(reader: R, chunk_size: usize, path: std::path::PathBuf) -> Self {
        Self { reader, buffer: vec![0u8; chunk_size], path }
    }
}

impl<R: Read> Iterator for ByteChunkIterator<R> {
    type Item = io::Result<Vec<u8>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.reader.read(&mut self.buffer) {
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
        super::stream_bytes,
        std::{fs, io},
        tempfile::tempdir,
    };

    #[test]
    fn chunks_exact_multiple() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("data.bin");
        // 10 bytes total
        fs::write(&file, b"abcdefghij").unwrap();

        let chunks: Result<Vec<_>, _> = stream_bytes(&file, 5).unwrap().collect();
        let chunks = chunks.unwrap();

        assert_eq!(chunks.len(), 2);
        assert_eq!(chunks[0], b"abcde");
        assert_eq!(chunks[1], b"fghij");
    }

    #[test]
    fn chunks_with_remainder() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("data.bin");
        // 13 bytes -> 5,5,3
        fs::write(&file, b"abcdefghijklmn"[..13].to_vec()).unwrap();

        let chunks: Result<Vec<_>, _> = stream_bytes(&file, 5).unwrap().collect();
        let chunks = chunks.unwrap();

        assert_eq!(chunks.len(), 3);
        assert_eq!(chunks[0], b"abcde");
        assert_eq!(chunks[1], b"fghij");
        assert_eq!(chunks[2], b"klm");
    }

    #[test]
    fn single_chunk_when_chunk_bigger_than_file() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("data.bin");
        fs::write(&file, b"hello!!").unwrap(); // 7 bytes

        let chunks: Result<Vec<_>, _> = stream_bytes(&file, 64).unwrap().collect();
        let chunks = chunks.unwrap();

        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0], b"hello!!");
    }

    #[test]
    fn empty_file_returns_empty_iter() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("empty.bin");
        fs::File::create(&file).unwrap();

        let chunks: Result<Vec<_>, _> = stream_bytes(&file, 8).unwrap().collect();
        let chunks = chunks.unwrap();
        assert!(chunks.is_empty(), "expected no chunks for empty file");
    }

    #[test]
    fn err_when_chunk_size_zero() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("data.bin");
        fs::write(&file, b"data").unwrap();

        let err = stream_bytes(&file, 0).err().expect("expected error");
        assert_eq!(err.kind(), io::ErrorKind::InvalidInput);
    }
}
