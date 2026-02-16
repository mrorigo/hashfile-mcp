use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

/// Parse a .gitignore file and return glob patterns
pub fn parse_gitignore(gitignore_path: &Path) -> Result<Vec<String>> {
    let content = fs::read_to_string(gitignore_path)?;
    let patterns: Vec<String> = content
        .lines()
        .filter(|line| !line.trim().is_empty() && !line.starts_with('#'))
        .map(|line| convert_gitignore_to_glob(line.trim()))
        .filter(|pattern| !pattern.is_empty())
        .collect();
    Ok(patterns)
}

/// Convert .gitignore pattern to glob pattern
fn convert_gitignore_to_glob(pattern: &str) -> String {
    // Handle negation (not supported in our current system)
    if pattern.starts_with('!') {
        return String::new(); // Skip negation patterns for now
    }

    // .gitignore patterns are relative to the .gitignore location
    // Convert to glob format
    if pattern.ends_with('/') {
        // Directory pattern: "node_modules/" -> "node_modules/**"
        format!("{}**", pattern)
    } else if pattern.contains('/') {
        // Path pattern: "build/output" -> "build/output"
        pattern.to_string()
    } else {
        // Basename pattern: "*.log" -> "**/*.log"
        format!("**/{}", pattern)
    }
}

/// Find all .gitignore files from target path up to root
pub fn find_gitignore_files(target_path: &Path) -> Vec<PathBuf> {
    let mut gitignores = Vec::new();
    let mut current = target_path.parent();

    while let Some(dir) = current {
        let gitignore_path = dir.join(".gitignore");
        if gitignore_path.exists() {
            gitignores.push(gitignore_path);
        }
        current = dir.parent();
    }

    gitignores.reverse(); // Root first for correct precedence
    gitignores
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_gitignore_to_glob() {
        assert_eq!(convert_gitignore_to_glob("*.log"), "**/*.log");
        assert_eq!(convert_gitignore_to_glob("node_modules/"), "node_modules/**");
        assert_eq!(convert_gitignore_to_glob("build/output"), "build/output");
        assert_eq!(convert_gitignore_to_glob("!important.log"), ""); // Negation skipped
    }

    #[test]
    fn test_parse_gitignore_filters_comments() {
        use std::io::Write;
        let temp_dir = std::env::temp_dir();
        let gitignore_path = temp_dir.join("test_gitignore");
        
        let mut file = fs::File::create(&gitignore_path).unwrap();
        writeln!(file, "# This is a comment").unwrap();
        writeln!(file, "*.log").unwrap();
        writeln!(file, "").unwrap();
        writeln!(file, "node_modules/").unwrap();
        writeln!(file, "# Another comment").unwrap();
        drop(file);

        let patterns = parse_gitignore(&gitignore_path).unwrap();
        assert_eq!(patterns.len(), 2);
        assert_eq!(patterns[0], "**/*.log");
        assert_eq!(patterns[1], "node_modules/**");

        fs::remove_file(gitignore_path).ok();
    }
}
