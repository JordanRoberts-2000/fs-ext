use {crate::tokio::fsx::TempFile, std::io};

pub async fn temp() -> io::Result<TempFile> {
    TempFile::new().await
}
