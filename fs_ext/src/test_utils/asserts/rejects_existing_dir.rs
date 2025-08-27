use std::{fs, io, path::Path};

pub fn assert_fn_rejects_existing_dir<F, T>(f: F) -> io::Result<()>
where
    F: FnOnce(&Path) -> io::Result<T>,
{
    let tmp = tempfile::tempdir()?;
    let dir = tmp.path().join("exists_dir");
    fs::create_dir(&dir)?;

    match f(&dir) {
        Ok(_) => panic!(
            "expected error when passing an EXISTING directory path ({}), but got Ok(…)",
            dir.display()
        ),
        Err(err) => {
            assert!(
                matches!(err.kind(), io::ErrorKind::AlreadyExists | io::ErrorKind::InvalidInput),
                "expected AlreadyExists or InvalidInput, got: {:?} (err: {err})",
                err.kind()
            );
            let msg = err.to_string();
            assert!(
                msg.contains(&dir.display().to_string()),
                "error message should include path.\npath: {}\nerror: {}",
                dir.display(),
                msg
            );
            Ok(())
        }
    }
}
