use std::{fs, io, path::Path};

pub fn assert_fn_rejects_existing_file<F, T>(f: F) -> io::Result<()>
where
    F: FnOnce(&Path) -> io::Result<T>,
{
    let tmp = tempfile::tempdir()?;
    let path = tmp.path().join("exists.bin");
    fs::write(&path, b"x")?;

    match f(&path) {
        Ok(_) => panic!(
            "expected error when passing an EXISTING path ({}), but got Ok(…)",
            path.display()
        ),
        Err(err) => {
            assert!(
                matches!(err.kind(), io::ErrorKind::AlreadyExists | io::ErrorKind::InvalidInput),
                "expected AlreadyExists or InvalidInput, got: {:?} (err: {err})",
                err.kind()
            );
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
