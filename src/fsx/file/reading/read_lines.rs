use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

pub fn read_lines(path: impl AsRef<Path>) -> io::Result<Vec<String>> {
    _read_lines(path.as_ref())
}

fn _read_lines(path: &Path) -> io::Result<Vec<String>> {
    let file = File::open(path).map_err(|e| {
        io::Error::new(e.kind(), format!("Failed to open file '{}': {e}", path.display()))
    })?;

    let reader = BufReader::new(file);
    reader.lines().collect::<Result<Vec<_>, _>>().map_err(|e| {
        io::Error::new(e.kind(), format!("Failed to read lines from '{}': {e}", path.display()))
    })
}
