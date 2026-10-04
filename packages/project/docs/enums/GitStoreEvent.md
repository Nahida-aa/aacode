---
id: GitStoreEvent
title: GitStoreEvent
---

# Enum: GitStoreEvent

Defined in: [`packages/project/src/git_store/mod.rs:852`](../../../../packages/project/src/git_store/mod.rs#L852)

## Definition

```rust
pub enum GitStoreEvent {
    ActiveRepositoryChanged(Option<RepositoryId>),
    // Bool is true when the repository that's updated is the active repository
    RepositoryUpdated(RepositoryId, RepositoryEvent, bool),
    RepositoryAdded,
    RepositoryRemoved(RepositoryId),
    IndexWriteError(Error),
    JobsUpdated,
    ConflictsUpdated,
    GlobalConfigurationUpdated,
    DiffBaseChanged(Option<RepositoryId>),
}
```

## Variants

### RepositoryUpdated

Defined in: [`packages/project/src/git_store/mod.rs:855`](../../../../packages/project/src/git_store/mod.rs#L855)

Bool is true when the repository that's updated is the active repository

## Trait Implementations

- `impl Borrow for GitStoreEvent`
- `impl BorrowMut for GitStoreEvent`
- `impl Into for GitStoreEvent`
- `impl From for GitStoreEvent`
- `impl TryInto for GitStoreEvent`
- `impl TryFrom for GitStoreEvent`
- `impl Any for GitStoreEvent`
- `impl VZip for GitStoreEvent`
- `impl CastableFrom for GitStoreEvent`
- `impl CastableFrom for GitStoreEvent`
- `impl Read for GitStoreEvent`
- `impl IntoEither for GitStoreEvent`
- `impl ErasedDestructor for GitStoreEvent`
- `impl Same for GitStoreEvent`
- `impl Pointable for GitStoreEvent`
- `impl Instrument for GitStoreEvent`
- `impl WithSubscriber for GitStoreEvent`
- `impl FromAngle for GitStoreEvent`
- `impl IntoAngle for GitStoreEvent`
- `impl IntoCam16Unclamped for GitStoreEvent`
- `impl Cam16IntoUnclamped for GitStoreEvent`
- `impl ArraysFrom for GitStoreEvent`
- `impl ArraysInto for GitStoreEvent`
- `impl ComponentsFrom for GitStoreEvent`
- `impl TryComponentsInto for GitStoreEvent`
- `impl UintsFrom for GitStoreEvent`
- `impl UintsInto for GitStoreEvent`
- `impl AdaptIntoUnclamped for GitStoreEvent`
- `impl AdaptInto for GitStoreEvent`
- `impl IntoColor for GitStoreEvent`
- `impl IntoColorUnclamped for GitStoreEvent`
- `impl TryIntoColor for GitStoreEvent`
- `impl FromStimulus for GitStoreEvent`
- `impl IntoStimulus for GitStoreEvent`
- `impl Debug for GitStoreEvent`
- `impl EventEmitter for GitStoreEvent`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

