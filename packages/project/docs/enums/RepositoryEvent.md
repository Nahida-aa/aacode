---
id: RepositoryEvent
title: RepositoryEvent
---

# Enum: RepositoryEvent

Defined in: [`packages/project/src/git_store/mod.rs:838`](../../../../packages/project/src/git_store/mod.rs#L838)

## Definition

```rust
pub enum RepositoryEvent
{
    StatusesChanged,
    HeadChanged,
    BranchListChanged,
    StashEntriesChanged,
    GitWorktreeListChanged,
    PendingOpsChanged{ .. },
    GraphEvent((LogSource, LogOrder), GitGraphEvent),
}
```

## Trait Implementations

- `impl Borrow for RepositoryEvent`
- `impl BorrowMut for RepositoryEvent`
- `impl CloneToUninit for RepositoryEvent`
- `impl Into for RepositoryEvent`
- `impl From for RepositoryEvent`
- `impl TryInto for RepositoryEvent`
- `impl TryFrom for RepositoryEvent`
- `impl Any for RepositoryEvent`
- `impl ToOwned for RepositoryEvent`
- `impl Equivalent for RepositoryEvent`
- `impl DynClone for RepositoryEvent`
- `impl VZip for RepositoryEvent`
- `impl CastableFrom for RepositoryEvent`
- `impl CastableFrom for RepositoryEvent`
- `impl Read for RepositoryEvent`
- `impl IntoEither for RepositoryEvent`
- `impl ErasedDestructor for RepositoryEvent`
- `impl Same for RepositoryEvent`
- `impl Pointable for RepositoryEvent`
- `impl Instrument for RepositoryEvent`
- `impl WithSubscriber for RepositoryEvent`
- `impl FromAngle for RepositoryEvent`
- `impl IntoAngle for RepositoryEvent`
- `impl IntoCam16Unclamped for RepositoryEvent`
- `impl Cam16IntoUnclamped for RepositoryEvent`
- `impl ArraysFrom for RepositoryEvent`
- `impl ArraysInto for RepositoryEvent`
- `impl ComponentsFrom for RepositoryEvent`
- `impl TryComponentsInto for RepositoryEvent`
- `impl UintsFrom for RepositoryEvent`
- `impl UintsInto for RepositoryEvent`
- `impl AdaptIntoUnclamped for RepositoryEvent`
- `impl AdaptInto for RepositoryEvent`
- `impl IntoColor for RepositoryEvent`
- `impl IntoColorUnclamped for RepositoryEvent`
- `impl TryIntoColor for RepositoryEvent`
- `impl FromStimulus for RepositoryEvent`
- `impl IntoStimulus for RepositoryEvent`
- `impl Equivalent for RepositoryEvent`
- `impl Clone for RepositoryEvent`
- `impl Debug for RepositoryEvent`
- `impl StructuralPartialEq for RepositoryEvent`
- `impl PartialEq for RepositoryEvent`
- `impl Eq for RepositoryEvent`
- `impl EventEmitter for RepositoryEvent`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

