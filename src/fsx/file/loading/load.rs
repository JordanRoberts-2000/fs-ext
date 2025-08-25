use {
    crate::{CodecError, Format},
    serde::de::DeserializeOwned,
    std::path::Path,
};

pub fn load<T, F>(path: impl AsRef<Path>) -> Result<T, CodecError>
where
    F: Format,
    T: DeserializeOwned,
{
    F::load(path)
}
