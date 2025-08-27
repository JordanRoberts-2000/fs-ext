use {
    crate::{DeserializeError, Format, SerializeError},
    serde::{Serialize, de::DeserializeOwned},
};

pub enum Json {}
impl Format for Json {
    fn parse_str<T>(s: &str) -> Result<T, DeserializeError>
    where
        T: DeserializeOwned,
    {
        serde_json::from_str(s).map_err(DeserializeError::Json)
    }

    fn to_string<T>(value: T) -> Result<String, SerializeError>
    where
        T: Serialize,
    {
        serde_json::to_string_pretty(&value).map_err(SerializeError::Json)
    }
}

pub enum Toml {}
impl Format for Toml {
    fn parse_str<T>(s: &str) -> Result<T, DeserializeError>
    where
        T: DeserializeOwned,
    {
        toml::from_str(s).map_err(DeserializeError::Toml)
    }

    fn to_string<T>(value: T) -> Result<String, SerializeError>
    where
        T: Serialize,
    {
        toml::to_string_pretty(&value).map_err(SerializeError::Toml)
    }
}

pub enum Yaml {}
impl Format for Yaml {
    fn parse_str<T>(s: &str) -> Result<T, DeserializeError>
    where
        T: DeserializeOwned,
    {
        serde_yaml::from_str(s).map_err(DeserializeError::Yaml)
    }

    fn to_string<T>(value: T) -> Result<String, SerializeError>
    where
        T: Serialize,
    {
        serde_yaml::to_string(&value).map_err(SerializeError::Yaml)
    }
}
