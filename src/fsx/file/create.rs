#[macro_export]
macro_rules! create_file {
    ($path:expr) => {
        std::fs::File::create($path)
    };
    ($path:expr, $content:expr) => {
        std::fs::write($path, $content)
    };
}

pub use create_file as create;
