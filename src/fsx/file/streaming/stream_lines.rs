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
