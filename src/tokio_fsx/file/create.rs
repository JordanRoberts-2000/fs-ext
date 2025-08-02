#[macro_export]
macro_rules! create_file_async {
    ($path:expr) => {
        tokio::fs::File::create($path).await
    };
    ($path:expr, $content:expr) => {
        tokio::fs::write($path, $content).await
    };
}

pub use create_file_async as create;
