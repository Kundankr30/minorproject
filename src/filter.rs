use walkdir::DirEntry;
use regex::Regex;

pub fn file_filter(entry: &DirEntry, min_size: u64, max_size: u64, allowed_exts: &[&str], name_regex: &Regex) -> bool {
    let meta = match entry.metadata() {
        Ok(m) => m,
        Err(_) => return false,
    };
    let size = meta.len();
    let ext = entry.path().extension().and_then(|s| s.to_str()).unwrap_or("");
    size >= min_size && size <= max_size &&
    allowed_exts.iter().any(|x| x == &ext) &&
    name_regex.is_match(&entry.file_name().to_string_lossy())
} 