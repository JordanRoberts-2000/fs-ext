use {
    crate::{CodecError, Format, file, tokio::utils::join_err_to_io},
    serde::de::DeserializeOwned,
    std::path::Path,
    tokio::task,
};

pub async fn load<T, F>(path: impl AsRef<Path>) -> Result<T, CodecError>
where
    F: Format,
    T: DeserializeOwned + Send + 'static,
{
    let path = path.as_ref().to_owned();

    task::spawn_blocking(move || file::load::<T, F>(path))
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
    async fn async_load_smoke_json() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("demo.json");
        fs::write(&path, r#"{ "id": 1, "name": "alpha" }"#).unwrap();

        let got: Demo = load::<Demo, Json>(&path).await.expect("async load should succeed");

        assert_eq!(got, Demo { id: 1, name: "alpha".into() });
    }
}
