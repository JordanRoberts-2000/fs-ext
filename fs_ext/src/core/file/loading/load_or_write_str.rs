use {
    crate::{CodecError, Format, file},
    serde::de::DeserializeOwned,
    std::{
        io::{self, Write},
        path::Path,
    },
};

pub fn load_or_write_str<F, T>(
    path: impl AsRef<Path>, str: impl AsRef<str>,
) -> Result<T, CodecError>
where
    F: Format,
    T: DeserializeOwned,
{
    let path = path.as_ref();
    let str = str.as_ref();

    match F::load::<T>(path) {
        Ok(v) => Ok(v),

        Err(CodecError::Io(ref e)) if e.kind() == std::io::ErrorKind::NotFound => {
            file::atomic::create_new(path, |file| -> io::Result<()> {
                file.write_all(str.as_bytes())?;
                file.sync_all()?;
                Ok(())
            })?;

            F::parse_str::<T>(str).map_err(CodecError::Deserialize)
        }

        Err(e) => Err(e),
    }
}
