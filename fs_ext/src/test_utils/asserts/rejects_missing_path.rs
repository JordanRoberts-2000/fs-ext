use {
    std::{io, path::Path},
    tempfile::tempdir,
};

pub fn assert_fn_rejects_missing_path<F, T>(f: F) -> io::Result<()>
where
    F: FnOnce(&Path) -> io::Result<T>,
{
    let tmp = tempdir()?;
    let path = tmp.path().join("nope").join("again").join("missing.txt");

    match f(&path) {
        Ok(_) => panic!("expected error when passing a MISSING path"),
        Err(err) => {
            assert_eq!(err.kind(), io::ErrorKind::NotFound, "err: {err}");
            let msg = err.to_string();
            assert!(
                msg.contains(&path.display().to_string()),
                "error message should include path.\npath: {}\nerror: {}",
                path.display(),
                msg
            );
            Ok(())
        }
    }
}
