---
id: resolve_git_worktree_to_main_repo
title: resolve_git_worktree_to_main_repo
---

# Function: resolve_git_worktree_to_main_repo()

```rust
pub async fn resolve_git_worktree_to_main_repo(fs: &dyn Fs, path: &Path) -> Option<PathBuf>
```

Defined in: [`packages/project/src/git_store/mod.rs:10731`](../../../../packages/project/src/git_store/mod.rs#L10731)

If `path` is a git linked worktree checkout, resolves it to the main
repository's identity path. For regular linked worktrees this is the main
repository's working directory; for linked worktrees backed by a bare repo
such as `.bare`, this is the parent project directory users think of as the
repository root. Returns `None` if `path` is a normal repository, not a git
repo, or if resolution fails.

Resolution works by:
1. Reading the `.git` file to get the `gitdir:` pointer
2. Following that to the worktree-specific git directory
3. Reading the `commondir` file to find the shared `.git` directory
4. Deriving the main repo's identity path from the common dir

## Parameters

### fs

`&dyn Fs`

### path

`&Path`

## Returns

`Option<PathBuf>`

