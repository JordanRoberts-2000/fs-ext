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
