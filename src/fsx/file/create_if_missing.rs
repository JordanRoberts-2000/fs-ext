#[macro_export]
macro_rules! create_file_if_missing {
    ($path:expr) => {{
        let path: &std::path::Path = $path.as_ref();
        match std::fs::metadata(path) {
            Ok(meta) if meta.is_file() => Ok(()),
            Ok(_) => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Path exists but is not a file",
            )),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                if let Some(parent) = path.parent() {
                    std::fs::create_dir_all(parent)?;
                }
                std::fs::File::create(path)?;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }};
    ($path:expr, $content:expr) => {{
        let path: &std::path::Path = $path.as_ref();
        match std::fs::metadata(path) {
            Ok(meta) if meta.is_file() => Ok(()),
            Ok(_) => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Path exists but is not a file",
            )),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                if let Some(parent) = path.parent() {
                    std::fs::create_dir_all(parent)?;
                }
                std::fs::write(path, $content)
            }
            Err(e) => Err(e),
        }
    }};
}

pub use create_file_if_missing as create_if_missing;
