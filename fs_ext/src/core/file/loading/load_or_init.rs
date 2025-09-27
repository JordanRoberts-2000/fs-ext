use {
    crate::{CodecError, Format},
    serde::{Serialize, de::DeserializeOwned},
    std::{io, path::Path},
};

pub fn load_or_init<T, F>(path: impl AsRef<Path>, model: T) -> Result<T, CodecError>
where
    F: Format,
    T: DeserializeOwned + Serialize,
{
    match F::load(&path) {
        Ok(v) => Ok(v),

        Err(CodecError::Io(ref e)) if e.kind() == io::ErrorKind::NotFound => {
            F::save(&path, &model)?;
            Ok(model)
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
        std::fs,
        tempfile::tempdir,
    };

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct Demo {
        id: u32,
        name: String,
    }

    #[test]
    fn creates_file_when_missing_and_returns_model() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("demo.json");
        let model = Demo { id: 1, name: "alpha".into() };

        let got: Demo = load_or_init::<Demo, Json>(&path, model.clone()).expect("ok");
        assert_eq!(got, model);

        let on_disk = fs::read_to_string(&path).unwrap();
        assert!(on_disk.contains("\"alpha\""));

        let loaded: Demo = Json::load(&path).unwrap();
        assert_eq!(loaded, model);
    }

    #[test]
    fn loads_existing_and_does_not_overwrite_with_model() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("demo.json");

        let existing = Demo { id: 2, name: "beta".into() };
        Json::save(&path, &existing).unwrap();

        let model = Demo { id: 9, name: "SHOULD_NOT_WRITE".into() };

        let got: Demo = load_or_init::<Demo, Json>(&path, model).expect("ok");
        assert_eq!(got, existing);

        // Ensure file content remains as existing
        let on_disk: Demo = Json::load(&path).unwrap();
        assert_eq!(on_disk, existing);
    }

    #[test]
    fn existing_invalid_returns_deserialize_error_and_keeps_file() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("demo.json");

        // invalid JSON
        fs::write(&path, "{ id: 3, }").unwrap();

        let model = Demo { id: 4, name: "gamma".into() };
        let err = load_or_init::<Demo, Json>(&path, model).unwrap_err();

        assert!(matches!(err, CodecError::Deserialize(DeserializeError::Json(_))));
        let _ = err;

        // File unchanged
        let raw = fs::read_to_string(&path).unwrap();
        assert_eq!(raw, "{ id: 3, }");
    }

    #[test]
    fn save_failure_propagates_io_error() {
        // Parent dir doesn't exist → create_new/save should fail
        let dir = tempdir().unwrap();
        let bad_path = dir.path().join("no_such_dir").join("demo.json");
        let model = Demo { id: 10, name: "delta".into() };

        let err = load_or_init::<Demo, Json>(&bad_path, model).unwrap_err();

        assert!(matches!(err, CodecError::Io(_)));
        let _ = err;
    }
}
