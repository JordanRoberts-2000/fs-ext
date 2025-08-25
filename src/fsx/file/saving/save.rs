use {
    crate::{CodecError, Format},
    serde::Serialize,
    std::path::Path,
};

pub fn save<T, F>(path: impl AsRef<Path>, model: T) -> Result<(), CodecError>
where
    F: Format,
    T: Serialize,
{
    F::save(path, model)
}
