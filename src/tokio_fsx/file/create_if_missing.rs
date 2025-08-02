#[macro_export]
macro_rules! create_file_if_missing_async {
    ($path:expr) => {{
        let path: &std::path::Path = $path.as_ref();
        match tokio::fs::metadata(path).await {
            Ok(meta) if meta.is_file() => Ok(()),
            Ok(_) => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Path exists but is not a file",
            )),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                if let Some(parent) = path.parent() {
                    tokio::fs::create_dir_all(parent).await?;
                }
                tokio::fs::File::create(path).await?;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }};
    ($path:expr, $content:expr) => {{
        let path: &std::path::Path = $path.as_ref();
        match tokio::fs::metadata(path).await {
            Ok(meta) if meta.is_file() => Ok(()),
            Ok(_) => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Path exists but is not a file",
            )),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                if let Some(parent) = path.parent() {
                    tokio::fs::create_dir_all(parent).await?;
                }
                tokio::fs::write(path, $content).await
            }
            Err(e) => Err(e),
        }
    }};
}

pub use create_file_if_missing_async as create_if_missing;
