---
id: RepositorySnapshot
title: RepositorySnapshot
---

# Struct: RepositorySnapshot

Defined in: [`packages/project/src/git_store/mod.rs:600`](../../../../packages/project/src/git_store/mod.rs#L600)

## Definition

```rust
pub struct RepositorySnapshot
{
    pub id: RepositoryId,
    pub statuses_by_path: SumTree<StatusEntry>,
    pub work_directory_abs_path: Arc<Path>,
    pub dot_git_abs_path: Arc<Path>,
    // Absolute path to the directory holding this worktree's Git state. For a linked worktree this is the worktree-specific directory under the common Git directory, such as `<main>/.git/worktrees/<name>`.
    pub repository_dir_abs_path: Arc<Path>,
    // Absolute path to the repository's common Git directory. For a normal checkout this is `<work_directory>/.git`. For a linked worktree this is the common Git directory shared by all worktrees. If that common directory is a bare repository, there may be no main worktree path to derive from it.
    pub common_dir_abs_path: Arc<Path>,
    pub path_style: PathStyle,
    pub branch: Option<Branch>,
    pub branch_list: Arc<[Branch]>,
    pub branch_list_error: Option<SharedString>,
    pub head_commit: Option<CommitDetails>,
    pub scan_id: u64,
    pub merge: MergeDetails,
    pub remote_origin_url: Option<String>,
    pub remote_upstream_url: Option<String>,
    pub stash_entries: GitStash,
    pub linked_worktrees: Arc<[Worktree]>,
}
```

## Fields

### repository_dir_abs_path

Defined in: [`packages/project/src/git_store/mod.rs:609`](../../../../packages/project/src/git_store/mod.rs#L609)

Absolute path to the directory holding this worktree's Git state.

For a linked worktree this is the worktree-specific directory under the
common Git directory, such as `<main>/.git/worktrees/<name>`.


***

### common_dir_abs_path

Defined in: [`packages/project/src/git_store/mod.rs:616`](../../../../packages/project/src/git_store/mod.rs#L616)

Absolute path to the repository's common Git directory.

For a normal checkout this is `<work_directory>/.git`. For a linked
worktree this is the common Git directory shared by all worktrees. If
that common directory is a bare repository, there may be no main
worktree path to derive from it.

## Implementations

### main_worktree_abs_path()

```rust
pub fn main_worktree_abs_path(&self) -> Option<&Path>
```

Defined in: [`packages/project/src/git_store/mod.rs:6276`](../../../../packages/project/src/git_store/mod.rs#L6276)

Returns the main worktree path for this repository, if one exists.

Linked worktrees attached to bare repositories do not have a main
worktree. For linked worktrees attached to a non-bare repository, the
common Git directory is the main worktree's `.git` directory.

#### Returns

`Option<&Path>`


***

### is_main_worktree()

```rust
pub fn is_main_worktree(&self) -> bool
```

Defined in: [`packages/project/src/git_store/mod.rs:6295`](../../../../packages/project/src/git_store/mod.rs#L6295)

The main worktree is the original checkout that other worktrees were
created from.

For example, if you had both `~/code/zed` and `~/code/worktrees/zed-2`,
then `~/code/zed` is the main worktree and `~/code/worktrees/zed-2` is a linked worktree.

Submodules also return `true` here, since they are not linked worktrees.

#### Returns

`bool`


***

### is_linked_worktree()

```rust
pub fn is_linked_worktree(&self) -> bool
```

Defined in: [`packages/project/src/git_store/mod.rs:6303`](../../../../packages/project/src/git_store/mod.rs#L6303)

Returns true if this repository is a linked worktree, that is, one that
was created from another worktree.

Returns `false` for both the main worktree and submodules.

#### Returns

`bool`


***

### linked_worktrees()

```rust
pub fn linked_worktrees(&self) -> &[GitWorktree]
```

Defined in: [`packages/project/src/git_store/mod.rs:6307`](../../../../packages/project/src/git_store/mod.rs#L6307)

#### Returns

`&[GitWorktree]`


***

### status()

```rust
pub fn status(&self) -> impl ? + ?
```

Defined in: [`packages/project/src/git_store/mod.rs:6311`](../../../../packages/project/src/git_store/mod.rs#L6311)

#### Returns

`impl ? + ?`


***

### status_summary()

```rust
pub fn status_summary(&self) -> GitSummary
```

Defined in: [`packages/project/src/git_store/mod.rs:6315`](../../../../packages/project/src/git_store/mod.rs#L6315)

#### Returns

`GitSummary`


***

### status_for_path()

```rust
pub fn status_for_path(&self, path: &RepoPath) -> Option<StatusEntry>
```

Defined in: [`packages/project/src/git_store/mod.rs:6319`](../../../../packages/project/src/git_store/mod.rs#L6319)

#### Parameters

##### path

`&RepoPath`

#### Returns

`Option<StatusEntry>`


***

### diff_stat_for_path()

```rust
pub fn diff_stat_for_path(&self, path: &RepoPath) -> Option<DiffStat>
```

Defined in: [`packages/project/src/git_store/mod.rs:6325`](../../../../packages/project/src/git_store/mod.rs#L6325)

#### Parameters

##### path

`&RepoPath`

#### Returns

`Option<DiffStat>`


***

### abs_path_to_repo_path()

```rust
pub fn abs_path_to_repo_path(&self, abs_path: &Path) -> Option<RepoPath>
```

Defined in: [`packages/project/src/git_store/mod.rs:6331`](../../../../packages/project/src/git_store/mod.rs#L6331)

#### Parameters

##### abs_path

`&Path`

#### Returns

`Option<RepoPath>`


***

### repo_path_to_abs_path()

```rust
pub fn repo_path_to_abs_path(&self, repo_path: &RepoPath) -> PathBuf
```

Defined in: [`packages/project/src/git_store/mod.rs:6335`](../../../../packages/project/src/git_store/mod.rs#L6335)

#### Parameters

##### repo_path

`&RepoPath`

#### Returns

`PathBuf`


***

### had_conflict_on_last_merge_head_change()

```rust
pub fn had_conflict_on_last_merge_head_change(&self, repo_path: &RepoPath) -> bool
```

Defined in: [`packages/project/src/git_store/mod.rs:6354`](../../../../packages/project/src/git_store/mod.rs#L6354)

#### Parameters

##### repo_path

`&RepoPath`

#### Returns

`bool`


***

### has_conflict()

```rust
pub fn has_conflict(&self, repo_path: &RepoPath) -> bool
```

Defined in: [`packages/project/src/git_store/mod.rs:6360`](../../../../packages/project/src/git_store/mod.rs#L6360)

#### Parameters

##### repo_path

`&RepoPath`

#### Returns

`bool`


***

### display_name()

```rust
pub fn display_name(&self) -> SharedString
```

Defined in: [`packages/project/src/git_store/mod.rs:6372`](../../../../packages/project/src/git_store/mod.rs#L6372)

This is the name that will be displayed in the repository selector for this repository.

#### Returns

`SharedString`

## Trait Implementations

- `impl Borrow for RepositorySnapshot`
- `impl BorrowMut for RepositorySnapshot`
- `impl CloneToUninit for RepositorySnapshot`
- `impl Into for RepositorySnapshot`
- `impl From for RepositorySnapshot`
- `impl TryInto for RepositorySnapshot`
- `impl TryFrom for RepositorySnapshot`
- `impl Any for RepositorySnapshot`
- `impl ToOwned for RepositorySnapshot`
- `impl Equivalent for RepositorySnapshot`
- `impl DynClone for RepositorySnapshot`
- `impl VZip for RepositorySnapshot`
- `impl CastableFrom for RepositorySnapshot`
- `impl CastableFrom for RepositorySnapshot`
- `impl Read for RepositorySnapshot`
- `impl IntoEither for RepositorySnapshot`
- `impl ErasedDestructor for RepositorySnapshot`
- `impl Same for RepositorySnapshot`
- `impl Pointable for RepositorySnapshot`
- `impl Instrument for RepositorySnapshot`
- `impl WithSubscriber for RepositorySnapshot`
- `impl FromAngle for RepositorySnapshot`
- `impl IntoAngle for RepositorySnapshot`
- `impl IntoCam16Unclamped for RepositorySnapshot`
- `impl Cam16IntoUnclamped for RepositorySnapshot`
- `impl ArraysFrom for RepositorySnapshot`
- `impl ArraysInto for RepositorySnapshot`
- `impl ComponentsFrom for RepositorySnapshot`
- `impl TryComponentsInto for RepositorySnapshot`
- `impl UintsFrom for RepositorySnapshot`
- `impl UintsInto for RepositorySnapshot`
- `impl AdaptIntoUnclamped for RepositorySnapshot`
- `impl AdaptInto for RepositorySnapshot`
- `impl IntoColor for RepositorySnapshot`
- `impl IntoColorUnclamped for RepositorySnapshot`
- `impl TryIntoColor for RepositorySnapshot`
- `impl FromStimulus for RepositorySnapshot`
- `impl IntoStimulus for RepositorySnapshot`
- `impl Equivalent for RepositorySnapshot`
- `impl Clone for RepositorySnapshot`
- `impl Debug for RepositorySnapshot`
- `impl StructuralPartialEq for RepositorySnapshot`
- `impl PartialEq for RepositorySnapshot`
- `impl Eq for RepositorySnapshot`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

