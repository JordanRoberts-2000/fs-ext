use {
    crate::{fsx, tokio::utils::asyncify},
    std::{io, path::Path, time::SystemTime},
};

pub async fn last_modified(path: impl AsRef<Path>) -> io::Result<SystemTime> {
    let path = path.as_ref().to_owned();
    asyncify(move || fsx::file::meta::last_modified(path)).await
}

#[cfg(test)]
mod tests {
    use {
        super::last_modified,
        std::{io, time::SystemTime},
    };

    #[tokio::test]
    async fn smoke_last_modified() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let file_path = dir.path().join("file.txt");

        // create a file
        std::fs::write(&file_path, b"hello")?;

        // should return a modification time
        let modified_time: SystemTime = last_modified(&file_path).await?;
        assert!(modified_time <= SystemTime::now());

        Ok(())
    }
}
