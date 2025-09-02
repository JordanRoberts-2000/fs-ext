use {
    crate::{CodecError, fsx, tokio::utils::join_err_to_io},
    serde::Serialize,
    std::path::Path,
    tokio::task,
};

pub async fn save_auto<T>(path: impl AsRef<Path>, model: T) -> Result<(), CodecError>
where
    T: Serialize + Send + 'static,
{
    let path = path.as_ref().to_owned();

    task::spawn_blocking(move || fsx::file::save_auto::<T>(path, &model))
        .await
        .map_err(|e| CodecError::from(join_err_to_io(e)))??;

    Ok(())
}
