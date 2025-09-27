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
