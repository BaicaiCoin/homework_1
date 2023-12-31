use std::fs;
use std::path::Path;
use regex::Regex;

pub fn walk_tree(
    dir: &Path,
    regex: &Regex,
    matches: &mut Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                walk_tree(&path, regex, matches)?;
            } else if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
                if regex.is_match(filename) {
                    matches.push(path.to_string_lossy().to_string());
                }
            }
        }
    } else {
            if let Some(filename) = dir.file_name().and_then(|s| s.to_str()) {
                if regex.is_match(filename) {
                matches.push(dir.to_string_lossy().to_string());
            }
        }
    }
    Ok(())
}