pub fn normalize_ext(s: &str) -> String {
    s.trim().trim_start_matches('.').to_ascii_lowercase()
}

#[cfg(test)]
mod tests {
    use super::normalize_ext;

    #[test]
    fn normalize_ext_cases() {
        let cases = [
            ("png", "png"),        // already normalized
            ("PNG", "png"),        // case-fold
            (".jpeg", "jpeg"),     // single leading dot
            ("..jpeg", "jpeg"),    // multiple leading dots
            ("  .Jpg  ", "jpg"),   // trim + dot-strip + lowercase
            (".tar.gz", "tar.gz"), // keep interior dot
            ("", ""),              // empty
            ("   ", ""),           // whitespace-only
            ("...", ""),           // dots-only
            ("png.", "png."),      // trailing dot preserved (only leading dots stripped)
        ];

        for (input, expected) in cases {
            assert_eq!(normalize_ext(input), expected, "input: {:?}", input);
        }
    }
}
