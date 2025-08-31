#[macro_export]
macro_rules! dir {
    ($path:expr) => {{ $crate::fsx::dir::ensure($path) }};
}
