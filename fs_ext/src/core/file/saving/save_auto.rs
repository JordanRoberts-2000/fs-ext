use {
    crate::{
        CodecError, Format,
        formats::{Json, Toml, Yaml},
    },
    serde::Serialize,
    std::{io, path::Path},
};

pub fn save_auto<T>(path: impl AsRef<Path>, model: &T) -> Result<(), CodecError>
where
    T: Serialize,
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
        "toml" => Toml::save(path, model),
        "json" => Json::save(path, model),
        "yaml" | "yml" => Yaml::save(path, model),

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
        crate::formats::{Json, Toml, Yaml},
        serde::{Deserialize, Serialize},
        tempfile::tempdir,
    };

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct Demo {
        id: u32,
        name: String,
    }

    #[test]
    fn saves_json_then_loads_back() {
        let dir = tempdir().unwrap();
        let p = dir.path().join("d.json");
        let v = Demo { id: 1, name: "alpha".into() };

        save_auto(&p, &v).expect("save json");
        let got: Demo = Json::load(&p).expect("load json");
        assert_eq!(got, v);
    }

    #[test]
    fn saves_toml_then_loads_back() {
        let dir = tempdir().unwrap();
        let p = dir.path().join("d.toml");
        let v = Demo { id: 2, name: "beta".into() };

        save_auto(&p, &v).expect("save toml");
        let got: Demo = Toml::load(&p).expect("load toml");
        assert_eq!(got, v);
    }

    #[test]
    fn saves_yaml_then_loads_back() {
        let dir = tempdir().unwrap();
        let p = dir.path().join("d.yaml");
        let v = Demo { id: 3, name: "gamma".into() };

        save_auto(&p, &v).expect("save yaml");
        let got: Demo = Yaml::load(&p).expect("load yaml");
        assert_eq!(got, v);
    }

    #[test]
    fn saves_yml_alias_then_loads_back() {
        let dir = tempdir().unwrap();
        let p = dir.path().join("d.yml");
        let v = Demo { id: 4, name: "delta".into() };

        save_auto(&p, &v).expect("save yml");
        let got: Demo = Yaml::load(&p).expect("load yaml");
        assert_eq!(got, v);
    }

    #[test]
    fn extension_is_case_insensitive() {
        let dir = tempdir().unwrap();

        // JSON uppercased
        let pj = dir.path().join("X.JSON");
        let vj = Demo { id: 5, name: "eps".into() };
        save_auto(&pj, &vj).expect("save JSON");
        let got_j: Demo = Json::load(&pj).unwrap();
        assert_eq!(got_j, vj);

        // YAML mixed case
        let py = dir.path().join("Y.YaMl");
        let vy = Demo { id: 6, name: "zeta".into() };
        save_auto(&py, &vy).expect("save YaMl");
        let got_y: Demo = Yaml::load(&py).unwrap();
        assert_eq!(got_y, vy);

        // TOML weird case
        let pt = dir.path().join("Z.ToMl");
        let vt = Demo { id: 7, name: "eta".into() };
        save_auto(&pt, &vt).expect("save ToMl");
        let got_t: Demo = Toml::load(&pt).unwrap();
        assert_eq!(got_t, vt);
    }

    #[test]
    fn error_when_no_extension() {
        let dir = tempdir().unwrap();
        let p = dir.path().join("noext");
        let v = Demo { id: 8, name: "theta".into() };

        let err = save_auto(&p, &v).unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("has no extension"), "{msg}");
    }

    #[test]
    fn error_on_unsupported_extension() {
        let dir = tempdir().unwrap();
        let p = dir.path().join("bad.ini");
        let v = Demo { id: 9, name: "iota".into() };

        let err = save_auto(&p, &v).unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("unsupported extension 'ini'"), "{msg}");
    }

    #[test]
    fn save_failure_propagates_io_error() {
        let dir = tempdir().unwrap();
        let bad = dir.path().join("no_such_dir").join("d.json");
        let v = Demo { id: 10, name: "kappa".into() };

        let err = save_auto(&bad, &v).unwrap_err();
        assert!(matches!(err, CodecError::Io(_)));
    }
}
