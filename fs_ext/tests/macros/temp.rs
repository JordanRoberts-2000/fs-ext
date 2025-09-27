use {
    fs_ext::{tempdir, tempfile},
    std::{fs, io::Read, path::PathBuf},
};

#[test]
fn tempfile_default_is_writable_and_readable() {
    let mut tf = tempfile!().expect("tempfile!() should create a temp file");
    // Write via returned handle
    use std::io::Write as _;
    tf.as_file_mut().write_all(b"abc").unwrap();
    tf.as_file_mut().flush().unwrap();

    // Read via its path
    let mut s = String::new();
    fs::File::open(tf.path()).unwrap().read_to_string(&mut s).unwrap();
    assert_eq!(s, "abc");
}

#[test]
fn tempfile_in_dir_creates_under_that_dir() {
    // Make a sandbox parent directory
    let sandbox = tempfile::tempdir().unwrap();
    let parent: PathBuf = sandbox.path().join("parent");
    fs::create_dir_all(&parent).unwrap();

    let tf = tempfile!(&parent).expect("tempfile!(dir) should create a file inside the dir");
    assert!(tf.path().starts_with(&parent), "temp file not placed under requested parent");

    // Ensure the file is a file
    let meta = fs::metadata(tf.path()).unwrap();
    assert!(meta.is_file());
}

#[test]
fn tempfile_in_dir_with_content_writes_and_syncs() {
    let sandbox = tempfile::tempdir().unwrap();
    let parent = sandbox.path();

    // &str content
    let tf = tempfile!(parent, "hello world").expect("tempfile!(dir, content) should succeed");
    let got = fs::read_to_string(tf.path()).unwrap();
    assert_eq!(got, "hello world");

    // &[u8] content (AsRef<[u8]>)
    let tf2 = tempfile!(parent, b"\x00\x01\x02").unwrap();
    let got = fs::read(tf2.path()).unwrap();
    assert_eq!(got, vec![0, 1, 2]);

    // Vec<u8] content
    let data = vec![9u8, 8, 7];
    let tf3 = tempfile!(parent, data.clone()).unwrap();
    let got = fs::read(tf3.path()).unwrap();
    assert_eq!(got, data);
}

#[test]
fn tempdir_default_creates_directory() {
    let td = tempdir!().expect("tempdir!() should create a temp directory");
    let p = td.path();
    assert!(p.is_dir(), "returned path is not a directory");

    // We can create a file inside it
    let f = p.join("note.txt");
    fs::write(&f, "ok").unwrap();
    assert_eq!(fs::read_to_string(&f).unwrap(), "ok");
}

#[test]
fn tempdir_in_parent_places_dir_under_parent() {
    let sandbox = tempfile::tempdir().unwrap();
    let parent = sandbox.path();

    let td = tempdir!(parent).expect("tempdir!(parent) should succeed");
    let p = td.path();

    assert!(p.starts_with(parent), "tempdir not created under the requested parent");
    assert!(p.is_dir());
}
