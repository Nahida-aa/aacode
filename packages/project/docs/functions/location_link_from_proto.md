---
id: location_link_from_proto
title: location_link_from_proto
---

# Function: location_link_from_proto()

```rust
pub fn location_link_from_proto(link: LocationLink, lsp_store: Entity<LspStore>, cx: &AsyncApp) -> Task<Result<LocationLink>>
```

Defined in: [`packages/project/src/lsp_command/mod.rs:1921`](../../../../packages/project/src/lsp_command/mod.rs#L1921)

## Parameters

### link

`LocationLink`

### lsp_store

`Entity<LspStore>`

### cx

`&AsyncApp`

## Returns

`Task<Result<LocationLink>>`

