use {
    crate::{CodecError, Format},
    serde::{Serialize, de::DeserializeOwned},
    std::{io, path::Path},
};

pub fn load_or_default<T, F>(path: impl AsRef<Path>) -> Result<T, CodecError>
where
    F: Format,
    T: DeserializeOwned + Serialize + Default,
{
    match F::load(&path) {
        Ok(v) => Ok(v),

        Err(CodecError::Io(ref e)) if e.kind() == io::ErrorKind::NotFound => {
            let model = T::default();
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

    impl Default for Demo {
        fn default() -> Self {
            Self { id: 0, name: "default".into() }
        }
    }

    #[test]
    fn missing_file_uses_default_saves_and_returns() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("demo.json");

        let got: Demo = load_or_default::<Demo, Json>(&path).expect("ok");
        assert_eq!(got, Demo { id: 0, name: "default".into() });

        let on_disk: Demo = Json::load(&path).unwrap();
        assert_eq!(on_disk, got);
    }

    #[test]
    fn existing_valid_loads() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("demo.json");

        let existing = Demo { id: 2, name: "beta".into() };
        Json::save(&path, &existing).unwrap();

        let got: Demo = load_or_default::<Demo, Json>(&path).expect("ok");
        assert_eq!(got, existing);

        // File remains as existing
        let on_disk: Demo = Json::load(&path).unwrap();
        assert_eq!(on_disk, existing);
    }

    #[test]
    fn existing_invalid_returns_deserialize_error() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("demo.json");

        // invalid JSON to trigger Deserialize error path (not NotFound)
        fs::write(&path, "{ id: 3, }").unwrap();

        let err = load_or_default::<Demo, Json>(&path).unwrap_err();
        assert!(matches!(err, CodecError::Deserialize(DeserializeError::Json(_))));

        // File unchanged
        let raw = fs::read_to_string(&path).unwrap();
        assert_eq!(raw, "{ id: 3, }");
    }

    #[test]
    fn save_failure_propagates_io_error() {
        let dir = tempdir().unwrap();
        let bad_path = dir.path().join("no_such_dir").join("demo.json");

        let err = load_or_default::<Demo, Json>(&bad_path).unwrap_err();

        assert!(matches!(err, CodecError::Io(_)));
    }
}
