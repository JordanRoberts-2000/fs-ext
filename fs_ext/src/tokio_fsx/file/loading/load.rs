use {
    crate::{CodecError, Format},
    serde::de::DeserializeOwned,
    std::{
        io,
        path::{Path, PathBuf},
    },
    tokio::task,
};

pub async fn load<F, T>(path: impl AsRef<Path>) -> Result<T, CodecError>
where
    F: Format,
    T: DeserializeOwned + Send + 'static,
{
    let path: PathBuf = path.as_ref().to_owned();

    task::spawn_blocking(move || F::load::<T>(path))
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("join error: {e}")))?
}
