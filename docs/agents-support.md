# AGENTS.md Frontmatter Support

Hashfile MCP Server supports the AGENTS.md frontmatter specification for declaring file access constraints.

## Overview

Place an `AGENTS.md` file in your project with YAML frontmatter to control which files the MCP server can access:

```markdown
---
forbidden:
  - "secrets/**"
  - "**/*.env"

read_only:
  - "schemas/**"
  - "**/*.lock"

ignore:
  - "node_modules/**"
  - "**/*.generated.ts"
---

# Agent Instructions

Your natural language instructions here...
```

## Constraint Types

### `forbidden`
**Blocks all access** - The server will refuse to read, write, or interact with these paths.

**Use for**: Sensitive data, credentials, legal documents

**Example error**: `Error: Access to secrets/key.pem is forbidden by AGENTS.md`

### `read_only`
**Allows reads, blocks writes** - The server can read and reference these files but cannot modify them.

**Use for**: Schemas, lock files, generated contracts

**Example error**: `Error: package.lock is read-only per AGENTS.md`

### `ignore`
**Treats as non-existent** - The server will not read or interact with these paths.

**Use for**: Build artifacts, dependencies, generated code

**Example error**: `Error: node_modules/lib.js is ignored by AGENTS.md`

## Glob Patterns

Supports standard glob syntax:
- `**/*.lock` - All .lock files recursively
- `secrets/**` - Everything under secrets/
- `**/private/**` - Any private directory at any level
- `{a,b}` - Match either a or b

## Precedence

When multiple constraints match:
1. `forbidden` overrides everything
2. `read_only` overrides `ignore`
3. `ignore` is the weakest constraint

## Hierarchical Discovery

The server walks up the directory tree from the target file to find the nearest `AGENTS.md`. Only the closest file is used (no merging).

## Example

```
project/
├── AGENTS.md          # Root constraints
├── src/
│   └── main.rs        # Allowed (no constraints)
├── secrets/
│   └── key.pem        # Forbidden
├── schemas/
│   └── api.json       # Read-only
└── node_modules/
    └── lib.js         # Ignored
```

With this setup:
- ✅ `read_text_file("src/main.rs")` - Works
- ✅ `read_text_file("schemas/api.json")` - Works
- ❌ `write_text_file("schemas/api.json", ...)` - Blocked (read-only)
- ❌ `read_text_file("secrets/key.pem")` - Blocked (forbidden)
- ❌ `read_text_file("node_modules/lib.js")` - Blocked (ignored)

## Specification

For the complete specification, see [docs/agents.frontmatted.md](agents.frontmatted.md).
