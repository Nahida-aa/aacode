---
id: linked_worktree_short_name
title: linked_worktree_short_name
---

# Function: linked_worktree_short_name()

```rust
pub fn linked_worktree_short_name(main_worktree_path: &Path, linked_worktree_path: &Path) -> Option<SharedString>
```

Defined in: [`packages/project/src/git_store/mod.rs:10918`](../../../../packages/project/src/git_store/mod.rs#L10918)

Returns a short name for a linked worktree suitable for UI display

Uses the main worktree path to come up with a short name that disambiguates
the linked worktree from the main worktree.

## Parameters

### main_worktree_path

`&Path`

### linked_worktree_path

`&Path`

## Returns

`Option<SharedString>`

