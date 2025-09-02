use {
    crate::{CodecError, Format, fsx, tokio::utils::join_err_to_io},
    serde::{Serialize, de::DeserializeOwned},
    std::path::Path,
    tokio::task,
};

pub async fn load_or_init_with<F, T, C>(
    path: impl AsRef<Path>, create_fn: C,
) -> Result<T, CodecError>
where
    F: Format,
    T: DeserializeOwned + Serialize + Send + 'static,
    C: FnOnce() -> T + Send + 'static,
{
    let path = path.as_ref().to_owned();

    task::spawn_blocking(move || fsx::file::load_or_init_with::<F, T, C>(&path, create_fn))
        .await
        .map_err(|e| CodecError::from(join_err_to_io(e)))?
}
