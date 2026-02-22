use anyhow::{anyhow, Result};
use globset::{Glob, GlobSetBuilder};
use std::fs;
use std::path::Path;

use crate::agents;
use crate::tools::FileReadResult;

/// List directory contents with [FILE] or [DIR] prefixes
pub fn list_directory_impl(path: &str) -> Result<String> {
    agents::check_read_access(path)?;
    
    let entries = fs::read_dir(path)?;
    let mut result = String::new();
    
    let mut items: Vec<_> = entries.filter_map(|e| e.ok()).collect();
    items.sort_by_key(|e| e.file_name());
    
    for entry in items {
        let file_type = entry.file_type()?;
        let prefix = if file_type.is_dir() { "[DIR]" } else { "[FILE]" };
        result.push_str(&format!("{} {}\n", prefix, entry.file_name().to_string_lossy()));
    }
    
    Ok(result)
}

/// Get recursive directory tree in compact text format
pub fn directory_tree_impl(path: &str, exclude_patterns: &[String]) -> Result<String> {
    agents::check_read_access(path)?;
    
    let exclude_set = if exclude_patterns.is_empty() {
        GlobSetBuilder::new().build()?
    } else {
        let mut builder = GlobSetBuilder::new();
        for pattern in exclude_patterns {
            let glob = Glob::new(pattern)?;
            builder.add(glob);
        }
        builder.build()?
    };
    
    let mut output = String::new();
    let root_path = Path::new(path);
    
    // Print root directory name
    output.push_str(&format!("{}/\n", root_path.file_name().unwrap_or_default().to_string_lossy()));
    
    // Build tree for children
    build_tree_text(root_path, &exclude_set, "", true, &mut output)?;
    
    Ok(output)
}

fn build_tree_text(
    path: &Path,
    exclude: &globset::GlobSet,
    prefix: &str,
    _is_last: bool,
    output: &mut String,
) -> Result<()> {
    if !path.is_dir() {
        return Ok(());
    }
    
    let mut entries: Vec<_> = fs::read_dir(path)?
        .filter_map(|e| e.ok())
        .filter(|e| !exclude.is_match(e.path()))
        .collect();
    
    entries.sort_by_key(|e| e.file_name());
    
    for (i, entry) in entries.iter().enumerate() {
        let is_last_entry = i == entries.len() - 1;
        let entry_path = entry.path();
        let name = entry.file_name();
        let is_dir = entry_path.is_dir();
        
        // Print current entry
        let connector = if is_last_entry { "└── " } else { "├── " };
        let suffix = if is_dir { "/" } else { "" };
        output.push_str(&format!("{}{}{}{}\n", prefix, connector, name.to_string_lossy(), suffix));
        
        // Recurse into directories
        if is_dir {
            let new_prefix = format!(
                "{}{}",
                prefix,
                if is_last_entry { "    " } else { "│   " }
            );
            build_tree_text(&entry_path, exclude, &new_prefix, is_last_entry, output)?;
        }
    }
    
    Ok(())
}

/// Create directory with parents
pub fn create_directory_impl(path: &str) -> Result<String> {
    agents::check_write_access(path)?;
    
    fs::create_dir_all(path)?;
    Ok(format!("Created directory: {}", path))
}

/// Move or rename file/directory
pub fn move_file_impl(source: &str, destination: &str) -> Result<String> {
    agents::check_read_access(source)?;
    agents::check_write_access(destination)?;
    
    if Path::new(destination).exists() {
        return Err(anyhow!("Destination already exists: {}", destination));
    }
    
    fs::rename(source, destination)?;
    Ok(format!("Moved {} to {}", source, destination))
}

/// Write raw UTF-8 content to file (non-hashline)
pub fn write_file_impl(path: &str, content: &str) -> Result<String> {
    agents::check_write_access(path)?;
    
    fs::write(path, content)?;
    Ok(format!("Wrote {} bytes to {}", content.len(), path))
}

/// Read multiple files in one operation
pub fn read_multiple_files_impl(paths: &[String], for_edit: bool) -> Result<String> {
    let mut results = Vec::new();
    
    for path in paths {
        let result = match agents::check_read_access(path) {
            Ok(_) => match fs::read_to_string(path) {
                Ok(content) => {
                    let final_content = if for_edit {
                        // Return hashline-tagged content for editing
                        let tagged = crate::hashline::tag_content(&content);
                        let file_hash = crate::hashline::compute_file_hash(&content);
                        let total_lines = content.lines().count();
                        format!(
                            "[Metadata: total_lines={}, file_hash={}]\n{}",
                            total_lines, file_hash, tagged
                        )
                    } else {
                        // Return raw content
                        content
                    };
                    
                    FileReadResult {
                        path: path.clone(),
                        content: Some(final_content),
                        error: None,
                        success: true,
                    }
                },
                Err(e) => FileReadResult {
                    path: path.clone(),
                    content: None,
                    error: Some(e.to_string()),
                    success: false,
                },
            },
            Err(e) => FileReadResult {
                path: path.clone(),
                content: None,
                error: Some(e.to_string()),
                success: false,
            },
        };
        results.push(result);
    }
    
    Ok(serde_json::to_string_pretty(&results)?)
}
