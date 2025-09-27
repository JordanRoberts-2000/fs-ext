use {
    crate::{CodecError, file, tokio::utils::join_err_to_io},
    serde::de::DeserializeOwned,
    std::path::Path,
    tokio::task,
};

pub async fn load_auto<T>(path: impl AsRef<Path>) -> Result<T, CodecError>
where
    T: DeserializeOwned + Send + 'static,
{
    let path = path.as_ref().to_owned();

    task::spawn_blocking(move || file::load_auto::<T>(path))
        .await
        .map_err(|e| CodecError::from(join_err_to_io(e)))?
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
    }

    #[tokio::test]
    async fn load_auto_async_smoke_json() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("demo.json");
        fs::write(&path, r#"{ "id": 1, "name": "alpha" }"#).unwrap();

        let got: Demo = load_auto(&path).await.expect("async load_auto should succeed");
        assert_eq!(got, Demo { id: 1, name: "alpha".into() });
    }
}
