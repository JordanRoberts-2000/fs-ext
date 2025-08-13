use {
    crate::fsx::TempFile,
    std::{io, path::Path},
};

pub fn temp_in(dir: impl AsRef<Path>) -> io::Result<TempFile> {
    TempFile::in_dir(dir)
}
