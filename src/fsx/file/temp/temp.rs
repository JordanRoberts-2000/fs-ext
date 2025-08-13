use {crate::fsx::TempFile, std::io};

pub fn temp() -> io::Result<TempFile> {
    TempFile::new()
}
