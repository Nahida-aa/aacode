---
id: repo_identity_path_if_local
title: repo_identity_path_if_local
---

# Function: repo_identity_path_if_local()

```rust
pub fn repo_identity_path_if_local(common_dir: &Path, path_style: PathStyle) -> Option<&Path>
```

Defined in: [`packages/project/src/git_store/mod.rs:10888`](../../../../packages/project/src/git_store/mod.rs#L10888)

Returns the repository identity only when `std::path` can interpret the path correctly.

Callers whose downstream path operations are not yet `PathStyle`-aware use this to preserve
their existing behavior for foreign path styles.

## Parameters

### common_dir

`&Path`

### path_style

`PathStyle`

## Returns

`Option<&Path>`

