---
id: WorktreePaths
title: WorktreePaths
---

# Struct: WorktreePaths

Defined in: [`packages/project/src/worktree_store.rs:46`](../../../../packages/project/src/worktree_store.rs#L46)

The current paths for a project's worktrees. Each folder path has a corresponding
main worktree path at the same position. The two lists are always the
same length and are modified together via `add_path` / `remove_main_path`.

For non-linked worktrees, the main path and folder path are identical.
For linked worktrees, the main path is the original repo and the folder
path is the linked worktree location.

## Definition

```rust
pub struct WorktreePaths
```

_（存在非公开字段）_

## Implementations

### from_path_lists()

```rust
pub fn from_path_lists(main_worktree_paths: PathList, folder_paths: PathList) -> Result<Self>
```

Defined in: [`packages/project/src/worktree_store.rs:63`](../../../../packages/project/src/worktree_store.rs#L63)

Build from two parallel `PathList`s that already share the same
insertion order. Used for deserialization from DB.

Returns an error if the two lists have different lengths, which
indicates corrupted data from a prior migration bug.

#### Parameters

##### main_worktree_paths

`PathList`

##### folder_paths

`PathList`

#### Returns

`Result<Self>`


***

### from_folder_paths()

```rust
pub fn from_folder_paths(folder_paths: &PathList) -> Self
```

Defined in: [`packages/project/src/worktree_store.rs:80`](../../../../packages/project/src/worktree_store.rs#L80)

Build for non-linked worktrees where main == folder for every path.

#### Parameters

##### folder_paths

`&PathList`

#### Returns

`Self`


***

### is_empty()

```rust
pub fn is_empty(&self) -> bool
```

Defined in: [`packages/project/src/worktree_store.rs:87`](../../../../packages/project/src/worktree_store.rs#L87)

#### Returns

`bool`


***

### folder_path_list()

```rust
pub fn folder_path_list(&self) -> &PathList
```

Defined in: [`packages/project/src/worktree_store.rs:92`](../../../../packages/project/src/worktree_store.rs#L92)

The folder paths (for workspace matching / `threads_by_paths` index).

#### Returns

`&PathList`


***

### main_worktree_path_list()

```rust
pub fn main_worktree_path_list(&self) -> &PathList
```

Defined in: [`packages/project/src/worktree_store.rs:97`](../../../../packages/project/src/worktree_store.rs#L97)

The main worktree paths (for group key / `threads_by_main_paths` index).

#### Returns

`&PathList`


***

### ordered_pairs()

```rust
pub fn ordered_pairs(&self) -> impl ?
```

Defined in: [`packages/project/src/worktree_store.rs:102`](../../../../packages/project/src/worktree_store.rs#L102)

Iterate the (main_worktree_path, folder_path) pairs in insertion order.

#### Returns

`impl ?`


***

### add_path()

```rust
pub fn add_path(&self, main_path: &Path, folder_path: &Path)
```

Defined in: [`packages/project/src/worktree_store.rs:111`](../../../../packages/project/src/worktree_store.rs#L111)

Add a new path pair. If the exact (main, folder) pair already exists,
this is a no-op. Rebuilds both internal `PathList`s to maintain
consistent ordering.

#### Parameters

##### main_path

`&Path`

##### folder_path

`&Path`


***

### remove_main_path()

```rust
pub fn remove_main_path(&self, main_path: &Path)
```

Defined in: [`packages/project/src/worktree_store.rs:130`](../../../../packages/project/src/worktree_store.rs#L130)

Remove all pairs whose main worktree path matches the given path.
This removes the corresponding entries from both lists.

#### Parameters

##### main_path

`&Path`


***

### remove_folder_path()

```rust
pub fn remove_folder_path(&self, folder_path: &Path)
```

Defined in: [`packages/project/src/worktree_store.rs:142`](../../../../packages/project/src/worktree_store.rs#L142)

Remove all pairs whose folder path matches the given path.
This removes the corresponding entries from both lists.

#### Parameters

##### folder_path

`&Path`

## Trait Implementations

- `impl Borrow for WorktreePaths`
- `impl BorrowMut for WorktreePaths`
- `impl CloneToUninit for WorktreePaths`
- `impl Into for WorktreePaths`
- `impl From for WorktreePaths`
- `impl TryInto for WorktreePaths`
- `impl TryFrom for WorktreePaths`
- `impl Any for WorktreePaths`
- `impl ToOwned for WorktreePaths`
- `impl DynClone for WorktreePaths`
- `impl VZip for WorktreePaths`
- `impl CastableFrom for WorktreePaths`
- `impl CastableFrom for WorktreePaths`
- `impl Read for WorktreePaths`
- `impl IntoEither for WorktreePaths`
- `impl ErasedDestructor for WorktreePaths`
- `impl Same for WorktreePaths`
- `impl ReadPrimitive for WorktreePaths`
- `impl Pointable for WorktreePaths`
- `impl Instrument for WorktreePaths`
- `impl WithSubscriber for WorktreePaths`
- `impl FromAngle for WorktreePaths`
- `impl IntoAngle for WorktreePaths`
- `impl IntoCam16Unclamped for WorktreePaths`
- `impl Cam16IntoUnclamped for WorktreePaths`
- `impl ArraysFrom for WorktreePaths`
- `impl ArraysInto for WorktreePaths`
- `impl ComponentsFrom for WorktreePaths`
- `impl TryComponentsInto for WorktreePaths`
- `impl UintsFrom for WorktreePaths`
- `impl UintsInto for WorktreePaths`
- `impl AdaptIntoUnclamped for WorktreePaths`
- `impl AdaptInto for WorktreePaths`
- `impl IntoColor for WorktreePaths`
- `impl IntoColorUnclamped for WorktreePaths`
- `impl TryIntoColor for WorktreePaths`
- `impl FromStimulus for WorktreePaths`
- `impl IntoStimulus for WorktreePaths`
- `impl Default for WorktreePaths`
- `impl Debug for WorktreePaths`
- `impl Clone for WorktreePaths`
- `impl PartialEq for WorktreePaths`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

