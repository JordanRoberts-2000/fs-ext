use {
    crate::{CodecError, Format, file, tokio::utils::join_err_to_io},
    serde::Serialize,
    std::path::Path,
    tokio::task,
};

pub async fn save<T, F>(path: impl AsRef<Path>, model: T) -> Result<(), CodecError>
where
    F: Format,
    T: Serialize + Send + 'static,
{
    let path = path.as_ref().to_owned();

    task::spawn_blocking(move || file::save::<T, F>(path, model))
        .await
        .map_err(|e| CodecError::from(join_err_to_io(e)))??;

    Ok(())
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
    async fn async_save_smoke_json() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("demo.json");
        let model = Demo { id: 1, name: "alpha".into() };

        save::<Demo, Json>(&path, model.clone()).await.expect("async save should succeed");

        let roundtrip: Demo = Json::load(&path).expect("sync load should succeed");
        assert_eq!(roundtrip, model);
    }
}
