use {
    crate::fsx,
    serde::{Deserialize, de::DeserializeOwned},
    std::{io, path::Path},
};

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("JSON parse error in")]
    Json(serde_json::Error),
    #[error("JSON parse error in")]
    Toml(toml::de::Error),
    #[error("JSON parse error in")]
    Yaml(serde_yaml::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum LoadError {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Parse(#[from] ParseError),
}

pub trait Parser {
    fn parse_str<'de, T: Deserialize<'de>>(s: &'de str) -> Result<T, ParseError>;
}

pub enum Json {}
impl Parser for Json {
    fn parse_str<'de, T: Deserialize<'de>>(s: &'de str) -> Result<T, ParseError> {
        serde_json::from_str(s).map_err(ParseError::Json)
    }
}

pub enum Toml {}
impl Parser for Toml {
    fn parse_str<'de, T: Deserialize<'de>>(s: &'de str) -> Result<T, ParseError> {
        toml::from_str(s).map_err(ParseError::Toml)
    }
}

pub enum Yaml {}
impl Parser for Yaml {
    fn parse_str<'de, T: Deserialize<'de>>(s: &'de str) -> Result<T, ParseError> {
        serde_yaml::from_str(s).map_err(ParseError::Yaml)
    }
}

pub fn load<P, T>(path: &Path) -> Result<T, LoadError>
where
    P: Parser,
    T: DeserializeOwned,
{
    let s = fsx::file::read_string(path)?;
    P::parse_str::<T>(&s).map_err(LoadError::Parse)
}
