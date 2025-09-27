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
            let contents_string =
                std::str::from_utf8(contents).map(|s| s.to_string()).map_err(|e| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Default content for '{}' is not valid UTF-8: {e}", path.display()),
                    )
                })?;

            fs::write(path, contents).map_err(|e| {
                io::Error::new(
                    e.kind(),
                    format!("Failed to write default content to '{}': {e}", path.display()),
                )
            })?;

            Ok(contents_string)
        }

        Err(e) => Err(io::Error::new(
            e.kind(),
            format!("Failed to read file '{}' as string: {e}", path.display()),
        )),
    }
}

#[cfg(test)]
mod tests {
    use {
        super::read_string_or_init,
        std::{fs, io},
        tempfile::tempdir,
    };

    #[test]
    fn returns_existing_content() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("data.txt");
        fs::write(&file, "hi").unwrap();

        let out = read_string_or_init(&file, "default").unwrap();
        assert_eq!(out, "hi");

        // file content unchanged
        assert_eq!(fs::read_to_string(&file).unwrap(), "hi");
    }

    #[test]
    fn creates_and_returns_default_when_missing() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("new.txt");

        let out = read_string_or_init(&file, "hello").unwrap();
        assert_eq!(out, "hello");

        assert_eq!(fs::read_to_string(&file).unwrap(), "hello");
    }

    #[test]
    fn error_when_existing_file_not_utf8() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("bad.bin");
        fs::write(&file, [0xFF, 0xFE, 0xFD]).unwrap();

        let err = read_string_or_init(&file, "default").unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidData);
    }

    #[test]
    fn error_when_default_content_not_utf8() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("missing.txt");

        let err = read_string_or_init(&file, vec![0xFF, 0xFE, 0xFD]).unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidData);
    }
}
