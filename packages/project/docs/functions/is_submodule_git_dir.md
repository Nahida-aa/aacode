---
id: is_submodule_git_dir
title: is_submodule_git_dir
---

# Function: is_submodule_git_dir()

```rust
pub fn is_submodule_git_dir(git_dir: &Path) -> bool
```

Defined in: [`packages/project/src/git_store/mod.rs:10899`](../../../../packages/project/src/git_store/mod.rs#L10899)

Returns true if `git_dir` is a Git submodule's git directory.

Submodules store their git directory inside the superproject at
`<superproject>/.git/modules/<name>`. Unlike a linked worktree, a submodule
is an independent project whose identity is its own working directory, so its
path must not be resolved to the superproject's `.git/modules/...` directory
(from which the working directory cannot be derived).

## Parameters

### git_dir

`&Path`

## Returns

`bool`

