use {
    crate::{CodecError, Format, file},
    serde::de::DeserializeOwned,
    std::{
        io::{self, Write},
        path::Path,
    },
};

pub fn load_or_write_str<F, T>(
    path: impl AsRef<Path>, str: impl AsRef<str>,
) -> Result<T, CodecError>
where
    F: Format,
    T: DeserializeOwned,
{
    let path = path.as_ref();
    let str = str.as_ref();

    match F::load::<T>(path) {
        Ok(v) => Ok(v),

        Err(CodecError::Io(ref e)) if e.kind() == std::io::ErrorKind::NotFound => {
            file::atomic::create_new(path, |file| -> io::Result<()> {
                file.write_all(str.as_bytes())?;
                file.sync_all()?;
                Ok(())
            })?;

            F::parse_str::<T>(str).map_err(CodecError::Deserialize)
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
    fn creates_file_when_missing_and_parses_str() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("demo.json");
        let seed = r#"{ "id": 1, "name": "alpha" }"#;

        let got: Demo = load_or_write_str::<Json, _>(&path, seed).expect("should parse");
        assert_eq!(got, Demo { id: 1, name: "alpha".into() });

        let on_disk = fs::read_to_string(&path).unwrap();
        assert_eq!(on_disk, seed);
    }

    #[test]
    fn loads_existing_file_and_does_not_overwrite_with_new_str() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("demo.json");

        let existing = r#"{ "id": 2, "name": "beta" }"#;
        fs::write(&path, existing).unwrap();

        let seed = r#"{ "id": 9, "name": "SHOULD_NOT_WRITE" }"#;

        let got: Demo = load_or_write_str::<Json, _>(&path, seed).expect("should load existing");
        assert_eq!(got, Demo { id: 2, name: "beta".into() });

        // Ensure file content was not changed
        let on_disk = fs::read_to_string(&path).unwrap();
        assert_eq!(on_disk, existing);
    }

    #[test]
    fn existing_file_with_invalid_content_returns_deserialize_error_and_keeps_file() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("demo.json");

        let invalid_existing = "{ id: 3, }"; // invalid JSON
        fs::write(&path, invalid_existing).unwrap();

        let seed = r#"{ "id": 4, "name": "gamma" }"#; // should be ignored
        let err = load_or_write_str::<Json, Demo>(&path, seed).unwrap_err();

        assert!(matches!(err, CodecError::Deserialize(DeserializeError::Json(_))));

        // File should remain unchanged (no overwrite)
        let on_disk = fs::read_to_string(&path).unwrap();
        assert_eq!(on_disk, invalid_existing);
        let _ = err;
    }

    #[test]
    fn missing_file_with_invalid_seed_writes_file_then_returns_deserialize_error() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("demo.json");

        let invalid_json = "{ id: 5, }";
        let err = load_or_write_str::<Json, Demo>(&path, invalid_json).unwrap_err();

        // File should have been created with the invalid content
        let on_disk = fs::read_to_string(&path).unwrap();
        assert_eq!(on_disk, invalid_json);

        assert!(matches!(err, CodecError::Deserialize(DeserializeError::Json(_))));
    }
}
