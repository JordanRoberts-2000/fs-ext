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
