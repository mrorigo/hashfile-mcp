use anyhow::{anyhow, Result};
use fnv::FnvHasher;
use std::hash::Hasher;

/// Computes the 2-character hex hash of a line's content (trimmed of trailing whitespace).
pub fn hash_line(content: &str) -> String {
    let trimmed = content.trim_end();
    let mut hasher = FnvHasher::default();
    hasher.write(trimmed.as_bytes());
    let hash = (hasher.finish() & 0xff) as u8;
    format!("{:02x}", hash)
}

/// Tags each line of the content with its line number and hash.
pub fn tag_content(content: &str) -> String {
    let lines: Vec<&str> = content.lines().collect();
    let mut result = String::new();
    for (i, line) in lines.iter().enumerate() {
        let h = hash_line(line);
        result.push_str(&format!("{}:{}|{}\n", i + 1, h, line));
    }
    result
}

/// Computes a 6-character hex hash of the entire file content using FNV.
/// This is shorter and more agent-friendly than SHA-256 while providing
/// sufficient collision resistance for practical file editing scenarios.
pub fn compute_file_hash(content: &str) -> String {
    let mut hasher = FnvHasher::default();
    hasher.write(content.as_bytes());
    let hash = hasher.finish();
    // Use 24 bits (6 hex chars) for reasonable collision resistance
    format!("{:06x}", hash & 0xFFFFFF)
}

#[derive(Debug, Clone, PartialEq)]
pub struct LineAnchor {
    pub line_num: usize,
    pub hash: String,
}

impl std::str::FromStr for LineAnchor {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 2 {
            return Err(anyhow!("Invalid anchor format. Expected 'line_num:hash'"));
        }
        let line_num = parts[0].parse::<usize>()?;
        let hash = parts[1].to_string();
        Ok(LineAnchor { line_num, hash })
    }
}

pub enum OperationType {
    Replace,
    InsertAfter,
    InsertBefore,
    Delete,
}

pub struct HashlineOperation {
    pub op_type: OperationType,
    pub anchor: LineAnchor,
    pub end_anchor: Option<LineAnchor>,
    pub content: Option<String>,
}

/// Resolves a line anchor to its current line index in the file.
/// Provides exact match first, then fuzzy match by hash if exactly one match is found.
pub fn resolve_anchor(lines: &[&str], anchor: &LineAnchor) -> Result<usize> {
    // 1-indexed to 0-indexed
    let idx = anchor.line_num.saturating_sub(1);

    // 1. Exact match
    if idx < lines.len() && hash_line(lines[idx]) == anchor.hash {
        return Ok(idx);
    }

    // 2. Fuzzy match (search for unique hash)
    let mut matches = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        if hash_line(line) == anchor.hash {
            matches.push(i);
        }
    }

    if matches.len() == 1 {
        Ok(matches[0])
    } else if matches.is_empty() {
        Err(anyhow!(
            "Anchor {}:{} not found",
            anchor.line_num,
            anchor.hash
        ))
    } else {
        Err(anyhow!(
            "Anchor {}:{} is ambiguous ({} matches found)",
            anchor.line_num,
            anchor.hash,
            matches.len()
        ))
    }
}

pub fn apply_operations(content: &str, operations: Vec<HashlineOperation>) -> Result<String> {
    let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

    // Sort operations by anchor line number in reverse to avoid index shifts affecting subsequent operations.
    // However, since anchors can move, we should resolve all anchors against ORIGINAL state first,
    // or apply them carefully.

    // For simplicity, we'll collect the resolved target indices first.
    let ref_lines: Vec<&str> = lines.iter().map(|s| s.as_str()).collect();
    let mut resolved_ops: Vec<(usize, Option<usize>, OperationType, Option<String>)> = Vec::new();

    for op in operations {
        let start_idx = resolve_anchor(&ref_lines, &op.anchor)?;
        let end_idx = if let Some(ref end) = op.end_anchor {
            Some(resolve_anchor(&ref_lines, end)?)
        } else {
            None
        };
        resolved_ops.push((start_idx, end_idx, op.op_type, op.content));
    }

    // Sort by start_idx descending
    resolved_ops.sort_by(|a, b| b.0.cmp(&a.0));

    for (start, end, op_type, content) in resolved_ops {
        match op_type {
            OperationType::Replace => {
                let count = if let Some(e) = end {
                    if e < start {
                        return Err(anyhow!("End anchor is before start anchor"));
                    }
                    e - start + 1
                } else {
                    1
                };
                lines.drain(start..start + count);
                if let Some(c) = content {
                    let new_lines: Vec<String> = c.lines().map(|s| s.to_string()).collect();
                    for (i, nl) in new_lines.into_iter().enumerate() {
                        lines.insert(start + i, nl);
                    }
                }
            }
            OperationType::Delete => {
                let count = if let Some(e) = end {
                    if e < start {
                        return Err(anyhow!("End anchor is before start anchor"));
                    }
                    e - start + 1
                } else {
                    1
                };
                lines.drain(start..start + count);
            }
            OperationType::InsertAfter => {
                if let Some(c) = content {
                    let new_lines: Vec<String> = c.lines().map(|s| s.to_string()).collect();
                    for (i, nl) in new_lines.into_iter().enumerate() {
                        lines.insert(start + 1 + i, nl);
                    }
                }
            }
            OperationType::InsertBefore => {
                if let Some(c) = content {
                    let new_lines: Vec<String> = c.lines().map(|s| s.to_string()).collect();
                    for (i, nl) in new_lines.into_iter().enumerate() {
                        lines.insert(start + i, nl);
                    }
                }
            }
        }
    }

    let mut result = lines.join("\n");
    if content.ends_with('\n') && !result.ends_with('\n') {
        result.push('\n');
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_line() {
        assert_eq!(hash_line("hello"), hash_line("hello  "));
        assert_ne!(hash_line("hello"), hash_line("world"));
        let h = hash_line("test");
        assert_eq!(h.len(), 2);
    }

    #[test]
    fn test_apply_operations() -> Result<()> {
        let content = "line1\nline2\nline3\n";
        let h2 = hash_line("line2");
        let ops = vec![HashlineOperation {
            op_type: OperationType::Replace,
            anchor: format!("2:{}", h2).parse()?,
            end_anchor: None,
            content: Some("new line 2".to_string()),
        }];

        let result = apply_operations(content, ops)?;
        assert_eq!(result, "line1\nnew line 2\nline3\n");
        Ok(())
    }
}
