use std::{fs, io, path::Path};

pub fn assert_fn_rejects_dir_path<F, T>(f: F) -> io::Result<()>
where
    F: FnOnce(&Path) -> io::Result<T>,
{
    let tmp = tempfile::tempdir()?;
    let dir_path = tmp.path().join("some_dir");
    fs::create_dir(&dir_path)?;

    match f(&dir_path) {
        Ok(_) => panic!(
            "expected error when passing a DIRECTORY path to a FILE fn: {}",
            dir_path.display()
        ),
        Err(err) => {
            assert!(
                matches!(
                    err.kind(),
                    io::ErrorKind::InvalidInput
                        | io::ErrorKind::IsADirectory
                        | io::ErrorKind::AlreadyExists
                ),
                "expected InvalidInput, IsADirectory, or AlreadyExists, got: {:?} (err: {err})",
                err.kind(),
            );

            let msg = err.to_string();
            assert!(
                msg.contains(&dir_path.display().to_string()),
                "error message should include path.\npath: {}\nerror: {}",
                dir_path.display(),
                msg
            );
        }
    }

    Ok(())
}
