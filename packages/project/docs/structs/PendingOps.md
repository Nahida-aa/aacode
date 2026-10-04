---
id: PendingOps
title: PendingOps
---

# Struct: PendingOps

Defined in: [`packages/project/src/git_store/pending_op.rs:23`](../../../../packages/project/src/git_store/pending_op.rs#L23)

## Definition

```rust
pub struct PendingOps
{
    pub repo_path: RepoPath,
    pub ops: Vec<PendingOp>,
}
```

## Implementations

### new()

```rust
pub fn new(path: &RepoPath) -> Self
```

Defined in: [`packages/project/src/git_store/pending_op.rs:95`](../../../../packages/project/src/git_store/pending_op.rs#L95)

#### Parameters

##### path

`&RepoPath`

#### Returns

`Self`


***

### max_id()

```rust
pub fn max_id(&self) -> PendingOpId
```

Defined in: [`packages/project/src/git_store/pending_op.rs:102`](../../../../packages/project/src/git_store/pending_op.rs#L102)

#### Returns

[`PendingOpId`](PendingOpId.md)


***

### op_by_id()

```rust
pub fn op_by_id(&self, id: PendingOpId) -> Option<&PendingOp>
```

Defined in: [`packages/project/src/git_store/pending_op.rs:106`](../../../../packages/project/src/git_store/pending_op.rs#L106)

#### Parameters

##### id

[`PendingOpId`](PendingOpId.md)

#### Returns

`Option<&PendingOp>`


***

### op_by_id_mut()

```rust
pub fn op_by_id_mut(&self, id: PendingOpId) -> Option<&PendingOp>
```

Defined in: [`packages/project/src/git_store/pending_op.rs:110`](../../../../packages/project/src/git_store/pending_op.rs#L110)

#### Parameters

##### id

[`PendingOpId`](PendingOpId.md)

#### Returns

`Option<&PendingOp>`


***

### staged()

```rust
pub fn staged(&self) -> bool
```

Defined in: [`packages/project/src/git_store/pending_op.rs:115`](../../../../packages/project/src/git_store/pending_op.rs#L115)

File is staged if the last job is finished and has status Staged.

#### Returns

`bool`


***

### staging()

```rust
pub fn staging(&self) -> bool
```

Defined in: [`packages/project/src/git_store/pending_op.rs:125`](../../../../packages/project/src/git_store/pending_op.rs#L125)

File is staged if the last job is not finished and has status Staged.

#### Returns

`bool`


***

### last_op_errored()

```rust
pub fn last_op_errored(&self) -> bool
```

Defined in: [`packages/project/src/git_store/pending_op.rs:136`](../../../../packages/project/src/git_store/pending_op.rs#L136)

Checks whether the last operation in the pending operations resulted in
an error.

#### Returns

`bool`

## Trait Implementations

- `impl Borrow for PendingOps`
- `impl BorrowMut for PendingOps`
- `impl CloneToUninit for PendingOps`
- `impl Into for PendingOps`
- `impl From for PendingOps`
- `impl TryInto for PendingOps`
- `impl TryFrom for PendingOps`
- `impl Any for PendingOps`
- `impl ToOwned for PendingOps`
- `impl Equivalent for PendingOps`
- `impl DynClone for PendingOps`
- `impl VZip for PendingOps`
- `impl CastableFrom for PendingOps`
- `impl CastableFrom for PendingOps`
- `impl Read for PendingOps`
- `impl IntoEither for PendingOps`
- `impl ErasedDestructor for PendingOps`
- `impl Same for PendingOps`
- `impl Pointable for PendingOps`
- `impl Instrument for PendingOps`
- `impl WithSubscriber for PendingOps`
- `impl FromAngle for PendingOps`
- `impl IntoAngle for PendingOps`
- `impl IntoCam16Unclamped for PendingOps`
- `impl Cam16IntoUnclamped for PendingOps`
- `impl ArraysFrom for PendingOps`
- `impl ArraysInto for PendingOps`
- `impl ComponentsFrom for PendingOps`
- `impl TryComponentsInto for PendingOps`
- `impl UintsFrom for PendingOps`
- `impl UintsInto for PendingOps`
- `impl AdaptIntoUnclamped for PendingOps`
- `impl AdaptInto for PendingOps`
- `impl IntoColor for PendingOps`
- `impl IntoColorUnclamped for PendingOps`
- `impl TryIntoColor for PendingOps`
- `impl FromStimulus for PendingOps`
- `impl IntoStimulus for PendingOps`
- `impl Equivalent for PendingOps`
- `impl Clone for PendingOps`
- `impl Debug for PendingOps`
- `impl StructuralPartialEq for PendingOps`
- `impl PartialEq for PendingOps`
- `impl Eq for PendingOps`
- `impl Item for PendingOps`
- `impl KeyedItem for PendingOps`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

