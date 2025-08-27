use std::{fs, io, path::Path};

pub fn assert_fn_rejects_file_path<F, T>(f: F) -> io::Result<()>
where
    F: FnOnce(&Path) -> io::Result<T>,
{
    let tmp = tempfile::tempdir()?;
    let path = tmp.path().join("data.bin");
    fs::write(&path, b"x")?;

    match f(&path) {
        Ok(_) => panic!(
            "expected error (InvalidInput) when passing a NON-DIRECTORY path to a DIR fn: {}",
            path.display()
        ),
        Err(err) => {
            assert!(
                matches!(
                    err.kind(),
                    io::ErrorKind::InvalidInput
                        | io::ErrorKind::NotADirectory
                        | io::ErrorKind::AlreadyExists
                ),
                "expected InvalidInput or NotADirectory, got: {:?} (err: {err})",
                err.kind(),
            );

            let msg = err.to_string();
            assert!(
                msg.contains(&path.display().to_string()),
                "error message should include path.\npath: {}\nerror: {}",
                path.display(),
                msg
            );
        }
    }

    Ok(())
}
