use std::{fs, fs::FileType, io, path::Path};

pub fn file_type(path: impl AsRef<Path>) -> io::Result<FileType> {
    _file_type(path.as_ref())
}

fn _file_type(path: &Path) -> io::Result<FileType> {
    fs::metadata(path).map(|meta| meta.file_type()).map_err(|e| {
        io::Error::new(e.kind(), format!("Failed to get file type for '{}': {e}", path.display()))
    })
}
