#[macro_export]
macro_rules! load {
    ($path:expr) => {
        $crate::fsx::file::load_auto($path)
    };
}
