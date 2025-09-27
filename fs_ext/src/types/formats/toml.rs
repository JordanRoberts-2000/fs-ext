use {
    crate::{DeserializeError, Format, SerializeError},
    serde::{Serialize, de::DeserializeOwned},
};

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

#[cfg(test)]
mod tests {
    use {
        super::*,
        serde::{Deserialize, Serialize},
        std::fs,
        tempfile::tempdir,
    };

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct Demo {
        id: u32,
        name: String,
        flags: Vec<bool>,
    }

    fn demo() -> Demo {
        Demo { id: 7, name: "beta".into(), flags: vec![false, true] }
    }

    #[test]
    fn toml_to_string_and_parse_roundtrip() {
        let v = demo();
        let s = Toml::to_string(&v).expect("toml to_string");
        // TOML pretty output commonly has `key = value`
        assert!(s.contains("name = \"beta\""));
        let back: Demo = Toml::parse_str(&s).expect("toml parse_str");
        assert_eq!(v, back);
    }

    #[test]
    fn toml_parse_invalid() {
        // invalid TOML (incomplete assignment)
        let s = "id = \nname = \"x\"";
        let err = Toml::parse_str::<Demo>(s).unwrap_err();
        assert!(matches!(err, DeserializeError::Toml(_)));
    }

    #[test]
    fn toml_save_and_load_roundtrip() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("demo.toml");
        let v = demo();

        Toml::save(&path, &v).expect("save toml");
        let loaded: Demo = Toml::load(&path).expect("load toml");
        assert_eq!(v, loaded);

        let raw = fs::read_to_string(&path).unwrap();
        assert!(raw.contains("name = \"beta\""));
    }
}
