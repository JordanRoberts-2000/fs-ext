use {
    crate::{InferredPathType, utils::infer_path_type},
    std::{fs, io, path::Path},
    tokio::fs as t_fs,
};

pub fn create_file_or_dir(path: &Path) -> io::Result<()> {
    match infer_path_type(path) {
        InferredPathType::File => {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }

            fs::File::create(path)?;
            Ok(())
        }
        InferredPathType::Dir => fs::create_dir_all(path),
    }
}

pub async fn create_file_or_dir_async(path: &Path) -> io::Result<()> {
    match infer_path_type(path) {
        InferredPathType::File => {
            if let Some(parent) = path.parent() {
                t_fs::create_dir_all(parent).await?;
            }

            t_fs::File::create(path).await?;
            Ok(())
        }
        InferredPathType::Dir => t_fs::create_dir_all(path).await,
    }
}
