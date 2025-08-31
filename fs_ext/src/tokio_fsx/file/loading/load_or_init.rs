use {
    crate::{CodecError, Format, fsx, tokio_fsx::utils::join_err_to_io},
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

    task::spawn_blocking(move || fsx::file::load_or_init::<T, F>(&path, model))
        .await
        .map_err(|e| CodecError::from(join_err_to_io(e)))?
}
