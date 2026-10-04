---
id: PathTrust
title: PathTrust
---

# Enum: PathTrust

Defined in: [`packages/project/src/trusted_worktrees.rs:190`](../../../../packages/project/src/trusted_worktrees.rs#L190)

A unit of trust consideration inside a particular host:
either a familiar worktree, or a path that may influence other worktrees' trust.
See module-level documentation on the trust model.

## Definition

```rust
pub enum PathTrust {
    // A worktree that is familiar to this workspace. Either a single file or a directory worktree.
    Worktree(WorktreeId),
    // A path that may be another worktree yet not loaded into any workspace (hence, without any `WorktreeId`), or a parent path coming out of the security modal.
    AbsPath(PathBuf),
}
```

## Variants

### Worktree

Defined in: [`packages/project/src/trusted_worktrees.rs:193`](../../../../packages/project/src/trusted_worktrees.rs#L193)

A worktree that is familiar to this workspace.
Either a single file or a directory worktree.


***

### AbsPath

Defined in: [`packages/project/src/trusted_worktrees.rs:196`](../../../../packages/project/src/trusted_worktrees.rs#L196)

A path that may be another worktree yet not loaded into any workspace (hence, without any `WorktreeId`),
or a parent path coming out of the security modal.

## Implementations

### from_proto()

```rust
pub fn from_proto(proto: PathTrust) -> Option<Self>
```

Defined in: [`packages/project/src/trusted_worktrees.rs:215`](../../../../packages/project/src/trusted_worktrees.rs#L215)

#### Parameters

##### proto

`PathTrust`

#### Returns

`Option<Self>`

## Trait Implementations

- `impl Borrow for PathTrust`
- `impl BorrowMut for PathTrust`
- `impl CloneToUninit for PathTrust`
- `impl Into for PathTrust`
- `impl From for PathTrust`
- `impl TryInto for PathTrust`
- `impl TryFrom for PathTrust`
- `impl Any for PathTrust`
- `impl ToOwned for PathTrust`
- `impl Equivalent for PathTrust`
- `impl DynClone for PathTrust`
- `impl VZip for PathTrust`
- `impl CastableFrom for PathTrust`
- `impl CastableFrom for PathTrust`
- `impl Read for PathTrust`
- `impl IntoEither for PathTrust`
- `impl ErasedDestructor for PathTrust`
- `impl Same for PathTrust`
- `impl Pointable for PathTrust`
- `impl Instrument for PathTrust`
- `impl WithSubscriber for PathTrust`
- `impl FromAngle for PathTrust`
- `impl IntoAngle for PathTrust`
- `impl IntoCam16Unclamped for PathTrust`
- `impl Cam16IntoUnclamped for PathTrust`
- `impl ArraysFrom for PathTrust`
- `impl ArraysInto for PathTrust`
- `impl ComponentsFrom for PathTrust`
- `impl TryComponentsInto for PathTrust`
- `impl UintsFrom for PathTrust`
- `impl UintsInto for PathTrust`
- `impl AdaptIntoUnclamped for PathTrust`
- `impl AdaptInto for PathTrust`
- `impl IntoColor for PathTrust`
- `impl IntoColorUnclamped for PathTrust`
- `impl TryIntoColor for PathTrust`
- `impl FromStimulus for PathTrust`
- `impl IntoStimulus for PathTrust`
- `impl Equivalent for PathTrust`
- `impl Debug for PathTrust`
- `impl StructuralPartialEq for PathTrust`
- `impl PartialEq for PathTrust`
- `impl Eq for PathTrust`
- `impl Clone for PathTrust`
- `impl Hash for PathTrust`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

