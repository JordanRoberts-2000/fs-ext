use {
    crate::{IoResultExt, PathKind},
    std::{
        fs, io,
        path::{Path, PathBuf},
    },
};

pub trait PathExt {
    fn is_dir_strict(&self) -> io::Result<bool>;
    fn is_file_strict(&self) -> io::Result<bool>;
    fn kind(&self) -> io::Result<PathKind>;
    fn assert_dir(&self) -> io::Result<()>;
    fn assert_file(&self) -> io::Result<()>;
    fn parent_or_current(&self) -> PathBuf;
    fn utf8_stem(&self) -> io::Result<&str>;
}

impl PathExt for Path {
    fn is_dir_strict(&self) -> io::Result<bool> {
        let meta = fs::metadata(self).with_path_context("could not read metadata", self)?;
        Ok(meta.is_dir())
    }

    fn assert_dir(&self) -> io::Result<()> {
        if self.is_dir_strict()? {
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Path '{}' is not a directory", self.display()),
            ))
        }
    }

    fn is_file_strict(&self) -> io::Result<bool> {
        let meta = fs::metadata(self).with_path_context("could not read metadata", self)?;
        Ok(meta.is_file())
    }

    fn assert_file(&self) -> io::Result<()> {
        if self.is_file_strict()? {
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Path '{}' is not a file", self.display()),
            ))
        }
    }

    fn kind(&self) -> io::Result<PathKind> {
        let meta = fs::metadata(self)?;
        Ok(if meta.is_dir() {
            PathKind::Dir
        } else if meta.is_file() {
            PathKind::File
        } else if meta.is_symlink() {
            PathKind::SymLink
        } else {
            PathKind::Other
        })
    }

    fn parent_or_current(&self) -> PathBuf {
        self.parent()
            .filter(|p| !p.as_os_str().is_empty())
            .map_or_else(|| PathBuf::from("."), |p| p.to_path_buf())
    }

    fn utf8_stem(&self) -> io::Result<&str> {
        self.file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "no UTF-8 file stem"))
    }
}

#[cfg(test)]
mod tests {
    use {
        super::PathExt,
        std::{
            io,
            path::{Path, PathBuf},
        },
    };

    // ---------- parent_or_current ----------

    #[test]
    fn parent_or_current_regular_relative() {
        let p = Path::new("a/b/c.txt");
        assert_eq!(p.parent_or_current(), PathBuf::from("a/b"));
    }

    #[test]
    fn parent_or_current_single_component() {
        let p = Path::new("file.txt");
        assert_eq!(p.parent_or_current(), PathBuf::from("."));
    }

    #[test]
    fn parent_or_current_empty_path() {
        let p = Path::new("");
        assert_eq!(p.parent_or_current(), PathBuf::from("."));
    }

    #[test]
    fn parent_or_current_dot_current() {
        let p = Path::new(".");
        assert_eq!(p.parent_or_current(), PathBuf::from("."));
    }

    // ---------- utf8_stem (&str) ----------

    #[test]
    fn utf8_stem_simple_file() {
        let p = Path::new("foo.txt");
        assert_eq!(p.utf8_stem().unwrap(), "foo");
    }

    #[test]
    fn utf8_stem_no_extension() {
        let p = Path::new("foo");
        assert_eq!(p.utf8_stem().unwrap(), "foo");
    }

    #[test]
    fn utf8_stem_dotfile_no_extension() {
        let p = Path::new(".bashrc");
        assert_eq!(p.utf8_stem().unwrap(), ".bashrc");
    }

    #[test]
    fn utf8_stem_multi_dots() {
        let p = Path::new("archive.tar.gz");
        assert_eq!(p.utf8_stem().unwrap(), "archive.tar");
    }

    #[test]
    fn utf8_stem_trailing_dot_empty_ext() {
        let p = Path::new("name.");
        assert_eq!(p.utf8_stem().unwrap(), "name");
    }

    #[test]
    fn utf8_stem_path_without_final_component() {
        #[cfg(unix)]
        {
            let p = Path::new("/");
            let err = p.utf8_stem().unwrap_err();
            assert_eq!(err.kind(), io::ErrorKind::InvalidData);
        }
        #[cfg(windows)]
        {
            let p = Path::new(r"C:\");
            let err = p.utf8_stem().unwrap_err();
            assert_eq!(err.kind(), io::ErrorKind::InvalidData);
        }
    }

    #[cfg(unix)]
    #[test]
    fn utf8_stem_non_utf8_fails() {
        use std::{ffi::OsStr, os::unix::ffi::OsStrExt};

        // "fo\x80o.txt" (0x80 makes it invalid UTF-8)
        let raw = b"fo\x80o.txt";
        let p = Path::new(OsStr::from_bytes(raw));
        let err = p.utf8_stem().unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidData);
    }
}
