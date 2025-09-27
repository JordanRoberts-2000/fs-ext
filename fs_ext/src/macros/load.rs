#[macro_export]
macro_rules! load {
    ($path:expr) => {
        $crate::file::load_auto($path)
    };
}
