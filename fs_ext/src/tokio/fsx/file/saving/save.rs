use {
    crate::{CodecError, Format, fsx, tokio::utils::join_err_to_io},
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

    task::spawn_blocking(move || fsx::file::save::<T, F>(path, model))
        .await
        .map_err(|e| CodecError::from(join_err_to_io(e)))??;

    Ok(())
}
