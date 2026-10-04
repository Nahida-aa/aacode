---
id: ProjectPath
title: ProjectPath
---

# Struct: ProjectPath

Defined in: [`packages/project/src/path.rs:11`](../../../../packages/project/src/path.rs#L11)

## Definition

```rust
pub struct ProjectPath
{
    pub worktree_id: WorktreeId,
    pub path: Arc<RelPath>,
}
```

## Implementations

### from_file()

```rust
pub fn from_file(value: &dyn File, cx: &App) -> Self
```

Defined in: [`packages/project/src/path.rs:17`](../../../../packages/project/src/path.rs#L17)

#### Parameters

##### value

`&dyn File`

##### cx

`&App`

#### Returns

`Self`


***

### from_proto()

```rust
pub fn from_proto(p: ProjectPath) -> Option<Self>
```

Defined in: [`packages/project/src/path.rs:24`](../../../../packages/project/src/path.rs#L24)

#### Parameters

##### p

`ProjectPath`

#### Returns

`Option<Self>`


***

### to_proto()

```rust
pub fn to_proto(&self) -> ProjectPath
```

Defined in: [`packages/project/src/path.rs:31`](../../../../packages/project/src/path.rs#L31)

#### Returns

`ProjectPath`


***

### root_path()

```rust
pub fn root_path(worktree_id: WorktreeId) -> Self
```

Defined in: [`packages/project/src/path.rs:38`](../../../../packages/project/src/path.rs#L38)

#### Parameters

##### worktree_id

`WorktreeId`

#### Returns

`Self`


***

### starts_with()

```rust
pub fn starts_with(&self, other: &ProjectPath) -> bool
```

Defined in: [`packages/project/src/path.rs:45`](../../../../packages/project/src/path.rs#L45)

#### Parameters

##### other

`&ProjectPath`

#### Returns

`bool`

## Trait Implementations

- `impl Borrow for ProjectPath`
- `impl BorrowMut for ProjectPath`
- `impl CloneToUninit for ProjectPath`
- `impl Into for ProjectPath`
- `impl From for ProjectPath`
- `impl TryInto for ProjectPath`
- `impl TryFrom for ProjectPath`
- `impl Any for ProjectPath`
- `impl ToOwned for ProjectPath`
- `impl Equivalent for ProjectPath`
- `impl Comparable for ProjectPath`
- `impl DynClone for ProjectPath`
- `impl VZip for ProjectPath`
- `impl CastableFrom for ProjectPath`
- `impl CastableFrom for ProjectPath`
- `impl Read for ProjectPath`
- `impl IntoEither for ProjectPath`
- `impl ErasedDestructor for ProjectPath`
- `impl Same for ProjectPath`
- `impl Pointable for ProjectPath`
- `impl MapSeekTarget for ProjectPath`
- `impl Instrument for ProjectPath`
- `impl WithSubscriber for ProjectPath`
- `impl FromAngle for ProjectPath`
- `impl IntoAngle for ProjectPath`
- `impl IntoCam16Unclamped for ProjectPath`
- `impl Cam16IntoUnclamped for ProjectPath`
- `impl ArraysFrom for ProjectPath`
- `impl ArraysInto for ProjectPath`
- `impl ComponentsFrom for ProjectPath`
- `impl TryComponentsInto for ProjectPath`
- `impl UintsFrom for ProjectPath`
- `impl UintsInto for ProjectPath`
- `impl AdaptIntoUnclamped for ProjectPath`
- `impl AdaptInto for ProjectPath`
- `impl IntoColor for ProjectPath`
- `impl IntoColorUnclamped for ProjectPath`
- `impl TryIntoColor for ProjectPath`
- `impl FromStimulus for ProjectPath`
- `impl IntoStimulus for ProjectPath`
- `impl Equivalent for ProjectPath`
- `impl Clone for ProjectPath`
- `impl Debug for ProjectPath`
- `impl Eq for ProjectPath`
- `impl StructuralPartialEq for ProjectPath`
- `impl PartialEq for ProjectPath`
- `impl Hash for ProjectPath`
- `impl PartialOrd for ProjectPath`
- `impl Ord for ProjectPath`
- `impl From for ProjectPath`
- `impl From for ProjectPath`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

