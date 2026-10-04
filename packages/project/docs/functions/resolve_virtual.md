---
id: resolve_virtual
title: resolve_virtual
---

# Function: resolve_virtual()

```rust
pub fn resolve_virtual(path: &Path) -> Option<Arc<Path>>
```

Defined in: [`packages/project/src/yarn.rs:26`](../../../../packages/project/src/yarn.rs#L26)

Returns `None` when passed path is a malformed virtual path or it's not a virtual path at all.

## Parameters

### path

`&Path`

## Returns

`Option<Arc<Path>>`

