use {
    crate::fsx,
    std::{io, path::Path},
};

pub fn is_empty(path: &Path) -> io::Result<bool> {
    Ok(fsx::file::size(path)? == 0)
}
