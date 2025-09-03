use {
    crate::{IoResultExt, PathKind},
    std::{fs, io, path::Path},
};

pub trait PathExt {
    fn is_dir_strict(&self) -> io::Result<bool>;
    fn is_file_strict(&self) -> io::Result<bool>;
    fn kind(&self) -> io::Result<PathKind>;
    fn assert_dir(&self) -> io::Result<()>;
    fn assert_file(&self) -> io::Result<()>;
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
}
