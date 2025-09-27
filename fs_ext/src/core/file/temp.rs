use {
    crate::TempFile,
    std::{io, path::Path},
};

#[cfg_attr(test, fs_ext_test_macros::fs_test(rejects_missing_path, rejects_file, existing_dir_ok))]
pub fn temp_in(dir: impl AsRef<Path>) -> io::Result<TempFile> {
    TempFile::in_dir(dir)
}

pub fn temp() -> io::Result<TempFile> {
    TempFile::new()
}
