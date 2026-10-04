---
id: location_links_from_proto
title: location_links_from_proto
---

# Function: location_links_from_proto()

```rust
pub async fn location_links_from_proto(proto_links: Vec<LocationLink>, lsp_store: Entity<LspStore>, cx: AsyncApp) -> Result<Vec<LocationLink>>
```

Defined in: [`packages/project/src/lsp_command/mod.rs:1907`](../../../../packages/project/src/lsp_command/mod.rs#L1907)

## Parameters

### proto_links

`Vec<LocationLink>`

### lsp_store

`Entity<LspStore>`

### cx

`AsyncApp`

## Returns

`Result<Vec<LocationLink>>`

