use {
    fs_ext::{load, save},
    std::path::PathBuf,
};

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
struct Person {
    name: String,
    age: u8,
}

#[test]
fn save_then_load_round_trip_toml_json_yaml() {
    let td = tempfile::tempdir().unwrap();
    let ada = Person { name: "Ada".into(), age: 37 };

    // --- TOML ---
    let p_toml: PathBuf = td.path().join("person.toml");
    save!(&p_toml, &ada).expect("save toml");
    let got: Person = load!(&p_toml).expect("load toml");
    assert_eq!(got, ada);

    // --- JSON ---
    let p_json: PathBuf = td.path().join("person.json");
    save!(&p_json, &ada).expect("save json");
    let got: Person = load!(&p_json).expect("load json");
    assert_eq!(got, ada);

    // --- YAML ---
    let p_yaml: PathBuf = td.path().join("person.yaml");
    save!(&p_yaml, &ada).expect("save yaml");
    let got: Person = load!(&p_yaml).expect("load yaml");
    assert_eq!(got, ada);
}
