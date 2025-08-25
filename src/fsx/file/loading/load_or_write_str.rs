use {
    crate::{CodecError, Format, fsx},
    serde::de::DeserializeOwned,
    std::{io::Write, path::Path},
};

pub fn load_or_write_str<F, T>(path: impl AsRef<Path>, str: &str) -> Result<T, CodecError>
where
    F: Format,
    T: DeserializeOwned,
{
    let path = path.as_ref();

    match F::load::<T>(path) {
        Ok(v) => Ok(v),

        Err(CodecError::Io(ref e)) if e.kind() == std::io::ErrorKind::NotFound => {
            fsx::file::atomic::create_new(path, |file| {
                file.write_all(str.as_bytes())?;
                file.sync_all()?;
                Ok(())
            })?;

            F::parse_str::<T>(str).map_err(CodecError::Deserialize)
        }

        Err(e) => Err(e),
    }
}
