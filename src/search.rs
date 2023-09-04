use regex::Regex;
use std::fs;
use std::path::Path;

pub fn find<P: AsRef<Path>>(root: P, regex: &Regex) -> Result<(Vec<String>, usize), Box<dyn std::error::Error>> {
    let mut matches = Vec::new();
    let mut unmatches = Vec::new();
    walk_tree(root.as_ref(), regex, &mut matches, &mut unmatches)?;
    let match_count = matches.len();
    matches.append(&mut unmatches);
    Ok((matches, match_count))
}

fn walk_tree(
    dir: &Path,
    regex: &Regex,
    matches: &mut Vec<String>,
    unmatches: &mut Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                walk_tree(&path, regex, matches, unmatches)?;
            } else if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
                if regex.is_match(filename) {
                    matches.push(path.to_string_lossy().to_string());
                } else {
                    unmatches.push(path.to_string_lossy().to_string());
                }
            }
        }
    }
    Ok(())
}