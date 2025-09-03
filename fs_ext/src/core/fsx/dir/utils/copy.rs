use {
    crate::fsx,
    std::{io, path::Path},
};

pub fn copy(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    _copy(src.as_ref(), dst.as_ref())
}

fn _copy(src: &Path, dst: &Path) -> io::Result<()> {
    fsx::dir::assert_exists(src)?;
    fsx::dir::ensure(dst)?;
    fsx::dir::copy_dir_contents(src, dst)
}

#[cfg(test)]
mod tests {
    use {
        super::copy,
        std::{fs, io},
        tempfile::tempdir,
    };

    #[test]
    fn creates_dst_when_missing_and_copies_entire_tree() -> io::Result<()> {
        let src = tempdir()?;
        let root = src.path();
        fs::create_dir_all(root.join("a/b"))?;
        fs::write(root.join("a/b/file.txt"), b"hello")?;
        fs::write(root.join("root.bin"), [1u8, 2, 3])?;

        let tmp = tempdir()?;
        let dst = tmp.path().join("my_copy");

        copy(src.path(), &dst)?;

        assert!(dst.is_dir());
        assert_eq!(fs::read(dst.join("a/b/file.txt"))?, b"hello");
        assert_eq!(fs::read(dst.join("root.bin"))?, vec![1, 2, 3]);
        Ok(())
    }

    #[test]
    fn overwrites_existing_files_in_destination() -> io::Result<()> {
        let src = tempdir()?;
        fs::write(src.path().join("same.txt"), b"SRC")?;

        let dst_root = tempdir()?;
        let dst = dst_root.path().join("dst");
        fs::create_dir_all(&dst)?;

        fs::write(dst.join("same.txt"), b"DST")?;

        copy(src.path(), &dst)?;
        assert_eq!(fs::read(dst.join("same.txt"))?, b"SRC");
        Ok(())
    }

    #[test]
    fn errors_when_src_is_a_file() -> io::Result<()> {
        let tmp = tempdir()?;
        let src_file = tmp.path().join("not_a_dir");
        fs::write(&src_file, b"x")?;

        let dst_root = tempdir()?;
        let dst = dst_root.path().join("dst");

        let res = copy(&src_file, &dst);
        assert!(res.is_err(), "copy() should error when src is a file");
        Ok(())
    }

    #[test]
    fn errors_when_dst_is_a_file() -> io::Result<()> {
        let src = tempdir()?;
        fs::write(src.path().join("a.txt"), b"x")?;

        let tmp = tempdir()?;
        let dst_file = tmp.path().join("not_a_dir");
        fs::write(&dst_file, b"")?;

        let res = copy(src.path(), &dst_file);
        assert!(res.is_err(), "copy() should error when dst path is a file");
        Ok(())
    }

    #[test]
    fn copying_twice_fails_on_conflicting_subdir() -> io::Result<()> {
        // Because underlying implementation creates subdirs with `create_new`,
        // the second copy into the same `dst` will hit a conflicting subdir and error.
        let src = tempdir()?;
        fs::create_dir_all(src.path().join("sub"))?;
        fs::write(src.path().join("sub").join("x.txt"), b"x")?;

        let dst_root = tempdir()?;
        let dst = dst_root.path().join("dst");

        copy(src.path(), &dst)?;
        assert_eq!(fs::read(dst.join("sub/x.txt"))?, b"x");

        let res = copy(src.path(), &dst);
        assert!(
            res.is_err(),
            "second copy should fail due to existing subdir created with create_new"
        );
        Ok(())
    }
}
