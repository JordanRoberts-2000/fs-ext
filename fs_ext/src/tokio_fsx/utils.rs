use {std::io, tokio::task};

pub(crate) async fn asyncify<F, T>(f: F) -> io::Result<T>
where
    F: FnOnce() -> io::Result<T> + Send + 'static,
    T: Send + 'static,
{
    task::spawn_blocking(f).await.map_err(join_err_to_io)?
}

fn join_err_to_io(e: tokio::task::JoinError) -> io::Error {
    let msg = if e.is_cancelled() {
        "blocking task was cancelled"
    } else if e.is_panic() {
        "blocking task panicked"
    } else {
        "blocking task failed"
    };
    io::Error::new(io::ErrorKind::Other, format!("{msg}: {e:?}"))
}
