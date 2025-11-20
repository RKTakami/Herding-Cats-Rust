# MCP Servers Integration for Herding Cats Rust Development

## Overview

This document outlines the integration of two key MCP (Model Context Protocol) servers into the project's development workflow via [`[.roo/mcp.json](.roo/mcp.json)`]:

- **context7** (`npx -y @upstash/context7-mcp`): Provides up-to-date documentation, code snippets, and examples for any library/framework. Ideal for Rust crates research.
- **sequentialthinking** (`npx -y @modelcontextprotocol/server-sequential-thinking`): Facilitates dynamic, iterative Chain-of-Thought reasoning for complex planning, debugging, and architecture design.

These servers augment Roo AI's tools for efficient project development.

## Context7: Library Documentation

**When to use**: Before implementing features, check latest API/docs for crates like `tokio`, `iced`, `winit`.

### Step 1: Resolve Library ID
Always start here unless ID known.

```xml
<use_mcp_tool>
<server_name>context7</server_name>
<tool_name>resolve-library-id</tool_name>
<arguments>
{
  "libraryName": "tokio"
}
</arguments>
</use_mcp_tool>
```

**Example Response**: Select `/tokio-rs/tokio` (high benchmark score, comprehensive coverage).

### Step 2: Fetch Docs
```xml
<use_mcp_tool>
<server_name>context7</server_name>
<tool_name>get-library-docs</tool_name>
<arguments>
{
  "context7CompatibleLibraryID": "/tokio-rs/tokio",
  "topic": "tasks",
  "page": 1
}
</arguments>
</use_mcp_tool>
```

**Rust-Specific Examples**:
- `tokio`: `topic: "async runtime"` or `"spawn"` for task spawning.
- `iced`: `topic: "widgets"` for GUI button/menu impl.
- `serde`: `topic: "deserialize"` for JSON handling.

Iterate `page` if more needed.

## Sequential Thinking: Complex Reasoning

**When to use**: Architecture planning, bug diagnosis, multi-step refactors (e.g., GUI button menu connections).

### Iterative Workflow
Start with initial thought, continue until `nextThoughtNeeded: false`.

```xml
<use_mcp_tool>
<server_name>sequentialthinking</server_name>
<tool_name>sequentialthinking</tool_name>
<arguments>
{
  "thought": "Analyze current GUI button menu failures: review [BUTTON_MENU_FAILURE_ANALYSIS.md](BUTTON_MENU_FAILURE_ANALYSIS.md). Propose Rust-idiomatic fix using iced state management.",
  "nextThoughtNeeded": true,
  "thoughtNumber": 1,
  "totalThoughts": 6,
  "isRevision": false
}
</arguments>
</use_mcp_tool>
```

**Example for Rust GUI Architecture**:
- Thought 1: Decompose problem (e.g., state sync in `iced` app).
- Subsequent: Hypothesis, verify, revise if needed.
- Final: Concrete implementation plan.

## Verification Steps

1. Save `.roo/mcp.json` and **restart VS Code/Roo** to spawn servers.
2. Test Context7:
   - Resolve `tokio` → Get ID.
   - Docs on `"tasks"`.
3. Test Sequential Thinking:
   - Sample thought on project task.
4. Check console/terminals for server logs.

## Architecture Flow

```mermaid
graph LR
  A[Development Task<br/>e.g., Add Tokio Async] --> B{Need Docs?}
  B -->|Yes| C[resolve-library-id<br/>"tokio"]
  C --> D[get-library-docs<br/>ID + "tasks"]
  D --> E[Implement w/ Examples]
  B -->|No| F{Complex Plan?}
  F -->|Yes| G[sequentialthinking<br/>Thought 1: Decompose]
  G --> H[Iterate Thoughts → Solution]
  H --> E
  F -->|No| I[Direct Code/Debug]
