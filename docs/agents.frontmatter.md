
# Frontmatter Metadata for AGENTS.md

## Status

**Proposed — Ready for Review**

This document specifies an optional, backwards-compatible extension to the
`AGENTS.md` convention: YAML frontmatter for machine-readable metadata.

---

## Background and Motivation

Since the introduction of `AGENTS.md`, multiple tools and agent frameworks have
adopted it as a human-readable source of instructions. At the same time, agent
tooling has continued to rely on *separate, tool-specific configuration files*
to express constraints such as:

- Files an agent must not read
- Paths that must not be modified
- Sensitive or out-of-scope areas of a repository

Examples include `.cursorignore`, `.clineignore`, and similar files across
ecosystems.

A community discussion (raised approximately six months prior to this proposal)
identified YAML frontmatter as a natural way to express **structured metadata**
inside `AGENTS.md`, while preserving its Markdown-based, human-first design.

This proposal formalizes that idea.

---

## Goals

- Provide a **single, canonical place** for agent constraints and intent
- Enable **machine-parseable rules** without constraining natural language
- Remain **fully backwards compatible**
- Avoid coupling the specification to any single tool or vendor
- Support hierarchical and composable usage across directories

---

## Non-Goals

- Defining required fields or schemas for `AGENTS.md`
- Mandating behavior for any specific agent implementation
- Replacing all existing ignore or configuration files
- Introducing execution logic or imperative instructions

---

## Overview

`AGENTS.md` MAY include an optional YAML frontmatter block at the very top of the
file. When present, this frontmatter contains **declarative metadata** intended
for automated tooling.

Tools that do not recognize or support frontmatter MUST safely ignore it.

---

## Format

Frontmatter MUST:

- Appear at the very beginning of the file
- Be valid YAML
- Be delimited by triple-dash markers

Example:

```markdown
---
ignore:
  - "**/node_modules/**"
---

# Agent Instructions
````

---

## Defined Frontmatter Keys

### `ignore`

#### Purpose

Declares files or paths that an agent SHOULD NOT read, index, summarize, or  
reason about.

#### Semantics

- Treated as non-existent from the agent’s perspective
    
- The agent SHOULD NOT:
    
    - Open files
        
    - Reference contents
        
    - Infer behavior or structure
        
- Intended primarily for noise reduction and scope control
    

#### Type

```yaml
ignore:
  - <glob>
```

#### Example

```yaml
ignore:
  - "**/*.generated.ts"
  - "vendor/**"
```

---

### `read_only`

#### Purpose

Declares files or paths that an agent MAY read but MUST NOT modify.

#### Semantics

- Agent MAY:
    
    - Read
        
    - Reference
        
    - Summarize
        
- Agent MUST NOT:
    
    - Edit files
        
    - Generate patches
        
    - Propose inline modifications
        

This key explicitly distinguishes _visibility_ from _mutability_.

#### Type

```yaml
read_only:
  - <glob>
```

#### Example

```yaml
read_only:
  - "schemas/**"
  - "**/*.lock"
```

---

### `forbidden`

#### Purpose

Declares files or paths that an agent MUST NOT interact with in any way.

#### Semantics

- Stronger than `ignore`
    
- Agent MUST NOT:
    
    - Read
        
    - Modify
        
    - Summarize
        
    - Ask questions about
        
- Intended for hard security or policy boundaries
    

#### Type

```yaml
forbidden:
  - <glob>
```

#### Example

```yaml
forbidden:
  - "secrets/**"
  - "legal/**"
```

#### Notes

- `forbidden` is OPTIONAL
    
- Tools MAY choose to treat `forbidden` as equivalent to `ignore` if strict  
    enforcement is not supported
    

---

## Naming Rationale

### Why `ignore` instead of `forbidden` by default?

| Key         | Meaning                     | Typical Use Case       |
| ----------- | --------------------------- | ---------------------- |
| `ignore`    | Out of scope / invisible    | Noise, generated files |
| `read_only` | Visible but write-protected | Contracts, schemas     |
| `forbidden` | Explicitly prohibited       | Security, compliance   |

The proposal standardizes `ignore` and `read_only` as the primary mechanisms,  
with `forbidden` reserved for higher-assurance scenarios.

---

## Precedence and Hierarchy

- Multiple `AGENTS.md` files MAY exist in a repository
    
- Agents SHOULD apply the closest `AGENTS.md` in the directory tree
    
- Frontmatter keys are merged with the following precedence:
    
    1. Nearest file wins
        
    2. `forbidden` overrides `read_only`
        
    3. `ignore` overrides Markdown instructions
        

---

## Example: Complete AGENTS.md

```markdown
---
ignore:
  - "**/node_modules/**"
  - "**/*.generated.ts"

read_only:
  - "schemas/**"
  - "**/*.lock"

forbidden:
  - "secrets/**"
---

# Agent Instructions

## Scope

Focus on application logic only.

## Style

- Prefer small, composable functions
- Avoid introducing new dependencies
```

---

## Tooling Guidance (Non-Normative)

- Tools MAY:
    
    - Map `ignore` to internal ignore mechanisms
        
    - Enforce `read_only` as write guards
        
    - Treat `forbidden` as a hard failure
        
- Tools SHOULD:
    
    - Warn if frontmatter is present but unsupported
        
- Tools MUST NOT:
    
    - Interpret frontmatter as natural-language instructions
        

---

## Backwards Compatibility

- Existing `AGENTS.md` files remain valid
    
- Frontmatter is optional
    
- Tools that ignore frontmatter continue to function unchanged
    

---

## Future Extensions

This structure allows for future, optional keys such as:

- `applies_to`: agent or tool identifiers
    
- `capabilities`: feature opt-in/out flags
    
- `compliance`: regulatory annotations
    

These are intentionally out of scope for this proposal.
