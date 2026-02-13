# Hashfile MCP Server

A Model Context Protocol (MCP) server that provides reliable file editing through hash-anchored operations. Built with Rust and rmcp 0.15.0.

## Features

- **Hash-Anchored Editing**: Every line is tagged with a content hash, ensuring edits target the correct location even if line numbers change
- **Content Verification**: File-level 6-character hashing prevents editing stale content
- **Fuzzy Matching**: Automatically finds the correct line if content has shifted
- **MCP Protocol**: Standard Model Context Protocol for AI agent integration

## Installation

```bash
cargo build --release
```

The binary will be at `target/release/hashfile-mcp`.

## Usage

### MCP Client Configuration

Add to your MCP client configuration (e.g., Claude Desktop):

```json
{
  "mcpServers": {
    "hashfile": {
      "command": "/path/to/hashfile-mcp/target/release/hashfile-mcp"
    }
  }
}
```

### Available Tools

#### `read_text_file`

**Description:** Read a file and return hashline-tagged content for reliable editing

**Input:**
```json
{
  "path": "/absolute/path/to/file.txt"
}
```

**Output:**
```
1:a3|First line of content
2:7f|Second line of content
3:2c|Third line of content
---
hashline_version: 1
total_lines: 3
file_hash: 8f3a9b
```

#### `write_text_file`

**Description:** Write content to a file, creating it if it doesn't exist

**Input:**
```json
{
  "path": "/absolute/path/to/file.txt",
  "content": "File content to write"
}
```

**Output:**
```
Successfully wrote 21 bytes to /absolute/path/to/file.txt
```

#### `edit_text_file`

**Description:** Edit a file using hash-anchored operations

**Input:**
```json
{
  "path": "/absolute/path/to/file.txt",
  "file_hash": "abc123...",
  "operations": [
    {
      "op_type": "replace",
      "anchor": "2:7f",
      "content": "New second line"
    }
  ]
}
```

**Operation Types:**
- `replace`: Replace line(s) at anchor
- `insert_after`: Insert content after anchor
- `insert_before`: Insert content before anchor
- `delete`: Delete line(s) at anchor

**Anchors:**
- Format: `"lineNum:hash"` (e.g., `"5:a3"`)
- For ranges: use `anchor` and `end_anchor`

## How It Works

### Hashline Concept

Each line is tagged with:
1. **Line number**: 1-indexed position
2. **Content hash**: 2-character hex hash (FNV-1a) of trimmed line content
3. **Separator**: `|` character
4. **Original content**: The actual line text

Example: `5:a3|console.log("hello");`

### Reliability

- **Content verification**: File hash must match before editing
- **Anchor resolution**: Finds correct line even if line numbers changed
- **Fuzzy matching**: If exact line doesn't match, searches for unique hash match
- **Error detection**: Reports conflicts if content has changed

## Development

### Run Tests

```bash
cargo test
```

### Project Structure

```
src/
├── main.rs       # MCP server setup
├── tools.rs      # MCP tool definitions
└── hashline.rs   # Core hashline logic
```

## Technical Details

- **Language**: Rust (edition 2021)
- **MCP SDK**: rmcp 0.15.0
- **Line Hashing**: FNV-1a (2 hex chars)
- **File Hashing**: FNV-1a (6 hex chars)
- **Transport**: stdio

## License

See LICENSE file for details.

## See Also

- [IDEA](https://blog.can.ac/2026/02/12/the-harness-problem/) - I Improved 15 LLMs at Coding in One Afternoon. Only the Harness Changed. - Can Bölük Feb 2026
