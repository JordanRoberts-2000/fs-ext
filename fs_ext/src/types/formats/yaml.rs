use {
    crate::{DeserializeError, Format, SerializeError},
    serde::{Serialize, de::DeserializeOwned},
};

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
        Demo { id: 1, name: "gamma".into(), flags: vec![true] }
    }

    #[test]
    fn yaml_to_string_and_parse_roundtrip() {
        let v = demo();
        let s = Yaml::to_string(&v).expect("yaml to_string");
        // YAML typically has `key: value`
        assert!(s.contains("name: gamma"));
        let back: Demo = Yaml::parse_str(&s).expect("yaml parse_str");
        assert_eq!(v, back);
    }

    #[test]
    fn yaml_parse_invalid() {
        // invalid YAML (unterminated sequence)
        let s = "id: [1, 2\nname: x";
        let err = Yaml::parse_str::<Demo>(s).unwrap_err();
        assert!(matches!(err, DeserializeError::Yaml(_)));
    }

    #[test]
    fn yaml_save_and_load_roundtrip() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("demo.yaml");
        let v = demo();

        Yaml::save(&path, &v).expect("save yaml");
        let loaded: Demo = Yaml::load(&path).expect("load yaml");
        assert_eq!(v, loaded);

        let raw = fs::read_to_string(&path).unwrap();
        assert!(raw.contains("name: gamma"));
    }
}
