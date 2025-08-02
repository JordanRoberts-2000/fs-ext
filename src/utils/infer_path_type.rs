use {crate::InferredPathType, std::path::Path};

pub fn infer_path_type(path: &Path) -> InferredPathType {
    let path_str = path.to_string_lossy();
    if path_str.ends_with('/') || path_str.ends_with('\\') {
        return InferredPathType::Dir;
    }

    if path.extension().is_some() { InferredPathType::File } else { InferredPathType::Dir }
}
