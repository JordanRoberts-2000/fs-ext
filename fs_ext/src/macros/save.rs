#[macro_export]
macro_rules! save {
    ($path:expr, $model:expr) => {
        $crate::file::save_auto($path, &$model)
    };
}
