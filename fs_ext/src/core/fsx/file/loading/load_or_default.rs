use {
    crate::{CodecError, Format},
    serde::{Serialize, de::DeserializeOwned},
    std::{io, path::Path},
};

pub fn load_or_default<T, F>(path: impl AsRef<Path>) -> Result<T, CodecError>
where
    F: Format,
    T: DeserializeOwned + Serialize + Default,
{
    match F::load(&path) {
        Ok(v) => Ok(v),

        Err(CodecError::Io(ref e)) if e.kind() == io::ErrorKind::NotFound => {
            let model = T::default();
            F::save(&path, &model)?;
            Ok(model)
        }

        Err(e) => Err(e),
    }
}
