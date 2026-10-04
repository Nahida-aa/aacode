---
id: register_schema_handler
title: register_schema_handler
---

# Function: register_schema_handler()

```rust
pub fn register_schema_handler(handler: fn fn(_: Entity<LspStore>, _: String, _: &AsyncApp) -> Task<Result<String>>, cx: &App)
```

Defined in: [`packages/project/src/lsp_store/json_language_server_ext.rs:30`](../../../../packages/project/src/lsp_store/json_language_server_ext.rs#L30)

## Parameters

### handler

`fn fn(_: Entity<LspStore>, _: String, _: &AsyncApp) -> Task<Result<String>>`

### cx

`&App`

