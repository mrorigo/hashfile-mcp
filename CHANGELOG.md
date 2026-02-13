# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2026-02-13

### Added
- `write_text_file` tool for creating/overwriting files with content
- Enhanced landing page with split-view demo section showing editorial content and terminal simulation
- Professional industrial color scheme (steel blue/slate) replacing purple theme
- Documentation files: `docs/IDEA.md`, `docs/build-mcp-server.md`, `docs/mcp-roots.md`

### Changed
- **BREAKING**: Simplified file hash from 64-character SHA-256 to 6-character FNV hash for better AI agent usability
- Updated landing page design with left-aligned code examples and improved typography
- Improved demo section with "How It Works" explanation and interactive terminal examples
- Updated all download links to v0.2.0

### Fixed
- Corrected Linux ARM64 musl release link in landing page

### Removed
- Unused SHA-256 and hex dependencies (now using FNV for all hashing)

## [0.1.0] - 2026-02-12

### Added
- Initial release of Hashfile MCP Server
- `read_text_file` tool with hashline-tagged content
- `edit_text_file` tool with hash-anchored operations
- FNV-1a hashing for line content (2 hex characters)
- SHA-256 hashing for file verification
- Fuzzy anchor matching when line numbers shift
- MCP protocol integration via rmcp 0.15.0
- Cross-platform release builds (macOS, Linux, Windows)
- Landing page with features, quick start, and downloads

[0.2.0]: https://github.com/mrorigo/hashfile-mcp/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/mrorigo/hashfile-mcp/releases/tag/v0.1.0
