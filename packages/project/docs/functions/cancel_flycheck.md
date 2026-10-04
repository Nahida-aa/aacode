---
id: cancel_flycheck
title: cancel_flycheck
---

# Function: cancel_flycheck()

```rust
pub fn cancel_flycheck(project: Entity<Project>, buffer_path: Option<ProjectPath>, cx: &App) -> Task<Result<()>>
```

Defined in: [`packages/project/src/lsp_store/rust_analyzer_ext.rs:84`](../../../../packages/project/src/lsp_store/rust_analyzer_ext.rs#L84)

## Parameters

### project

`Entity<Project>`

### buffer_path

`Option<ProjectPath>`

### cx

`&App`

## Returns

`Task<Result<()>>`

