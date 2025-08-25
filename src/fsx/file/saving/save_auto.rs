use {
    crate::{
        CodecError, Format,
        formats::{Json, Toml, Yaml},
    },
    serde::Serialize,
    std::{io, path::Path},
};

pub fn save_auto<T, F>(path: impl AsRef<Path>, model: &T) -> Result<(), CodecError>
where
    F: Format,
    T: Serialize,
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
        "toml" => Toml::save(path, model),
        "json" => Json::save(path, model),
        "yaml" | "yml" => Yaml::save(path, model),

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
