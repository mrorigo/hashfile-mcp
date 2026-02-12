use std::path::PathBuf;
use rmcp::model::Root;
use anyhow::{Result, anyhow};
use url::Url;

pub struct RootsManager {
    roots: Vec<Root>,
}

impl RootsManager {
    pub fn new() -> Self {
        Self { roots: Vec::new() }
    }

    pub fn set_roots(&mut self, roots: Vec<Root>) {
        self.roots = roots;
    }

    pub fn is_path_allowed(&self, path_str: &str) -> Result<bool> {
        let path = PathBuf::from(path_str);
        
        // Ensure path is absolute
        if !path.is_absolute() {
            return Err(anyhow!("Path must be absolute: {}", path_str));
        }

        // We use canonicalize to resolve symlinks and '..' for security.
        // If the path doesn't exist, we check its parent.
        let absolute_path = if path.exists() {
            match path.canonicalize() {
                Ok(p) => p,
                Err(e) => return Err(anyhow!("Failed to canonicalize path {}: {}", path_str, e)),
            }
        } else {
            let parent = path.parent().ok_or_else(|| anyhow!("Path has no parent"))?;
            if parent.exists() {
                match parent.canonicalize() {
                    Ok(p) => p.join(path.file_name().ok_or_else(|| anyhow!("Invalid filename"))?),
                    Err(e) => return Err(anyhow!("Failed to canonicalize parent of {}: {}", path_str, e)),
                }
            } else {
                return Ok(false);
            }
        };

        for root in &self.roots {
            if let Ok(root_uri) = Url::parse(&root.uri) {
                if root_uri.scheme() == "file" {
                    let root_path_str = root_uri.path();
                    let root_path = PathBuf::from(root_path_str);
                    if let Ok(abs_root) = root_path.canonicalize() {
                        if absolute_path.starts_with(abs_root) {
                            return Ok(true);
                        }
                    } else if absolute_path.starts_with(root_path) {
                        return Ok(true);
                    }
                }
            }
        }
        Ok(false)
    }
}
