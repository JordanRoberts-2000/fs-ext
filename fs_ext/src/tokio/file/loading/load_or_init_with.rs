use {
    crate::{CodecError, Format, file, tokio::utils::join_err_to_io},
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

    task::spawn_blocking(move || file::load_or_init_with::<F, T, C>(&path, create_fn))
        .await
        .map_err(|e| CodecError::from(join_err_to_io(e)))?
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::formats::Json,
        serde::{Deserialize, Serialize},
        std::sync::{
            Arc,
            atomic::{AtomicUsize, Ordering},
        },
        tempfile::tempdir,
    };

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct Demo {
        id: u32,
        name: String,
    }

    #[tokio::test]
    async fn async_load_or_init_with_creates_when_missing_calls_once() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("demo.json");

        let calls = Arc::new(AtomicUsize::new(0));
        let calls_cloned = Arc::clone(&calls);

        let got: Demo = load_or_init_with::<Json, _, _>(&path, move || {
            calls_cloned.fetch_add(1, Ordering::SeqCst);
            Demo { id: 1, name: "alpha".into() }
        })
        .await
        .expect("should create and return value");

        assert_eq!(calls.load(Ordering::SeqCst), 1, "create_fn must be called exactly once");
        assert_eq!(got, Demo { id: 1, name: "alpha".into() });

        // File should exist with serialized value
        let roundtrip: Demo = Json::load(&path).expect("sync load after create");
        assert_eq!(roundtrip, got);
    }

    #[tokio::test]
    async fn async_load_or_init_with_loads_existing_and_does_not_call_fn() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("demo.json");

        let existing = Demo { id: 42, name: "beta".into() };
        Json::save(&path, &existing).unwrap();

        let calls = Arc::new(AtomicUsize::new(0));
        let calls_cloned = Arc::clone(&calls);

        // This would be used if missing, but should be ignored here.
        let got: Demo = load_or_init_with::<Json, _, _>(&path, move || {
            calls_cloned.fetch_add(1, Ordering::SeqCst);
            Demo { id: 999, name: "IGNORED".into() }
        })
        .await
        .expect("should load existing");

        assert_eq!(calls.load(Ordering::SeqCst), 0, "create_fn must not be called");
        assert_eq!(got, existing);

        // Ensure file unchanged
        let roundtrip: Demo = Json::load(&path).unwrap();
        assert_eq!(roundtrip, existing);
    }
}
