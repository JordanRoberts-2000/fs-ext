use {
    crate::{CodecError, Format, file, tokio::utils::join_err_to_io},
    serde::{Serialize, de::DeserializeOwned},
    std::path::Path,
    tokio::task,
};

pub async fn load_or_default<T, F>(path: impl AsRef<Path>) -> Result<T, CodecError>
where
    F: Format,
    T: DeserializeOwned + Serialize + Default + Send + 'static,
{
    let path = path.as_ref().to_owned();

    task::spawn_blocking(move || file::load_or_default::<T, F>(&path))
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

    impl Default for Demo {
        fn default() -> Self {
            Demo { id: 0, name: "default".into() }
        }
    }

    #[tokio::test]
    async fn async_load_or_default_creates_when_missing() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("demo.json");

        // File doesn't exist yet → should write default and return it
        let got: Demo = load_or_default::<Demo, Json>(&path)
            .await
            .expect("async load_or_default should succeed on missing file");

        assert_eq!(got, Demo { id: 0, name: "default".into() });

        let loaded_sync: Demo = Json::load(&path).expect("sync load should succeed");
        assert_eq!(loaded_sync, got);
    }

    #[tokio::test]
    async fn async_load_or_default_loads_existing() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("demo.json");

        let existing = Demo { id: 42, name: "alpha".into() };
        Json::save(&path, &existing).unwrap();

        let got: Demo = load_or_default::<Demo, Json>(&path)
            .await
            .expect("async load_or_default should load existing");

        assert_eq!(got, existing);

        // Ensure file remained unchanged
        let loaded_sync: Demo = Json::load(&path).unwrap();
        assert_eq!(loaded_sync, existing);
    }
}
