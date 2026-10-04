---
id: ContextServerFactory
title: ContextServerFactory
---

# Type Alias: ContextServerFactory

Defined in: [`packages/project/src/context_server_store/mod.rs:267`](../../../../packages/project/src/context_server_store/mod.rs#L267)

## Definition

```rust
pub type ContextServerFactory = Box<dyn Fn(ContextServerId, Arc<ContextServerConfiguration>) -> Arc<ContextServer>>
```

