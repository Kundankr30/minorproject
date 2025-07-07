use std::path::Path;
use std::fs;

pub fn move_to_quarantine<P: AsRef<Path>>(file_path: P, quarantine_dir: P) -> std::io::Result<()> {
    let file_name = file_path.as_ref().file_name().unwrap();
    let dest = quarantine_dir.as_ref().join(file_name);
    fs::rename(&file_path, &dest)
} 