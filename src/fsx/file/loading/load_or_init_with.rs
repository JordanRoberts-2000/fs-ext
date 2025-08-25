use {
    crate::{CodecError, Format},
    serde::{Serialize, de::DeserializeOwned},
    std::{io, path::Path},
};

pub fn load_or_init_with<F, T, C>(path: impl AsRef<Path>, create_fn: C) -> Result<T, CodecError>
where
    F: Format,
    T: DeserializeOwned + Serialize,
    C: FnOnce() -> T,
{
    let path = path.as_ref();

    match F::load::<T>(path) {
        Ok(v) => Ok(v),

        Err(CodecError::Io(ref e)) if e.kind() == io::ErrorKind::NotFound => {
            let value = create_fn();
            F::save(path, &value)?;
            Ok(value)
        }

        Err(e) => Err(e),
    }
}
