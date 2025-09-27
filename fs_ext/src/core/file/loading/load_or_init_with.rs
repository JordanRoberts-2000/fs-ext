use {
    crate::{CodecError, Format},
    serde::{Serialize, de::DeserializeOwned},
    std::{io, path::Path},
};

pub fn load_or_init_with<F, T, C>(path: impl AsRef<Path>, create_fn: C) -> Result<T, CodecError>
where
    F: Format,
    T: DeserializeOwned + Serialize,
    C: FnOnce() -> T,
{
    let path = path.as_ref();

    match F::load::<T>(path) {
        Ok(v) => Ok(v),

        Err(CodecError::Io(ref e)) if e.kind() == io::ErrorKind::NotFound => {
            let value = create_fn();
            F::save(path, &value)?;
            Ok(value)
        }

        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{DeserializeError, formats::Json},
        serde::{Deserialize, Serialize},
        std::{cell::Cell, fs, rc::Rc},
        tempfile::tempdir,
    };

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct Demo {
        id: u32,
        name: String,
    }

    #[test]
    fn when_missing_calls_create_once_saves_and_returns_value() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("demo.json");

        let calls = Rc::new(Cell::new(0));
        let calls_cloned = Rc::clone(&calls);

        let got: Demo = load_or_init_with::<Json, _, _>(&path, move || {
            calls_cloned.set(calls_cloned.get() + 1);
            Demo { id: 1, name: "alpha".into() }
        })
        .expect("ok");

        assert_eq!(calls.get(), 1);
        assert_eq!(got, Demo { id: 1, name: "alpha".into() });

        // File should exist and contain serialized value
        let on_disk: Demo = Json::load(&path).unwrap();
        assert_eq!(on_disk, got);
    }

    #[test]
    fn when_existing_valid_loads_and_does_not_call_create() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("demo.json");

        let existing = Demo { id: 2, name: "beta".into() };
        Json::save(&path, &existing).unwrap();

        let calls = Rc::new(Cell::new(0));
        let calls_cloned = Rc::clone(&calls);

        let got: Demo = load_or_init_with::<Json, _, _>(&path, move || {
            calls_cloned.set(calls_cloned.get() + 1);
            Demo { id: 999, name: "should-not-create".into() }
        })
        .expect("ok");

        assert_eq!(calls.get(), 0, "create_fn must not be called when file exists");
        assert_eq!(got, existing);

        let on_disk: Demo = Json::load(&path).unwrap();
        assert_eq!(on_disk, existing);
    }

    #[test]
    fn when_existing_invalid_returns_deserialize_error_and_does_not_call_create() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("demo.json");

        // invalid JSON
        fs::write(&path, "{ id: 3, }").unwrap();

        let calls = Rc::new(Cell::new(0));
        let calls_cloned = Rc::clone(&calls);

        let err = load_or_init_with::<Json, Demo, _>(&path, move || {
            calls_cloned.set(calls_cloned.get() + 1);
            Demo { id: 4, name: "gamma".into() }
        })
        .unwrap_err();

        assert_eq!(calls.get(), 0, "create_fn must not be called on deserialize error");
        assert!(matches!(err, CodecError::Deserialize(DeserializeError::Json(_))));

        // File unchanged
        let raw = fs::read_to_string(&path).unwrap();
        assert_eq!(raw, "{ id: 3, }");
    }

    #[test]
    fn save_failure_propagates_io_error_and_create_called_once() {
        let dir = tempdir().unwrap();
        let bad_path = dir.path().join("no_such_dir").join("demo.json");

        let calls = Rc::new(Cell::new(0));
        let calls_cloned = Rc::clone(&calls);

        let err = load_or_init_with::<Json, Demo, _>(&bad_path, move || {
            calls_cloned.set(calls_cloned.get() + 1);
            Demo { id: 10, name: "delta".into() }
        })
        .unwrap_err();

        assert_eq!(calls.get(), 1, "create_fn should be called exactly once");
        assert!(matches!(err, CodecError::Io(_)));
    }
}
