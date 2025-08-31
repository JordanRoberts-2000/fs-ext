#[macro_export]
macro_rules! file {
    ($path:expr) => {{ $crate::fsx::file::ensure($path) }};

    ($path:expr, $content:expr) => {{
        (|| -> ::std::io::Result<::std::fs::File> {
            let mut __f: ::std::fs::File = $crate::fsx::file::overwrite($path)?;
            use ::std::io::Write as _;
            __f.write_all(::std::convert::AsRef::<[u8]>::as_ref(&$content))?;
            Ok(__f)
        })()
    }};
}
