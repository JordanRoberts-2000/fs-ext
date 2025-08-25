use {
    crate::{CodecError, Format},
    serde::{Serialize, de::DeserializeOwned},
    std::{io, path::Path},
};

pub fn load_or_init<T, F>(path: impl AsRef<Path>, model: T) -> Result<T, CodecError>
where
    F: Format,
    T: DeserializeOwned + Serialize,
{
    match F::load(&path) {
        Ok(v) => Ok(v),

        Err(CodecError::Io(ref e)) if e.kind() == io::ErrorKind::NotFound => {
            F::save(&path, &model)?;
            Ok(model)
        }

        Err(e) => Err(e),
    }
}
