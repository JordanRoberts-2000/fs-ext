use {
    crate::{CodecError, Format},
    serde::de::DeserializeOwned,
    std::path::Path,
};

pub fn load<T, F>(path: impl AsRef<Path>) -> Result<T, CodecError>
where
    F: Format,
    T: DeserializeOwned,
{
    F::load(path)
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::formats::Json,
        serde::{Deserialize, Serialize},
        std::fs,
        tempfile::tempdir,
    };

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct Demo {
        id: u32,
        name: String,
    }

    #[test]
    fn sync_load_smoke_json() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("demo.json");
        fs::write(&path, r#"{ "id": 1, "name": "alpha" }"#).unwrap();

        let got: Demo = load::<Demo, Json>(&path).expect("sync load should succeed");
        assert_eq!(got, Demo { id: 1, name: "alpha".into() });
    }
}
