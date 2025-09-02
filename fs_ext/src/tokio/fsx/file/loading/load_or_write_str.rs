use {
    crate::{CodecError, Format, fsx, tokio::utils::join_err_to_io},
    serde::de::DeserializeOwned,
    std::path::Path,
    tokio::task,
};

pub async fn load_or_write_str<F, T>(path: impl AsRef<Path>, content: &str) -> Result<T, CodecError>
where
    F: Format,
    T: DeserializeOwned + Send + 'static,
{
    let path = path.as_ref().to_owned();
    let content_owned = content.to_owned();

    task::spawn_blocking(move || fsx::file::load_or_write_str::<F, T>(&path, &content_owned))
        .await
        .map_err(|e| CodecError::from(join_err_to_io(e)))?
}
