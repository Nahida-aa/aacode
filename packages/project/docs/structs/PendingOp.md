---
id: PendingOp
title: PendingOp
---

# Struct: PendingOp

Defined in: [`packages/project/src/git_store/pending_op.rs:29`](../../../../packages/project/src/git_store/pending_op.rs#L29)

## Definition

```rust
pub struct PendingOp {
    pub id: PendingOpId,
    pub git_status: GitStatus,
    pub job_status: JobStatus,
}
```

## Implementations

### running()

```rust
pub fn running(&self) -> bool
```

Defined in: [`packages/project/src/git_store/pending_op.rs:142`](../../../../packages/project/src/git_store/pending_op.rs#L142)

#### Returns

`bool`


***

### finished()

```rust
pub fn finished(&self) -> bool
```

Defined in: [`packages/project/src/git_store/pending_op.rs:146`](../../../../packages/project/src/git_store/pending_op.rs#L146)

#### Returns

`bool`


***

### error()

```rust
pub fn error(&self) -> bool
```

Defined in: [`packages/project/src/git_store/pending_op.rs:150`](../../../../packages/project/src/git_store/pending_op.rs#L150)

#### Returns

`bool`

## Trait Implementations

- `impl Borrow for PendingOp`
- `impl BorrowMut for PendingOp`
- `impl CloneToUninit for PendingOp`
- `impl Into for PendingOp`
- `impl From for PendingOp`
- `impl TryInto for PendingOp`
- `impl TryFrom for PendingOp`
- `impl Any for PendingOp`
- `impl ToOwned for PendingOp`
- `impl Equivalent for PendingOp`
- `impl DynClone for PendingOp`
- `impl VZip for PendingOp`
- `impl CastableFrom for PendingOp`
- `impl CastableFrom for PendingOp`
- `impl Read for PendingOp`
- `impl IntoEither for PendingOp`
- `impl ErasedDestructor for PendingOp`
- `impl Same for PendingOp`
- `impl Pointable for PendingOp`
- `impl Instrument for PendingOp`
- `impl WithSubscriber for PendingOp`
- `impl FromAngle for PendingOp`
- `impl IntoAngle for PendingOp`
- `impl IntoCam16Unclamped for PendingOp`
- `impl Cam16IntoUnclamped for PendingOp`
- `impl ArraysFrom for PendingOp`
- `impl ArraysInto for PendingOp`
- `impl ComponentsFrom for PendingOp`
- `impl TryComponentsInto for PendingOp`
- `impl UintsFrom for PendingOp`
- `impl UintsInto for PendingOp`
- `impl AdaptIntoUnclamped for PendingOp`
- `impl AdaptInto for PendingOp`
- `impl IntoColor for PendingOp`
- `impl IntoColorUnclamped for PendingOp`
- `impl TryIntoColor for PendingOp`
- `impl FromStimulus for PendingOp`
- `impl IntoStimulus for PendingOp`
- `impl Equivalent for PendingOp`
- `impl ResetDiscriminant for PendingOp`
- `impl Clone for PendingOp`
- `impl Copy for PendingOp`
- `impl Debug for PendingOp`
- `impl StructuralPartialEq for PendingOp`
- `impl PartialEq for PendingOp`
- `impl Eq for PendingOp`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

