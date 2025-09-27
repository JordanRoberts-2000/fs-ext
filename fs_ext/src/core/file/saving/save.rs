use {
    crate::{CodecError, Format},
    serde::Serialize,
    std::path::Path,
};

pub fn save<T, F>(path: impl AsRef<Path>, model: T) -> Result<(), CodecError>
where
    F: Format,
    T: Serialize,
{
    F::save(path, model)
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::formats::Json,
        serde::{Deserialize, Serialize},
        tempfile::tempdir,
    };

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct Demo {
        id: u32,
        name: String,
    }

    #[test]
    fn sync_save_smoke_json() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("demo.json");
        let model = Demo { id: 1, name: "alpha".into() };

        save::<Demo, Json>(&path, model.clone()).expect("sync save should succeed");

        let roundtrip: Demo = Json::load(&path).expect("sync load should succeed");
        assert_eq!(roundtrip, model);
    }
}
