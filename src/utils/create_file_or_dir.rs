use {
    crate::{InferredPathType, utils::infer_path_type},
    std::{fs, io, path::Path},
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
