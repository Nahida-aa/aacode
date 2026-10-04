---
id: worktrees_directory_for_repo
title: worktrees_directory_for_repo
---

# Function: worktrees_directory_for_repo()

```rust
pub fn worktrees_directory_for_repo(repository_anchor_path: &Path, worktree_directory_setting: &str, path_style: PathStyle) -> Result<PathBuf>
```

Defined in: [`packages/project/src/git_store/mod.rs:10762`](../../../../packages/project/src/git_store/mod.rs#L10762)

Validates that the resolved worktree directory is acceptable:
- The setting must not be an absolute path.
- The resolved path must be either a subdirectory of the working
  directory or a subdirectory of its parent (i.e., a sibling).

Returns `Ok(resolved_path)` or an error with a user-facing message.

## Parameters

### repository_anchor_path

`&Path`

### worktree_directory_setting

`&str`

### path_style

`PathStyle`

## Returns

`Result<PathBuf>`

