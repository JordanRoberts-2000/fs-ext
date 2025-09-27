use {
    crate::{IoResultExt, dir},
    std::{fs, io, path::Path},
};

#[cfg_attr(test, fs_ext_test_macros::fs_test(rejects_missing_path, rejects_file, existing_dir_ok))]
pub fn clear(path: impl AsRef<Path>) -> io::Result<()> {
    _clear(path.as_ref())
}

fn _clear(path: &Path) -> io::Result<()> {
    dir::assert_exists(path)?;

    let entries = fs::read_dir(path).with_path_context("failed to read directory", path)?;

    for entry in entries {
        let entry = entry.with_path_context("failed to read an entry in directory", path)?;

        let child = entry.path();
        let ft = entry.file_type().with_path_context("failed to read file type", &child)?;

        if ft.is_dir() {
            fs::remove_dir_all(&child)
                .with_path_context("failed to remove subdirectory", &child)?;
        } else {
            fs::remove_file(&child).with_path_context("failed to remove file", &child)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use {
        super::clear,
        std::{fs, io},
        tempfile::tempdir,
    };

    #[test]
    fn empty_dir_is_noop_and_remains() -> io::Result<()> {
        let d = tempdir()?;
        let root = d.path();

        clear(root)?;
        assert!(root.exists());
        assert!(fs::read_dir(root)?.next().is_none());
        Ok(())
    }

    #[test]
    fn removes_files_and_subdirs_but_keeps_root() -> io::Result<()> {
        let d = tempdir()?;
        let root = d.path();

        fs::create_dir_all(root.join("sub/nest"))?;
        fs::write(root.join("a.txt"), b"hello")?;
        fs::write(root.join("sub").join("b.bin"), [1, 2, 3])?;
        fs::write(root.join("sub/nest").join("c"), [])?;

        clear(root)?;
        assert!(root.exists());
        assert!(fs::read_dir(root)?.next().is_none());
        Ok(())
    }

    #[test]
    fn idempotent_clear() -> io::Result<()> {
        let d = tempdir()?;
        let root = d.path();

        fs::create_dir_all(root.join("sub"))?;
        fs::write(root.join("sub").join("x"), b"x")?;

        clear(root)?;
        // run again on already-empty dir
        clear(root)?;
        assert!(root.exists());
        assert!(fs::read_dir(root)?.next().is_none());
        Ok(())
    }
}
