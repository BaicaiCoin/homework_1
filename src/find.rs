use std::path::Path;
use regex::Regex;

pub mod walk_tree;

pub fn find<P:AsRef<Path>>(root: P, regex: &Regex) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut matches = Vec::new();
    walk_tree::walk_tree(root.as_ref(), regex, &mut matches)?;
    Ok(matches)
}