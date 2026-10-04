---
id: repo_identity_path
title: repo_identity_path
---

# Function: repo_identity_path()

```rust
pub fn repo_identity_path(common_dir: &Path, path_style: PathStyle) -> &Path
```

Defined in: [`packages/project/src/git_store/mod.rs:10873`](../../../../packages/project/src/git_store/mod.rs#L10873)

Returns the repository's identity path given its common Git directory.

This is the canonical, on-disk path used for project grouping and as the
basis for display names. The goal is to return the directory the user
thinks of as "the project":

- If `common_dir`'s last component starts with `.` (e.g. `.git` for a
  normal checkout, or `.bare` for a bare clone), the parent directory is
  returned. Both of these are internal Git directories; the parent is the
  meaningful project root.
- Otherwise (e.g. `zed.git` for a bare clone), `common_dir` itself is
  returned — it is already a meaningful on-disk path.

## Parameters

### common_dir

`&Path`

### path_style

`PathStyle`

## Returns

`&Path`

