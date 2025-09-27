use {
    crate::{CodecError, Format, file, tokio::utils::join_err_to_io},
    serde::de::DeserializeOwned,
    std::path::Path,
    tokio::task,
};

pub async fn load_or_write_str<F, T>(
    path: impl AsRef<Path>, content: impl AsRef<str>,
) -> Result<T, CodecError>
where
    F: Format,
    T: DeserializeOwned + Send + 'static,
{
    let path = path.as_ref().to_owned();
    let content_owned = content.as_ref().to_owned();

    task::spawn_blocking(move || file::load_or_write_str::<F, T>(&path, &content_owned))
        .await
        .map_err(|e| CodecError::from(join_err_to_io(e)))?
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

    #[tokio::test]
    async fn async_load_or_write_str_creates_when_missing_and_parses() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("demo.json");
        let seed = r#"{ "id": 1, "name": "alpha" }"#;

        let got: Demo = load_or_write_str::<Json, _>(&path, seed)
            .await
            .expect("should create file and parse content");
        assert_eq!(got, Demo { id: 1, name: "alpha".into() });

        // File now exists with exactly the provided content
        let on_disk = fs::read_to_string(&path).unwrap();
        assert_eq!(on_disk, seed);

        let roundtrip: Demo = Json::load(&path).unwrap();
        assert_eq!(roundtrip, got);
    }

    #[tokio::test]
    async fn async_load_or_write_str_loads_existing_and_does_not_overwrite() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("demo.json");

        // Pre-write existing valid JSON
        let existing = r#"{ "id": 2, "name": "beta" }"#;
        fs::write(&path, existing).unwrap();

        // Provide different seed; must be ignored since file exists
        let new_seed = r#"{ "id": 999, "name": "IGNORED" }"#;

        let got: Demo = load_or_write_str::<Json, _>(&path, new_seed)
            .await
            .expect("should load existing file, not overwrite");
        assert_eq!(got, Demo { id: 2, name: "beta".into() });

        let on_disk = fs::read_to_string(&path).unwrap();
        assert_eq!(on_disk, existing);
    }
}
