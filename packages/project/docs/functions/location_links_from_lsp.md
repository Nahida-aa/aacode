---
id: location_links_from_lsp
title: location_links_from_lsp
---

# Function: location_links_from_lsp()

```rust
pub async fn location_links_from_lsp(message: Option<GotoDefinitionResponse>, lsp_store: Entity<LspStore>, buffer: Entity<Buffer>, server_id: LanguageServerId, cx: AsyncApp) -> Result<Vec<LocationLink>>
```

Defined in: [`packages/project/src/lsp_command/mod.rs:1980`](../../../../packages/project/src/lsp_command/mod.rs#L1980)

## Parameters

### message

`Option<GotoDefinitionResponse>`

### lsp_store

`Entity<LspStore>`

### buffer

`Entity<Buffer>`

### server_id

`LanguageServerId`

### cx

`AsyncApp`

## Returns

`Result<Vec<LocationLink>>`

