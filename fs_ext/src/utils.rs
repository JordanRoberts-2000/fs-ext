pub fn normalize_ext(s: &str) -> Option<String> {
    let s = s.trim().trim_start_matches('.');
    if s.is_empty() { None } else { Some(s.to_ascii_lowercase()) }
}
