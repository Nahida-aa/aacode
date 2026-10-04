---
id: location_link_from_lsp
title: location_link_from_lsp
---

# Function: location_link_from_lsp()

```rust
pub async fn location_link_from_lsp(link: LocationLink, lsp_store: &Entity<LspStore>, buffer: &Entity<Buffer>, server_id: LanguageServerId, cx: &AsyncApp) -> Result<LocationLink>
```

Defined in: [`packages/project/src/lsp_command/mod.rs:2118`](../../../../packages/project/src/lsp_command/mod.rs#L2118)

## Parameters

### link

`LocationLink`

### lsp_store

`&Entity<LspStore>`

### buffer

`&Entity<Buffer>`

### server_id

`LanguageServerId`

### cx

`&AsyncApp`

## Returns

`Result<LocationLink>`

