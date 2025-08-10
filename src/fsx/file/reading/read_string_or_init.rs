use std::{fs, io, path::Path};

pub fn read_string_or_init(
    path: impl AsRef<Path>, contents: impl AsRef<[u8]>,
) -> io::Result<String> {
  
    _read_string_or_init(path.as_ref(), contents.as_ref())
}

fn _read_string_or_init(path: &Path, contents: &[u8]) -> io::Result<String> {
    match fs::read_to_string(path) {
        Ok(content) => Ok(content),

        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            fs::write(path, contents).map_err(|e| {
                io::Error::new(
                    e.kind(),
                    format!("Failed to write default content to '{}': {e}", path.display()),
                )
            })?;

            std::str::from_utf8(contents).map(|s| s.to_string()).map_err(|e| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Default content for '{}' is not valid UTF-8: {e}", path.display()),
                )
            })
        }

        Err(e) => Err(io::Error::new(
            e.kind(),
            format!("Failed to read file '{}' as string: {e}", path.display()),
        )),
    }
}
