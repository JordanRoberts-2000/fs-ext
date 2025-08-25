use {
    crate::{CodecError, Format},
    serde::Serialize,
    std::{
        io,
        path::{Path, PathBuf},
    },
    tokio::task,
};

pub async fn save<F, T>(path: impl AsRef<Path>, model: T) -> Result<(), CodecError>
where
    F: Format,
    T: Serialize + Send + 'static,
{
    let path: PathBuf = path.as_ref().to_owned();

    task::spawn_blocking(move || F::save::<T>(path, model))
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("join error: {e}")))?
}
