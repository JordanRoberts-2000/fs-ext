use {
    crate::{
        CodecError, Format,
        formats::{Json, Toml, Yaml},
    },
    serde::de::DeserializeOwned,
    std::{io, path::Path},
};

pub fn load_auto<T>(path: impl AsRef<Path>) -> Result<T, CodecError>
where
    T: DeserializeOwned,
{
    let path = path.as_ref();

    let ext =
        path.extension().and_then(|s| s.to_str()).map(|s| s.to_ascii_lowercase()).ok_or_else(
            || {
                CodecError::from(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("path '{}' has no extension", path.display()),
                ))
            },
        )?;

    match ext.as_str() {
        "toml" => Toml::load::<T>(path),
        "json" => Json::load::<T>(path),
        "yaml" | "yml" => Yaml::load::<T>(path),

        other => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "unsupported extension '{other}' for '{}'; expected one of: toml, json, yaml/yml",
                path.display()
            ),
        )
        .into()),
    }
}
