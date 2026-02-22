use anyhow::{anyhow, Result};
use globset::{Glob, GlobSetBuilder};
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

/// Frontmatter structure from AGENTS.md
#[derive(Debug, Default, Deserialize)]
pub struct AgentsFrontmatter {
    #[serde(default)]
    pub forbidden: Vec<String>,
    #[serde(default)]
    pub read_only: Vec<String>,
    #[serde(default)]
    pub ignore: Vec<String>,
    #[serde(skip)]
    pub gitignore_patterns: Vec<String>,
}

/// Access level determined by AGENTS.md constraints
#[derive(Debug, PartialEq)]
pub enum AccessLevel {
    Forbidden,
    ReadOnly,
    Ignored,
    Allowed,
}

/// Find the nearest AGENTS.md file by walking up the directory tree
pub fn find_agents_md(target_path: &Path) -> Option<PathBuf> {
    let mut current = target_path.parent()?;

    loop {
        let agents_path = current.join("AGENTS.md");
        if agents_path.exists() {
            return Some(agents_path);
        }

        current = current.parent()?;
    }
}

/// Parse YAML frontmatter from AGENTS.md content
pub fn parse_frontmatter(content: &str) -> Result<Option<AgentsFrontmatter>> {
    // Check for frontmatter delimiters
    if !content.starts_with("---\n") {
        return Ok(None);
    }

    // Find end delimiter
    let rest = &content[4..];
    let end_pos = rest.find("\n---\n").ok_or_else(|| {
        anyhow!("Frontmatter missing closing delimiter")
    })?;

    let yaml = &rest[..end_pos];

    // Parse YAML
    let frontmatter: AgentsFrontmatter = serde_yaml::from_str(yaml)
        .map_err(|e| anyhow!("Failed to parse AGENTS.md frontmatter: {}", e))?;

    Ok(Some(frontmatter))
}

/// Determine access level for a path based on AGENTS.md constraints
fn determine_access_level(
    path: &Path,
    frontmatter: &AgentsFrontmatter,
) -> Result<AccessLevel> {
    // Build glob matchers
    let mut forbidden_builder = GlobSetBuilder::new();
    for pattern in &frontmatter.forbidden {
        let glob = Glob::new(pattern)
            .map_err(|e| anyhow!("Invalid glob pattern '{}': {}", pattern, e))?;
        forbidden_builder.add(glob);
    }
    let forbidden_set = forbidden_builder.build()?;

    let mut readonly_builder = GlobSetBuilder::new();
    for pattern in &frontmatter.read_only {
        let glob = Glob::new(pattern)
            .map_err(|e| anyhow!("Invalid glob pattern '{}': {}", pattern, e))?;
        readonly_builder.add(glob);
    }
    let readonly_set = readonly_builder.build()?;

    let mut ignore_builder = GlobSetBuilder::new();
    for pattern in &frontmatter.ignore {
        let glob = Glob::new(pattern)
            .map_err(|e| anyhow!("Invalid glob pattern '{}': {}", pattern, e))?;
        ignore_builder.add(glob);
    }
    let ignore_set = ignore_builder.build()?;

    // Check precedence: forbidden > read_only > ignore
    if forbidden_set.is_match(path) {
        return Ok(AccessLevel::Forbidden);
    }

    if readonly_set.is_match(path) {
        return Ok(AccessLevel::ReadOnly);
    }

    if ignore_set.is_match(path) {
        return Ok(AccessLevel::Ignored);
    }

    // Check .gitignore patterns
    let mut gitignore_builder = GlobSetBuilder::new();
    for pattern in &frontmatter.gitignore_patterns {
        if !pattern.is_empty() {
            let glob = Glob::new(pattern)
                .map_err(|e| anyhow!("Invalid gitignore pattern '{}': {}", pattern, e))?;
            gitignore_builder.add(glob);
        }
    }
    let gitignore_set = gitignore_builder.build()?;

    if gitignore_set.is_match(path) {
        return Ok(AccessLevel::Ignored);
    }

    Ok(AccessLevel::Allowed)
}

/// Check if a path can be read based on AGENTS.md constraints
pub fn check_read_access(path: &str) -> Result<()> {
    let path_buf = PathBuf::from(path);

    // Find nearest AGENTS.md
    let agents_md = match find_agents_md(&path_buf) {
        Some(p) => p,
        None => return Ok(()), // No AGENTS.md, allow all
    };

    // Read and parse frontmatter
    let content = fs::read_to_string(&agents_md)?;
    let mut frontmatter = parse_frontmatter(&content)?.unwrap_or_default();

    // Load .gitignore patterns
    let gitignore_files = crate::gitignore::find_gitignore_files(&path_buf);
    for gitignore_path in gitignore_files {
        if let Ok(patterns) = crate::gitignore::parse_gitignore(&gitignore_path) {
            frontmatter.gitignore_patterns.extend(patterns);
        }
    }

    // Make path relative to AGENTS.md directory for matching
    let agents_dir = agents_md.parent().unwrap();
    let relative_path = path_buf.strip_prefix(agents_dir).unwrap_or(&path_buf);

    // Determine access level
    let access = determine_access_level(relative_path, &frontmatter)?;

    match access {
        AccessLevel::Forbidden => Err(anyhow!(
            "Access to {} is forbidden by AGENTS.md",
            path
        )),
        AccessLevel::Ignored => Err(anyhow!(
            "{} is ignored by AGENTS.md",
            path
        )),
        AccessLevel::ReadOnly | AccessLevel::Allowed => Ok(()),
    }
}

/// Check if a path can be written based on AGENTS.md constraints
pub fn check_write_access(path: &str) -> Result<()> {
    let path_buf = PathBuf::from(path);

    // Find nearest AGENTS.md
    let agents_md = match find_agents_md(&path_buf) {
        Some(p) => p,
        None => return Ok(()), // No AGENTS.md, allow all
    };

    // Read and parse frontmatter
    let content = fs::read_to_string(&agents_md)?;
    let mut frontmatter = parse_frontmatter(&content)?.unwrap_or_default();

    // Load .gitignore patterns
    let gitignore_files = crate::gitignore::find_gitignore_files(&path_buf);
    for gitignore_path in gitignore_files {
        if let Ok(patterns) = crate::gitignore::parse_gitignore(&gitignore_path) {
            frontmatter.gitignore_patterns.extend(patterns);
        }
    }

    // Make path relative to AGENTS.md directory for matching
    let agents_dir = agents_md.parent().unwrap();
    let relative_path = path_buf.strip_prefix(agents_dir).unwrap_or(&path_buf);

    // Determine access level
    let access = determine_access_level(relative_path, &frontmatter)?;

    match access {
        AccessLevel::Forbidden => Err(anyhow!(
            "Access to {} is forbidden by AGENTS.md",
            path
        )),
        AccessLevel::ReadOnly => Err(anyhow!(
            "{} is read-only per AGENTS.md",
            path
        )),
        AccessLevel::Ignored => Err(anyhow!(
            "{} is ignored by AGENTS.md",
            path
        )),
        AccessLevel::Allowed => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_frontmatter_valid() {
        let content = r#"---
forbidden:
  - "secrets/**"
read_only:
  - "**/*.lock"
ignore:
  - "node_modules/**"
---

# Agent Instructions
"#;
        let result = parse_frontmatter(content).unwrap();
        assert!(result.is_some());
        let fm = result.unwrap();
        assert_eq!(fm.forbidden, vec!["secrets/**"]);
        assert_eq!(fm.read_only, vec!["**/*.lock"]);
        assert_eq!(fm.ignore, vec!["node_modules/**"]);
    }

    #[test]
    fn test_parse_frontmatter_none() {
        let content = "# Agent Instructions\n\nNo frontmatter here.";
        let result = parse_frontmatter(content).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_determine_access_level() {
        let frontmatter = AgentsFrontmatter {
            forbidden: vec!["secrets/**".to_string()],
            read_only: vec!["**/*.lock".to_string()],
            ignore: vec!["node_modules/**".to_string()],
            gitignore_patterns: vec![],
        };

        let path = Path::new("secrets/key.pem");
        assert_eq!(
            determine_access_level(path, &frontmatter).unwrap(),
            AccessLevel::Forbidden
        );

        let path = Path::new("package.lock");
        assert_eq!(
            determine_access_level(path, &frontmatter).unwrap(),
            AccessLevel::ReadOnly
        );

        let path = Path::new("node_modules/lib.js");
        assert_eq!(
            determine_access_level(path, &frontmatter).unwrap(),
            AccessLevel::Ignored
        );

        let path = Path::new("src/main.rs");
        assert_eq!(
            determine_access_level(path, &frontmatter).unwrap(),
            AccessLevel::Allowed
        );
    }
}
