---
id: clear_flycheck
title: clear_flycheck
---

# Function: clear_flycheck()

```rust
pub fn clear_flycheck(project: Entity<Project>, buffer_path: Option<ProjectPath>, cx: &App) -> Task<Result<()>>
```

Defined in: [`packages/project/src/lsp_store/rust_analyzer_ext.rs:190`](../../../../packages/project/src/lsp_store/rust_analyzer_ext.rs#L190)

## Parameters

### project

`Entity<Project>`

### buffer_path

`Option<ProjectPath>`

### cx

`&App`

## Returns

`Task<Result<()>>`

