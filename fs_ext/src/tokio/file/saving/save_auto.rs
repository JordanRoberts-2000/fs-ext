use {
    crate::{CodecError, file, tokio::utils::join_err_to_io},
    serde::Serialize,
    std::path::Path,
    tokio::task,
};

pub async fn save_auto<T>(path: impl AsRef<Path>, model: T) -> Result<(), CodecError>
where
    T: Serialize + Send + 'static,
{
    let path = path.as_ref().to_owned();

    task::spawn_blocking(move || file::save_auto::<T>(path, &model))
        .await
        .map_err(|e| CodecError::from(join_err_to_io(e)))??;

    Ok(())
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{Format, formats::Json},
        serde::{Deserialize, Serialize},
        tempfile::tempdir,
    };

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct Demo {
        id: u32,
        name: String,
    }

    #[tokio::test]
    async fn async_save_auto_smoke_json() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("demo.json");
        let model = Demo { id: 1, name: "alpha".into() };

        save_auto(&path, model.clone()).await.expect("async save_auto should succeed");

        let roundtrip: Demo = Json::load(&path).expect("sync load should succeed");
        assert_eq!(roundtrip, model);
    }
}
