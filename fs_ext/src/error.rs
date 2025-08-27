use std::io;

#[derive(Debug, thiserror::Error)]
pub enum DeserializeError {
    #[error("JSON parse error in")]
    Json(serde_json::Error),
    #[error("JSON parse error in")]
    Toml(toml::de::Error),
    #[error("JSON parse error in")]
    Yaml(serde_yaml::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum SerializeError {
    #[error("JSON parse error in")]
    Json(serde_json::Error),
    #[error("JSON parse error in")]
    Toml(toml::ser::Error),
    #[error("JSON parse error in")]
    Yaml(serde_yaml::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum CodecError {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Deserialize(#[from] DeserializeError),
    #[error(transparent)]
    Serialize(#[from] SerializeError),
}
