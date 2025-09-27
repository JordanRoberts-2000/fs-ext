use {
    crate::{IoResultExt, dir},
    std::{fs, io, path::Path},
};

pub fn copy_dir_contents(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    _copy_dir_contents(src.as_ref(), dst.as_ref())
}

fn _copy_dir_contents(src: &Path, dst: &Path) -> io::Result<()> {
    dir::assert_exists(src)?;
    dir::assert_exists(dst)?;

    let entries = fs::read_dir(src).with_path_context("failed to read source directory", src)?;

    for entry_res in entries {
        let entry =
            entry_res.with_path_context("failed to read an entry in source directory", src)?;
        let entry_path = entry.path();

        let name_os = entry.file_name();
        let dst_path = dst.join(Path::new(&name_os));

        let ft = entry.file_type().with_path_context("failed to read file type", &entry_path)?;

        if ft.is_dir() {
            dir::create_new(&dst_path)?;
            _copy_dir_contents(&entry_path, &dst_path)?;
        } else if ft.is_file() {
            fs::copy(&entry_path, &dst_path).with_paths_context(
                "failed to copy",
                entry_path,
                dst_path,
            )?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use {
        super::copy_dir_contents,
        std::{fs, io},
        tempfile::tempdir,
    };

    #[test]
    fn copies_flat_files() -> io::Result<()> {
        let src = tempdir()?;
        let dst = tempdir()?;

        fs::write(src.path().join("a.txt"), b"hello")?;
        fs::write(src.path().join("b.bin"), [1u8, 2, 3])?;

        copy_dir_contents(src.path(), dst.path())?;

        assert_eq!(fs::read(dst.path().join("a.txt"))?, b"hello");
        assert_eq!(fs::read(dst.path().join("b.bin"))?, vec![1, 2, 3]);
        Ok(())
    }

    #[test]
    fn recurses_into_subdirectories() -> io::Result<()> {
        let src = tempdir()?;
        let dst = tempdir()?;

        fs::create_dir_all(src.path().join("x/y"))?;
        fs::write(src.path().join("x").join("one"), b"1")?;
        fs::write(src.path().join("x/y").join("two"), b"22")?;

        copy_dir_contents(src.path(), dst.path())?;

        assert_eq!(fs::read(dst.path().join("x/one"))?, b"1");
        assert_eq!(fs::read(dst.path().join("x/y/two"))?, b"22");
        Ok(())
    }

    #[test]
    fn overwrites_existing_file_in_destination() -> io::Result<()> {
        let src = tempdir()?;
        let dst = tempdir()?;

        fs::write(src.path().join("same.txt"), b"SRC")?;
        fs::write(dst.path().join("same.txt"), b"DST")?;

        copy_dir_contents(src.path(), dst.path())?;
        assert_eq!(fs::read(dst.path().join("same.txt"))?, b"SRC");
        Ok(())
    }

    #[test]
    fn errors_when_src_is_a_file() -> io::Result<()> {
        let tmp = tempdir()?;
        let src_file = tmp.path().join("not_a_dir");
        fs::write(&src_file, b"x")?;
        let dst_dir = tempdir()?;

        let err = copy_dir_contents(&src_file, dst_dir.path()).unwrap_err();
        assert!(err.kind() != io::ErrorKind::Other || err.kind() != io::ErrorKind::NotFound);
        Ok(())
    }

    #[test]
    fn errors_when_dst_is_a_file() -> io::Result<()> {
        let src = tempdir()?;
        fs::write(src.path().join("a.txt"), b"x")?;

        let tmp = tempdir()?;
        let dst_file = tmp.path().join("not_a_dir");
        fs::write(&dst_file, b"")?;

        assert!(copy_dir_contents(src.path(), &dst_file).is_err());
        Ok(())
    }

    #[test]
    fn errors_when_conflicting_subdir_already_exists_in_dst() -> io::Result<()> {
        let src = tempdir()?;
        fs::create_dir_all(src.path().join("sub"))?;
        fs::write(src.path().join("sub").join("file"), b"x")?;

        let dst = tempdir()?;
        fs::create_dir_all(dst.path().join("sub"))?; // conflicting dir already exists

        let res = copy_dir_contents(src.path(), dst.path());
        assert!(res.is_err());
        Ok(())
    }
}
