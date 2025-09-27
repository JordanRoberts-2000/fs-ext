use {
    fs_ext::load,
    serde::Deserialize,
    std::{fs, path::PathBuf},
};

#[derive(Debug, Deserialize, PartialEq)]
struct Person {
    name: String,
    age: u8,
}

#[test]
fn load_toml_json_yaml() {
    let td = tempfile::tempdir().unwrap();

    // --- TOML ---
    let toml_p: PathBuf = td.path().join("person.toml");
    fs::write(
        &toml_p,
        r#"
            name = "Ada"
            age = 37
        "#,
    )
    .unwrap();
    let p: Person = load!(&toml_p).expect("toml load should work");
    assert_eq!(p, Person { name: "Ada".into(), age: 37 });

    // --- JSON ---
    let json_p: PathBuf = td.path().join("person.json");
    fs::write(&json_p, r#"{ "name": "Ada", "age": 37 }"#).unwrap();
    let p: Person = load!(&json_p).expect("json load should work");
    assert_eq!(p, Person { name: "Ada".into(), age: 37 });

    // --- YAML ---
    let yaml_p: PathBuf = td.path().join("person.yaml");
    fs::write(&yaml_p, "name: Ada\nage: 37\n").unwrap();
    let p: Person = load!(&yaml_p).expect("yaml load should work");
    assert_eq!(p, Person { name: "Ada".into(), age: 37 });
}
