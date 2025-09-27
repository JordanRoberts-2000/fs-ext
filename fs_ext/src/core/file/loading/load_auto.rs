use {
    crate::{
        CodecError, Format,
        formats::{Json, Toml, Yaml},
    },
    serde::de::DeserializeOwned,
    std::{io, path::Path},
};

pub fn load_auto<T>(path: impl AsRef<Path>) -> Result<T, CodecError>
where
    T: DeserializeOwned,
{
    let path = path.as_ref();

    let ext =
        path.extension().and_then(|s| s.to_str()).map(|s| s.to_ascii_lowercase()).ok_or_else(
            || {
                CodecError::from(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("path '{}' has no extension", path.display()),
                ))
            },
        )?;

    match ext.as_str() {
        "toml" => Toml::load::<T>(path),
        "json" => Json::load::<T>(path),
        "yaml" | "yml" => Yaml::load::<T>(path),

        other => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "unsupported extension '{other}' for '{}'; expected one of: toml, json, yaml/yml",
                path.display()
            ),
        )
        .into()),
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
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
    fn loads_json() {
        let dir = tempdir().unwrap();
        let p = dir.path().join("d.json");
        fs::write(&p, r#"{ "id": 1, "name": "alpha" }"#).unwrap();

        let got: Demo = load_auto(&p).expect("json load");
        assert_eq!(got, Demo { id: 1, name: "alpha".into() });
    }

    #[test]
    fn loads_toml() {
        let dir = tempdir().unwrap();
        let p = dir.path().join("d.toml");
        fs::write(&p, "id = 2\nname = \"beta\"\n").unwrap();

        let got: Demo = load_auto(&p).expect("toml load");
        assert_eq!(got, Demo { id: 2, name: "beta".into() });
    }

    #[test]
    fn loads_yaml() {
        let dir = tempdir().unwrap();
        let p = dir.path().join("d.yaml");
        fs::write(&p, "id: 3\nname: gamma\n").unwrap();

        let got: Demo = load_auto(&p).expect("yaml load");
        assert_eq!(got, Demo { id: 3, name: "gamma".into() });
    }

    #[test]
    fn loads_yml_alias() {
        let dir = tempdir().unwrap();
        let p = dir.path().join("d.yml");
        fs::write(&p, "id: 4\nname: delta\n").unwrap();

        let got: Demo = load_auto(&p).expect("yml load");
        assert_eq!(got, Demo { id: 4, name: "delta".into() });
    }

    #[test]
    fn extension_is_case_insensitive() {
        let dir = tempdir().unwrap();

        // JSON uppercased
        let pj = dir.path().join("X.JSON");
        fs::write(&pj, r#"{ "id": 5, "name": "eps" }"#).unwrap();
        let j: Demo = load_auto(&pj).expect("JSON load");
        assert_eq!(j, Demo { id: 5, name: "eps".into() });

        // YAML mixed case
        let py = dir.path().join("Y.YaMl");
        fs::write(&py, "id: 6\nname: zeta\n").unwrap();
        let y: Demo = load_auto(&py).expect("YaMl load");
        assert_eq!(y, Demo { id: 6, name: "zeta".into() });
    }

    #[test]
    fn error_when_no_extension() {
        let dir = tempdir().unwrap();
        let p = dir.path().join("noext");
        fs::write(&p, "id: 7\nname: eta\n").unwrap();

        let err = load_auto::<Demo>(&p).unwrap_err();
        assert!(matches!(err, CodecError::Io(_)));
    }

    #[test]
    fn error_on_unsupported_extension() {
        let dir = tempdir().unwrap();
        let p = dir.path().join("bad.ini");
        fs::write(&p, "id=8\nname=theta\n").unwrap();

        let err = load_auto::<Demo>(&p).unwrap_err();
        assert!(matches!(err, CodecError::Io(_)));
    }
}
