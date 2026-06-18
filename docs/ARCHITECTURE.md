# Lucard Architecture

## Crate Overview

The workspace contains **10 crates** after consolidation.

---

## Binary Crate

### `lucard_main`
The CLI application entry point that produces the `lucard` executable.

**Responsibilities:**
- CLI argument parsing
- REPL loop and user input/output
- Update notifications
- Session management

**Why separate:** Binary crates must be separate from library crates in Rust.

---

## Core Library Crates

### `lucard_domain`
Core domain models and business logic primitives.

**Contains:**
- `Conversation`, `Message`, `Tool`, `Provider` types
- XML parsing for tool calls
- Policy and configuration models

**Why separate:** Pure data structures with no external IO. Foundation layer that all other crates depend on.

---

### `lucard_app`
Application logic and orchestration layer (~148 files).

**Responsibilities:**
- Agent execution and conversation flow
- Tool resolution and execution
- Prompt building and LLM interaction
- System prompt generation

**Why separate:** Contains the core "brain" logic. Depends on domain, services, and infra layers.

---

### `lucard_services`
Service layer with business logic implementations.

**Contains:**
- Provider authentication
- MCP client management
- Policy enforcement
- Context engine
- `tracker` module (analytics)
- `snaps` module (file snapshots)

**Why separate:** Stateful services between app layer and infrastructure.

---

### `lucard_infra`
Infrastructure layer for external IO operations.

**Handles:**
- HTTP clients
- File system operations
- Environment variables
- MCP server connections
- Authentication strategies

**Why separate:** Isolates IO operations making the rest of the codebase testable with mocks.

---

### `lucard_repo`
Repository pattern implementations for persistence.

**Manages:**
- Conversation history
- Agent configurations
- App settings
- File snapshots

**Why separate:** Clean abstraction over storage backends.

---

### `lucard_common`
Shared utilities used across multiple crates.

**Modules:**
- `spinner` - Terminal loading animations
- `display` - Markdown rendering, diffs
- `select` - Fuzzy selection UI
- `stream` - Async stream utilities
- `walker` - File system traversal
- `json_repair` - JSON error recovery
- `template` - HTML elements
- `test_kit` - Test fixtures
- `fs` - File operations

**Why separate:** Prevents code duplication across layers.

---

### `lucard_api`
External API definitions and client implementations.

**Handles:**
- Request/response transformations for LLM providers
- OpenAI, Anthropic API specifics

**Why separate:** Allows adding new providers without touching core logic.

---

### `lucard_ci`
CI/CD and workflow automation utilities.

**Contains:**
- GitHub Actions workflow generation

**Why separate:** Specialized tooling that doesn't belong in core app.

---

### `lucard_tool_macros`
Procedural macros for tool definitions.

**Contains:**
- `#[tool]` derive macro

**Why separate:** Proc-macro crates **must** be separate in Rust (language requirement).

---

## Architecture Diagram

```
┌─────────────────────────────────────────────────────┐
│                   lucard_main (binary)                │
├─────────────────────────────────────────────────────┤
│                      lucard_app                       │
├──────────────┬──────────────────┬───────────────────┤
│ lucard_services│    lucard_repo     │     lucard_api      │
├──────────────┴──────────────────┴───────────────────┤
│                    lucard_infra                       │
├─────────────────────────────────────────────────────┤
│                    lucard_domain                      │
├─────────────────────────────────────────────────────┤
│  lucard_common  │  lucard_tool_macros  │    lucard_ci     │
└───────────────┴────────────────────┴────────────────┘
```

## Dependency Flow

```
lucard_main
    ↓
lucard_app ──→ lucard_api
    ↓
lucard_services ──→ lucard_repo
    ↓
lucard_infra
    ↓
lucard_domain
    ↓
lucard_common ←── lucard_tool_macros
```
