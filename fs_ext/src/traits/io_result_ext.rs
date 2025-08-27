use std::{fmt::Display, io, path::Path};

pub trait IoResultExt<T> {
    fn with_path_context(self, action: impl Display, path: impl AsRef<Path>) -> io::Result<T>;
    fn with_paths_context(
        self, action: impl Display, src: impl AsRef<Path>, dst: impl AsRef<Path>,
    ) -> io::Result<T>;
}

impl<T> IoResultExt<T> for io::Result<T> {
    fn with_path_context(self, action: impl Display, path: impl AsRef<Path>) -> io::Result<T> {
        self.map_err(|e| {
            io::Error::new(e.kind(), format!("{action} '{}': {e}", path.as_ref().display()))
        })
    }

    fn with_paths_context(
        self, action: impl Display, src: impl AsRef<Path>, dst: impl AsRef<Path>,
    ) -> io::Result<T> {
        self.map_err(|e| {
            io::Error::new(
                e.kind(),
                format!(
                    "{action} '{}' -> '{}': {e}",
                    src.as_ref().display(),
                    dst.as_ref().display()
                ),
            )
        })
    }
}
