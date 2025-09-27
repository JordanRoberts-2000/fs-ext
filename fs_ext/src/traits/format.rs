use {
    crate::{CodecError, DeserializeError, SerializeError, file},
    serde::{Serialize, de::DeserializeOwned},
    std::{io::Write, path::Path},
};

pub trait Format {
    fn parse_str<T>(s: &str) -> Result<T, DeserializeError>
    where
        T: DeserializeOwned;

    fn to_string<T>(value: T) -> Result<String, SerializeError>
    where
        T: Serialize;

    fn load<T>(path: impl AsRef<Path>) -> Result<T, CodecError>
    where
        Self: Sized,
        T: serde::de::DeserializeOwned,
    {
        let s = file::read_string(path)?;
        Self::parse_str(&s).map_err(CodecError::Deserialize)
    }

    fn save<T>(path: impl AsRef<Path>, value: T) -> Result<(), CodecError>
    where
        Self: Sized,
        T: Serialize,
    {
        let s = Self::to_string(value)?;
        file::atomic::overwrite(path, |file| file.write_all(s.as_bytes()))?;

        Ok(())
    }
}
