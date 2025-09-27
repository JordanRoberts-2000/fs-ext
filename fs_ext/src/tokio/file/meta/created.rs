use {
    crate::{file, tokio::utils::asyncify},
    std::{io, path::Path, time::SystemTime},
};

pub async fn created(path: impl AsRef<Path>) -> io::Result<SystemTime> {
    let path = path.as_ref().to_owned();
    asyncify(move || file::meta::created(path)).await
}

#[cfg(test)]
mod tests {
    use {
        super::created,
        std::{io, time::SystemTime},
    };

    #[tokio::test]
    async fn smoke_created() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let file_path = dir.path().join("file.txt");

        // create a file
        std::fs::write(&file_path, b"hello")?;

        // should return a creation time
        let created_time: SystemTime = created(&file_path).await?;
        assert!(created_time <= SystemTime::now());

        Ok(())
    }
}
