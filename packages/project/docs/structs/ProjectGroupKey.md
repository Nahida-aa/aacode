---
id: ProjectGroupKey
title: ProjectGroupKey
---

# Struct: ProjectGroupKey

Defined in: [`packages/project/src/project/group_key.rs:16`](../../../../packages/project/src/project/group_key.rs#L16)

Identifies a project group by a set of paths the workspaces in this group
have.

Paths are mapped to their main worktree path first so we can group
workspaces by main repos.

## Definition

```rust
pub struct ProjectGroupKey
```

_（存在非公开字段）_

## Implementations

### new()

```rust
pub fn new(host: Option<RemoteConnectionOptions>, paths: PathList) -> Self
```

Defined in: [`packages/project/src/project/group_key.rs:26`](../../../../packages/project/src/project/group_key.rs#L26)

Creates a new `ProjectGroupKey` with the given path list.

The path list should point to the git main worktree paths for a project.

#### Parameters

##### host

`Option<RemoteConnectionOptions>`

##### paths

`PathList`

#### Returns

`Self`


***

### from_project()

```rust
pub fn from_project(project: &Project, cx: &App) -> Self
```

Defined in: [`packages/project/src/project/group_key.rs:30`](../../../../packages/project/src/project/group_key.rs#L30)

#### Parameters

##### project

`&Project`

##### cx

`&App`

#### Returns

`Self`


***

### from_worktree_paths()

```rust
pub fn from_worktree_paths(paths: &WorktreePaths, host: Option<RemoteConnectionOptions>) -> Self
```

Defined in: [`packages/project/src/project/group_key.rs:39`](../../../../packages/project/src/project/group_key.rs#L39)

#### Parameters

##### paths

`&WorktreePaths`

##### host

`Option<RemoteConnectionOptions>`

#### Returns

`Self`


***

### path_list()

```rust
pub fn path_list(&self) -> &PathList
```

Defined in: [`packages/project/src/project/group_key.rs:49`](../../../../packages/project/src/project/group_key.rs#L49)

#### Returns

`&PathList`


***

### display_name()

```rust
pub fn display_name(&self, path_detail_map: &HashMap<PathBuf, usize>) -> SharedString
```

Defined in: [`packages/project/src/project/group_key.rs:51`](../../../../packages/project/src/project/group_key.rs#L51)

#### Parameters

##### path_detail_map

`&HashMap<PathBuf, usize>`

#### Returns

`SharedString`


***

### host()

```rust
pub fn host(&self) -> Option<RemoteConnectionOptions>
```

Defined in: [`packages/project/src/project/group_key.rs:77`](../../../../packages/project/src/project/group_key.rs#L77)

#### Returns

`Option<RemoteConnectionOptions>`


***

### matches()

```rust
pub fn matches(&self, other: &ProjectGroupKey) -> bool
```

Defined in: [`packages/project/src/project/group_key.rs:79`](../../../../packages/project/src/project/group_key.rs#L79)

#### Parameters

##### other

`&ProjectGroupKey`

#### Returns

`bool`

## Trait Implementations

- `impl Borrow for ProjectGroupKey`
- `impl BorrowMut for ProjectGroupKey`
- `impl CloneToUninit for ProjectGroupKey`
- `impl Into for ProjectGroupKey`
- `impl From for ProjectGroupKey`
- `impl TryInto for ProjectGroupKey`
- `impl TryFrom for ProjectGroupKey`
- `impl Any for ProjectGroupKey`
- `impl ToOwned for ProjectGroupKey`
- `impl Equivalent for ProjectGroupKey`
- `impl DynClone for ProjectGroupKey`
- `impl VZip for ProjectGroupKey`
- `impl CastableFrom for ProjectGroupKey`
- `impl CastableFrom for ProjectGroupKey`
- `impl Read for ProjectGroupKey`
- `impl IntoEither for ProjectGroupKey`
- `impl ErasedDestructor for ProjectGroupKey`
- `impl Same for ProjectGroupKey`
- `impl ReadPrimitive for ProjectGroupKey`
- `impl Pointable for ProjectGroupKey`
- `impl Instrument for ProjectGroupKey`
- `impl WithSubscriber for ProjectGroupKey`
- `impl FromAngle for ProjectGroupKey`
- `impl IntoAngle for ProjectGroupKey`
- `impl IntoCam16Unclamped for ProjectGroupKey`
- `impl Cam16IntoUnclamped for ProjectGroupKey`
- `impl ArraysFrom for ProjectGroupKey`
- `impl ArraysInto for ProjectGroupKey`
- `impl ComponentsFrom for ProjectGroupKey`
- `impl TryComponentsInto for ProjectGroupKey`
- `impl UintsFrom for ProjectGroupKey`
- `impl UintsInto for ProjectGroupKey`
- `impl AdaptIntoUnclamped for ProjectGroupKey`
- `impl AdaptInto for ProjectGroupKey`
- `impl IntoColor for ProjectGroupKey`
- `impl IntoColorUnclamped for ProjectGroupKey`
- `impl TryIntoColor for ProjectGroupKey`
- `impl FromStimulus for ProjectGroupKey`
- `impl IntoStimulus for ProjectGroupKey`
- `impl Equivalent for ProjectGroupKey`
- `impl StructuralPartialEq for ProjectGroupKey`
- `impl PartialEq for ProjectGroupKey`
- `impl Eq for ProjectGroupKey`
- `impl Hash for ProjectGroupKey`
- `impl Clone for ProjectGroupKey`
- `impl Debug for ProjectGroupKey`
- `impl Default for ProjectGroupKey`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

