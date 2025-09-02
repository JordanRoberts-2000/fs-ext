use std::{
    fs::OpenOptions,
    io::{self, Write},
    path::Path,
};

pub fn append(path: impl AsRef<Path>, content: impl AsRef<[u8]>) -> io::Result<()> {
    let mut file = OpenOptions::new().append(true).open(path)?;

    file.write_all(content.as_ref())?;
    file.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use {super::*, std::fs, tempfile::tempdir};

    #[test]
    fn appends_to_existing_file_without_truncation() -> io::Result<()> {
        let dir = tempdir()?;
        let path = dir.path().join("log.txt");

        fs::write(&path, "one")?;
        append(&path, "two")?;
        append(&path, "three")?;

        let s = fs::read_to_string(&path)?;
        assert_eq!(s, "onetwothree");
        Ok(())
    }

    #[test]
    fn empty_content_is_noop() -> io::Result<()> {
        let dir = tempdir()?;
        let path = dir.path().join("noop.txt");

        fs::write(&path, "keep-me")?;
        append(&path, &[] as &[u8])?;

        let s = fs::read_to_string(&path)?;
        assert_eq!(s, "keep-me");
        Ok(())
    }

    #[test]
    fn does_not_create_missing_file() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("missing.txt");

        let err = append(&path, "data").unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::NotFound);

        assert!(!path.exists());
    }

    #[test]
    fn error_when_target_is_directory() {
        let dir = tempdir().unwrap();
        let target = dir.path();

        let err = append(target, "data").unwrap_err();
        assert!(
            matches!(
                err.kind(),
                io::ErrorKind::IsADirectory
                    | io::ErrorKind::PermissionDenied
                    | io::ErrorKind::Other
            ),
            "unexpected error kind: {:?}",
            err.kind()
        );
    }
}
