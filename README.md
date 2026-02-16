# Hashfile MCP Server

**Fine-Grained Control for AI File Operations** — A Model Context Protocol (MCP) server that gives you surgical precision over file editing and ironclad control over file access.

## Why Hashfile MCP?

### 🎯 **Surgical File Editing**
Unlike traditional line-number-based editing that breaks when files change, Hashfile uses **content-anchored operations**:
- **Hash-Anchored Lines**: Every line tagged with a content hash — edits target the *right content*, not just the right line number
- **Fuzzy Matching**: Automatically finds moved lines even after insertions/deletions
- **Collision Detection**: File-level verification prevents editing stale content
- **Multi-Operation Edits**: Apply multiple precise changes in a single atomic operation

### 🔒 **Ironclad Access Control**
Take complete control over what AI agents can touch with **AGENTS.md frontmatter**:
- **`forbidden`**: Block access to secrets, credentials, sensitive data
- **`read_only`**: Allow reading schemas, configs, lock files — prevent modifications
- **`ignore`**: Hide generated code, dependencies, build artifacts from AI context
- **Hierarchical Discovery**: Place AGENTS.md anywhere in your tree — nearest file wins
- **Glob Patterns**: Fine-tune access with wildcards (`secrets/**`, `**/*.env`)

[📖 Full AGENTS.md Documentation](docs/agents-support.md) | [📋 Proposal Spec](docs/agents.frontmatter.md)

### 🛠️ **Drop-In Filesystem Compatibility**
All standard MCP filesystem tools included:
- `list_directory` — Browse with `[FILE]`/`[DIR]` prefixes
- `directory_tree` — Compact tree view (10x more token-efficient than JSON)
- `create_directory` — Recursive directory creation
- `move_file` — Rename/move files and directories
- `write_file` — Raw UTF-8 writes (non-hashline)
- `read_multiple_files` — Batch reads in one operation

## Features at a Glance

| Feature                   | Benefit                                                                |
| ------------------------- | ---------------------------------------------------------------------- |
| **Hash-Anchored Editing** | Edits survive file changes — no more "line 42 doesn't match" errors    |
| **Content Verification**  | 6-character file hashing prevents race conditions                      |
| **AGENTS.md Support**     | Declarative access control — protect secrets, lock schemas, hide noise |
| **Fuzzy Line Matching**   | Finds the right line even after insertions/deletions                   |
| **9 MCP Tools**           | 3 hashline tools + 6 standard filesystem tools                         |
| **Zero Dependencies**     | Pure Rust, compiles to a single binary                                 |

## Installation

```bash
cargo build --release
```

The binary will be at `target/release/hashfile-mcp`.

## Quick Start

### 1. Configure Your MCP Client

Add to Claude Desktop or your MCP client config:

```json
{
  "mcpServers": {
    "hashfile": {
      "command": "/path/to/hashfile-mcp/target/release/hashfile-mcp"
    }
  }
}
```

### 2. (Optional) Add Access Control

Create `AGENTS.md` in your project root:

```markdown
---
forbidden:
  - "secrets/**"
  - "**/*.env"
  - ".git/**"

read_only:
  - "package-lock.json"
  - "Cargo.lock"
  - "schema.sql"

ignore:
  - "node_modules/**"
  - "target/**"
  - "**/*.generated.ts"
---

# Project Instructions

Your custom instructions for AI agents here...
```

### 3. Start Editing

The AI can now:
- ✅ Read and edit source files with hash-anchored precision
- ✅ Browse directories and create new files
- ❌ Cannot touch your secrets or `.env` files
- ❌ Cannot modify lock files or schemas
- 🙈 Won't see `node_modules` or build artifacts

## How Hashline Works

### The Problem
Traditional line-number editing fails when files change:
```
Agent: "Replace line 42"
Reality: Someone inserted 3 lines at the top
Result: Wrong line replaced! 💥
```

### The Solution
Content-anchored editing with hash verification:

1. **Read**: Each line gets a hash tag → `42:a3|const x = 1;`
2. **Edit**: Operations reference content, not just position → `"anchor": "42:a3"`
3. **Apply**: Server finds the line by hash, even if it moved to line 45
4. **Verify**: File hash must match — detects if content changed since read

### Reliability Features

- **File-Level Verification**: 6-character hash prevents editing stale content
- **Line-Level Anchoring**: 2-character hash identifies specific lines
- **Fuzzy Matching**: Searches for unique hash match if line number changed
- **Conflict Detection**: Clear errors if content diverged


## Core Tools

### Hashline Tools (Precision Editing)

#### `read_text_file`
Returns content with hash-tagged lines for reliable editing:

```
1:a3|import { useState } from 'react';
2:7f|
3:2c|export function Counter() {
---
hashline_version: 1
total_lines: 3
file_hash: 8f3a9b
```

#### `edit_text_file`
Apply hash-anchored operations:

```json
{
  "path": "/path/to/file.ts",
  "file_hash": "8f3a9b",
  "operations": [
    {
      "op_type": "replace",
      "anchor": "3:2c",
      "content": "export function Counter({ initial = 0 }) {"
    }
  ]
}
```

**Operation types**: `replace`, `insert_after`, `insert_before`, `delete`

#### `write_text_file`
Write content and get back hashline-tagged verification

### Filesystem Tools (Standard Operations)

#### `list_directory`
```
[DIR] src
[DIR] tests
[FILE] README.md
[FILE] Cargo.toml
```

#### `directory_tree`
```
src/
├── agents.rs
├── config.rs
├── filesystem.rs
├── hashline.rs
├── main.rs
└── tools.rs
```

Supports `exclude_patterns` for filtering (e.g., `["**/node_modules/**", "**/.git/**"]`)

#### `create_directory`, `move_file`, `write_file`, `read_multiple_files`
Standard filesystem operations with AGENTS.md enforcement

## AGENTS.md Access Control

### Constraint Types

| Constraint  | Effect                    | Use Case                               |
| ----------- | ------------------------- | -------------------------------------- |
| `forbidden` | Block all access          | Secrets, credentials, private keys     |
| `read_only` | Allow reads, block writes | Schemas, lock files, generated configs |
| `ignore`    | Hide from AI context      | Dependencies, build artifacts, noise   |

### Automatic .gitignore Support

Hashfile MCP automatically respects `.gitignore` files in your project:

- **Hierarchical**: Walks up from target file to find all `.gitignore` files
- **Standard syntax**: Supports standard `.gitignore` patterns (basename, directory, path)
- **Combined with AGENTS.md**: Patterns from both sources are merged
- **Zero config**: Works out of the box with existing projects

**Pattern conversion:**
```
# .gitignore
*.log              → **/*.log
node_modules/      → node_modules/**
build/output       → build/output
```

**Precedence:** AGENTS.md `ignore` patterns take precedence over `.gitignore`.

### Example: Protect a Monorepo

```yaml
---
forbidden:
  - "**/secrets/**"
  - "**/*.key"
  - "**/*.pem"
  - ".env*"

read_only:
  - "**/package-lock.json"
  - "**/Cargo.lock"
  - "db/schema.sql"

ignore:
  - "**/node_modules/**"
  - "**/target/**"
  - "**/.next/**"
  - "**/*.generated.*"
---
```

### Hierarchical Control

Place `AGENTS.md` files at any level:
```
/project/AGENTS.md          ← Global rules
/project/backend/AGENTS.md  ← Backend-specific rules (overrides global)
/project/frontend/AGENTS.md ← Frontend-specific rules
```

The **nearest** `AGENTS.md` in the directory hierarchy applies.

## Development

### Run Tests
```bash
cargo test
```

### Project Structure
```
src/
├── main.rs        # MCP server setup
├── tools.rs       # Tool definitions (9 tools)
├── hashline.rs    # Hash-anchored editing logic
├── filesystem.rs  # Standard filesystem operations
├── agents.rs      # AGENTS.md frontmatter parsing
├── config.rs      # Configuration (future: tool enablement)
└── roots.rs       # Root path management (future)
```

## Technical Details

- **Language**: Rust (edition 2021)
- **MCP SDK**: rmcp 0.15.0
- **Line Hashing**: FNV-1a (2 hex chars, 256 buckets)
- **File Hashing**: FNV-1a (6 hex chars, 16M buckets)
- **Transport**: stdio
- **Dependencies**: `rmcp`, `serde`, `serde_json`, `serde_yaml`, `globset`, `anyhow`, `fnv`

## Roadmap

- [ ] Environment variable-based tool enablement (`ENABLE_FILESYSTEM_TOOLS=true`)
- [ ] Granular tool control (`ENABLE_LIST_DIRECTORY=true`, etc.)
- [ ] Additional tools: `get_file_info`, `search_files`, `list_directory_with_sizes`
- [ ] Command-line argument support for configuration

## License

See LICENSE file for details.

## See Also

- [The Harness Problem](https://blog.can.ac/2026/02/12/the-harness-problem/) - I Improved 15 LLMs at Coding in One Afternoon. Only the Harness Changed. - Can Bölük Feb 2026
- [AGENTS.md Proposal](docs/agents.frontmatter.md) - Frontmatter specification for agent access control
