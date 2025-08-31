#[macro_export]
macro_rules! tempfile {
    // system temp dir -> io::Result<TempFile>
    () => {{ $crate::fsx::file::temp() }};

    // create under a specific directory -> io::Result<TempFile>
    ($dir:expr) => {{ $crate::fsx::file::temp_in($dir) }};

    // create under dir and write initial content (bytes-like) -> io::Result<TempFile>
    ($dir:expr, $content:expr) => {{
        (|| -> ::std::io::Result<$crate::fsx::TempFile> {
            let mut __t: $crate::fsx::TempFile = $crate::fsx::file::temp_in($dir)?;
            use ::std::io::Write as _;
            // Accepts b"...", &[u8], Vec<u8], &Vec<u8], etc. For &str/String use .as_bytes().
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
