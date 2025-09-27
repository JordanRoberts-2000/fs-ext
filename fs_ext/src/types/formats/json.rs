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
        Demo { id: 42, name: "alpha".into(), flags: vec![true, false, true] }
    }

    #[test]
    fn json_to_string_and_parse_roundtrip() {
        let v = demo();
        let s = Json::to_string(&v).expect("json to_string");
        assert!(s.contains("\"name\""));
        let back: Demo = Json::parse_str(&s).expect("json parse_str");
        assert_eq!(v, back);
    }

    #[test]
    fn json_parse_invalid() {
        // invalid JSON (unquoted key, trailing comma)
        let s = "{ id: 1, }";
        let err = Json::parse_str::<Demo>(s).unwrap_err();
        assert!(matches!(err, DeserializeError::Json(_)));
    }

    #[test]
    fn json_save_and_load_roundtrip() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("demo.json");
        let v = demo();

        Json::save(&path, &v).expect("save json");
        let loaded: Demo = Json::load(&path).expect("load json");
        assert_eq!(v, loaded);

        let raw = fs::read_to_string(&path).unwrap();
        assert!(raw.contains("\"alpha\""));
    }
}
