use rmcp::{
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    schemars,
};
use serde::{Deserialize, Serialize};
use std::fs;

use crate::{agents, hashline};

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ReadTextInput {
    #[schemars(description = "Absolute path to the file to read")]
    pub path: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct WriteTextInput {
    #[schemars(description = "Absolute path to the file to write")]
    pub path: String,
    #[schemars(description = "Content to write to the file")]
    pub content: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct EditTextInput {
    #[schemars(description = "Absolute path to the file to edit")]
    pub path: String,
    #[schemars(description = "6-character hash of the entire file content from the last read")]
    pub file_hash: String,
    pub operations: Vec<EditOperation>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct EditOperation {
    #[schemars(description = "Type of operation: replace, insert_after, insert_before, or delete")]
    pub op_type: String,
    #[schemars(description = "Anchor in lineNum:hash format")]
    pub anchor: String,
    #[schemars(description = "Optional end anchor in lineNum:hash format for range operations")]
    pub end_anchor: Option<String>,
    #[schemars(description = "New content for replace or insert operations")]
    pub content: Option<String>,
}

// Filesystem tool input structs

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ReadFileInput {
    #[schemars(description = "Absolute path to the file to read")]
    pub path: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ListDirectoryInput {
    #[schemars(description = "Absolute path to the directory to list")]
    pub path: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct DirectoryTreeInput {
    #[schemars(description = "Absolute path to the directory")]
    pub path: String,
    #[schemars(description = "Optional glob patterns to exclude")]
    pub exclude_patterns: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CreateDirectoryInput {
    #[schemars(description = "Absolute path to the directory to create")]
    pub path: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct MoveFileInput {
    #[schemars(description = "Absolute path to the source file or directory")]
    pub source: String,
    #[schemars(description = "Absolute path to the destination")]
    pub destination: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct WriteFileInput {
    #[schemars(description = "Absolute path to the file to write")]
    pub path: String,
    #[schemars(description = "Content to write to the file")]
    pub content: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ReadMultipleFilesInput {
    #[schemars(description = "List of absolute paths to files to read")]
    pub paths: Vec<String>,
    #[schemars(description = "If true, return hashline-tagged content for editing. If false (default), return raw content")]
    pub for_edit: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct FileReadResult {
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub success: bool,
}

#[derive(Debug, Clone)]
pub struct HashfileServer {
    pub tool_router: ToolRouter<Self>,
}

#[rmcp::tool_router]
impl HashfileServer {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[rmcp::tool(
        description = "Read a file and return hashline-tagged content for reliable editing"
    )]
    fn read_text_file(
        &self,
        Parameters(ReadTextInput { path }): Parameters<ReadTextInput>,
    ) -> String {
        match Self::read_text_file_impl(&path) {
            Ok(output) => output,
            Err(e) => format!("Error: {}", e),
        }
    }

    #[rmcp::tool(description = "Read a file and return raw content (without hashline tags)")]
    fn read_file(
        &self,
        Parameters(ReadFileInput { path }): Parameters<ReadFileInput>,
    ) -> String {
        match Self::read_file_impl(&path) {
            Ok(content) => content,
            Err(e) => format!("Error: {}", e),
        }
    }

    #[rmcp::tool(description = "Write content to a file, creating it if it doesn't exist")]
    fn write_text_file(&self, Parameters(input): Parameters<WriteTextInput>) -> String {
        match Self::write_text_file_impl(&input.path, &input.content) {
            Ok(msg) => msg,
            Err(e) => format!("Error: {}", e),
        }
    }

    #[rmcp::tool(description = "Edit a file using hash-anchored operations")]
    fn edit_text_file(&self, Parameters(input): Parameters<EditTextInput>) -> String {
        match Self::edit_text_file_impl(&input.path, &input.file_hash, input.operations) {
            Ok(msg) => msg,
            Err(e) => format!("Error: {}", e),
        }
    }

    #[rmcp::tool(description = "List directory contents with [FILE] or [DIR] prefixes")]
    fn list_directory(&self, Parameters(input): Parameters<ListDirectoryInput>) -> String {
        match crate::filesystem::list_directory_impl(&input.path) {
            Ok(result) => result,
            Err(e) => format!("Error: {}", e),
        }
    }

    #[rmcp::tool(description = "Get recursive directory tree in compact text format (like Unix tree command)")]
    fn directory_tree(&self, Parameters(input): Parameters<DirectoryTreeInput>) -> String {
        let patterns = input.exclude_patterns.unwrap_or_default();
        match crate::filesystem::directory_tree_impl(&input.path, &patterns) {
            Ok(result) => result,
            Err(e) => format!("Error: {}", e),
        }
    }

    #[rmcp::tool(description = "Create a directory, including parent directories if needed")]
    fn create_directory(&self, Parameters(input): Parameters<CreateDirectoryInput>) -> String {
        match crate::filesystem::create_directory_impl(&input.path) {
            Ok(result) => result,
            Err(e) => format!("Error: {}", e),
        }
    }

    #[rmcp::tool(description = "Move or rename a file or directory")]
    fn move_file(&self, Parameters(input): Parameters<MoveFileInput>) -> String {
        match crate::filesystem::move_file_impl(&input.source, &input.destination) {
            Ok(result) => result,
            Err(e) => format!("Error: {}", e),
        }
    }

    #[rmcp::tool(description = "Write raw UTF-8 content to a file (overwrites existing content)")]
    fn write_file(&self, Parameters(input): Parameters<WriteFileInput>) -> String {
        match crate::filesystem::write_file_impl(&input.path, &input.content) {
            Ok(result) => result,
            Err(e) => format!("Error: {}", e),
        }
    }

    #[rmcp::tool(description = "Read multiple files in one operation, returns JSON array of results")]
    fn read_multiple_files(&self, Parameters(input): Parameters<ReadMultipleFilesInput>) -> String {
        let for_edit = input.for_edit.unwrap_or(false);
        match crate::filesystem::read_multiple_files_impl(&input.paths, for_edit) {
            Ok(result) => result,
            Err(e) => format!("Error: {}", e),
        }
    }
}

impl HashfileServer {
    fn read_text_file_impl(path: &str) -> anyhow::Result<String> {
        // Check AGENTS.md constraints
        agents::check_read_access(path)?;

        let content = fs::read_to_string(path)?;
        let tagged = hashline::tag_content(&content);
        let file_hash = hashline::compute_file_hash(&content);
        let total_lines = content.lines().count();

        let output = format!(
            "{}\n---\nhashline_version: 1\ntotal_lines: {}\nfile_hash: {}\n",
            tagged, total_lines, file_hash
        );

        Ok(output)
    }

    fn read_file_impl(path: &str) -> anyhow::Result<String> {
        // Check AGENTS.md constraints
        agents::check_read_access(path)?;

        let content = fs::read_to_string(path)?;
        Ok(content)
    }

    fn write_text_file_impl(path: &str, content: &str) -> anyhow::Result<String> {
        // Check AGENTS.md constraints
        agents::check_write_access(path)?;

        fs::write(path, content)?;
        Ok(format!("Successfully wrote {} bytes to {}", content.len(), path))
    }

    fn edit_text_file_impl(
        path: &str,
        file_hash: &str,
        operations: Vec<EditOperation>,
    ) -> anyhow::Result<String> {
        // Check AGENTS.md constraints
        agents::check_write_access(path)?;

        let current_content = fs::read_to_string(path)?;
        let current_hash = hashline::compute_file_hash(&current_content);

        if current_hash != file_hash {
            return Err(anyhow::anyhow!(
                "File {} has been modified since it was last read. Please re-read the file.",
                path
            ));
        }

        let mut ops = Vec::new();
        for op in operations {
            let anchor = op.anchor.parse::<hashline::LineAnchor>()?;
            let end_anchor = if let Some(ea) = op.end_anchor {
                Some(ea.parse::<hashline::LineAnchor>()?)
            } else {
                None
            };

            let op_type = match op.op_type.as_str() {
                "replace" => hashline::OperationType::Replace,
                "insert_after" => hashline::OperationType::InsertAfter,
                "insert_before" => hashline::OperationType::InsertBefore,
                "delete" => hashline::OperationType::Delete,
                _ => return Err(anyhow::anyhow!("Invalid operation type: {}", op.op_type)),
            };

            ops.push(hashline::HashlineOperation {
                op_type,
                anchor,
                end_anchor,
                content: op.content,
            });
        }

        let new_content = hashline::apply_operations(&current_content, ops)?;
        fs::write(path, &new_content)?;

        Ok(format!("Successfully edited {}", path))
    }
}
