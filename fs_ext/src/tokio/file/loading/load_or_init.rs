use {
    crate::{CodecError, Format, file, tokio::utils::join_err_to_io},
    serde::{Serialize, de::DeserializeOwned},
    std::path::Path,
    tokio::task,
};

pub async fn load_or_init<T, F>(path: impl AsRef<Path>, model: T) -> Result<T, CodecError>
where
    F: Format,
    T: DeserializeOwned + Serialize + Send + 'static,
{
    let path = path.as_ref().to_owned();

    task::spawn_blocking(move || file::load_or_init::<T, F>(&path, model))
        .await
        .map_err(|e| CodecError::from(join_err_to_io(e)))?
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

    #[tokio::test]
    async fn async_load_or_init_creates_when_missing() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("demo.json");
        let model = Demo { id: 1, name: "alpha".into() };

        let expected = model.clone();

        let got: Demo =
            load_or_init::<Demo, Json>(&path, model).await.expect("should create and return model");

        assert_eq!(got, expected);

        // File should exist with the serialized model
        let roundtrip: Demo = Json::load(&path).expect("sync load after create");
        assert_eq!(roundtrip, expected);
    }

    #[tokio::test]
    async fn async_load_or_init_loads_existing_and_ignores_model() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("demo.json");

        let existing = Demo { id: 42, name: "beta".into() };
        Json::save(&path, &existing).unwrap();

        // This should be ignored because the file already exists.
        let provided = Demo { id: 999, name: "IGNORED".into() };

        let got: Demo =
            load_or_init::<Demo, Json>(&path, provided).await.expect("should load existing");

        assert_eq!(got, existing);

        // Ensure file unchanged
        let roundtrip: Demo = Json::load(&path).unwrap();
        assert_eq!(roundtrip, existing);
    }
}
