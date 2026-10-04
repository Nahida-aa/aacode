---
id: run_flycheck
title: run_flycheck
---

# Function: run_flycheck()

```rust
pub fn run_flycheck(project: Entity<Project>, buffer_path: Option<ProjectPath>, cx: &App) -> Task<Result<()>>
```

Defined in: [`packages/project/src/lsp_store/rust_analyzer_ext.rs:133`](../../../../packages/project/src/lsp_store/rust_analyzer_ext.rs#L133)

## Parameters

### project

`Entity<Project>`

### buffer_path

`Option<ProjectPath>`

### cx

`&App`

## Returns

`Task<Result<()>>`

