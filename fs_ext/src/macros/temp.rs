#[macro_export]
macro_rules! tempfile {
    () => {{ $crate::fsx::file::temp() }};

    ($dir:expr) => {{ $crate::fsx::file::temp_in($dir) }};

    ($dir:expr, $content:expr) => {{
        (|| -> ::std::io::Result<$crate::TempFile> {
            let mut __t: $crate::TempFile = $crate::fsx::file::temp_in($dir)?;
            use ::std::io::Write as _;
            __t.as_file_mut().write_all(::std::convert::AsRef::<[u8]>::as_ref(&$content))?;
            __t.as_file_mut().sync_all()?;
            Ok(__t)
        })()
    }};
}

#[macro_export]
macro_rules! tempdir {
    () => {{ $crate::fsx::dir::temp() }};
    ($parent:expr) => {{ $crate::fsx::dir::temp_in($parent) }};
}
