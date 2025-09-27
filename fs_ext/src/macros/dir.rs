#[macro_export]
macro_rules! dir {
    ($path:expr) => {{ $crate::dir::ensure($path) }};
}
