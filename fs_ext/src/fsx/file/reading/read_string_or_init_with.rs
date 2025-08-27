use std::{fs, io, path::Path};

pub fn read_string_or_init_with<F, C>(path: impl AsRef<Path>, contents_fn: F) -> io::Result<String>
where
    F: FnOnce() -> C,
    C: AsRef<[u8]>,
{
    let path = path.as_ref();

    match fs::read_to_string(path) {
        Ok(content) => Ok(content),

        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            let contents = contents_fn();
            let bytes = contents.as_ref();

            let contents_string =
                std::str::from_utf8(bytes).map(|s| s.to_string()).map_err(|e| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Default content for '{}' is not valid UTF-8: {e}", path.display()),
                    )
                })?;

            fs::write(path, bytes).map_err(|e| {
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
        super::read_string_or_init_with,
        std::{
            fs, io,
            sync::atomic::{AtomicU32, Ordering},
        },
        tempfile::tempdir,
    };

    #[test]
    fn returns_existing_content() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("existing.txt");
        fs::write(&file, "existing content").unwrap();

        let out = read_string_or_init_with(&file, || "default content").unwrap();

        assert_eq!(out, "existing content");
        // Verify file content unchanged
        assert_eq!(fs::read_to_string(&file).unwrap(), "existing content");
    }

    #[test]
    fn creates_and_returns_default_when_missing() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("missing.txt");

        let out = read_string_or_init_with(&file, || "default content").unwrap();

        assert_eq!(out, "default content");
        assert_eq!(fs::read_to_string(&file).unwrap(), "default content");
    }

    #[test]
    fn closure_not_called_when_file_exists() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("exists.txt");
        fs::write(&file, "original").unwrap();

        let call_count = AtomicU32::new(0);
        let out = read_string_or_init_with(&file, || {
            call_count.fetch_add(1, Ordering::SeqCst);
            "should not be used"
        })
        .unwrap();

        assert_eq!(out, "original");
        assert_eq!(
            call_count.load(Ordering::SeqCst),
            0,
            "Closure should not be called when file exists"
        );
    }

    #[test]
    fn closure_called_when_file_missing() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("missing.txt");

        let call_count = AtomicU32::new(0);
        let out = read_string_or_init_with(&file, || {
            call_count.fetch_add(1, Ordering::SeqCst);
            "generated content"
        })
        .unwrap();

        assert_eq!(out, "generated content");
        assert_eq!(call_count.load(Ordering::SeqCst), 1, "Closure should be called exactly once");
    }

    #[test]
    fn error_when_existing_file_not_utf8() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("invalid_utf8.bin");
        fs::write(&file, [0xFF, 0xFE, 0xFD]).unwrap();

        let err = read_string_or_init_with(&file, || "default").unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidData);
    }

    #[test]
    fn error_when_default_content_not_utf8() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("missing.txt");

        let err = read_string_or_init_with(&file, || vec![0xFF, 0xFE, 0xFD]).unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidData);

        assert!(!file.exists(), "File should not be created when default content is invalid UTF-8");
    }
}
