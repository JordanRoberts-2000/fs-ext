use {
    fs_ext::file,
    std::{fs, io::Read, path::PathBuf},
};

#[test]
fn file_macro_creates_empty_file_and_returns_handle() {
    let td = tempfile::tempdir().unwrap();
    let p: PathBuf = td.path().join("empty.txt");

    let mut fh = file!(&p).expect("file! should create/ensure a file and return File");

    // File exists and is empty initially
    let meta = fs::metadata(&p).unwrap();
    assert!(meta.is_file());
    assert_eq!(meta.len(), 0);

    // The returned handle should be writable
    use std::io::Write as _;
    fh.write_all(b"abc").unwrap();
    fh.flush().unwrap();

    let mut s = String::new();
    fs::File::open(&p).unwrap().read_to_string(&mut s).unwrap();
    assert_eq!(s, "abc");
}

#[test]
fn file_macro_overwrites_and_writes_str() {
    let td = tempfile::tempdir().unwrap();
    let p = td.path().join("note.txt");

    fs::write(&p, "OLD").unwrap();

    let _fh = file!(&p, "hello world").expect("file! should overwrite and write content");

    let got = fs::read_to_string(&p).unwrap();
    assert_eq!(got, "hello world");
}

#[test]
fn file_macro_writes_bytes_and_vec_u8() {
    let td = tempfile::tempdir().unwrap();
    let p1 = td.path().join("bytes.bin");
    let p2 = td.path().join("vec.bin");

    // &[u8]
    let _ = file!(&p1, b"\x00\x01\x02\x03").unwrap();
    let got = fs::read(&p1).unwrap();
    assert_eq!(got, vec![0, 1, 2, 3]);

    // Vec<u8>
    let data = vec![10u8, 20, 30, 40];
    let _ = file!(&p2, data.clone()).unwrap();
    let got = fs::read(&p2).unwrap();
    assert_eq!(got, data);
}
