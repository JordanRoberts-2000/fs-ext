use fs_ext::dir;

#[test]
fn creates_nested_dirs() {
    let td = tempfile::tempdir().unwrap();
    let target = td.path().join("a/b/c");

    dir!(&target).unwrap();

    assert!(target.is_dir());
}
