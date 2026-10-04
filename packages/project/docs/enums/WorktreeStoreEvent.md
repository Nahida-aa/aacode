---
id: WorktreeStoreEvent
title: WorktreeStoreEvent
---

# Enum: WorktreeStoreEvent

Defined in: [`packages/project/src/worktree_store.rs:222`](../../../../packages/project/src/worktree_store.rs#L222)

## Definition

```rust
pub enum WorktreeStoreEvent
{
    WorktreeAdded(Entity<Worktree>),
    WorktreeRemoved(EntityId, WorktreeId),
    WorktreeReleased(EntityId, WorktreeId),
    WorktreeOrderChanged,
    WorktreeUpdateSent(Entity<Worktree>),
    WorktreeUpdatedEntries(WorktreeId, UpdatedEntriesSet),
    WorktreeUpdatedGitRepositories(WorktreeId, UpdatedGitRepositoriesSet),
    WorktreeDeletedEntry(WorktreeId, ProjectEntryId),
    WorktreeUpdatedRootRepoCommonDir(WorktreeId),
}
```

## Trait Implementations

- `impl Borrow for WorktreeStoreEvent`
- `impl BorrowMut for WorktreeStoreEvent`
- `impl Into for WorktreeStoreEvent`
- `impl From for WorktreeStoreEvent`
- `impl TryInto for WorktreeStoreEvent`
- `impl TryFrom for WorktreeStoreEvent`
- `impl Any for WorktreeStoreEvent`
- `impl VZip for WorktreeStoreEvent`
- `impl CastableFrom for WorktreeStoreEvent`
- `impl CastableFrom for WorktreeStoreEvent`
- `impl Read for WorktreeStoreEvent`
- `impl IntoEither for WorktreeStoreEvent`
- `impl ErasedDestructor for WorktreeStoreEvent`
- `impl Same for WorktreeStoreEvent`
- `impl Pointable for WorktreeStoreEvent`
- `impl Instrument for WorktreeStoreEvent`
- `impl WithSubscriber for WorktreeStoreEvent`
- `impl FromAngle for WorktreeStoreEvent`
- `impl IntoAngle for WorktreeStoreEvent`
- `impl IntoCam16Unclamped for WorktreeStoreEvent`
- `impl Cam16IntoUnclamped for WorktreeStoreEvent`
- `impl ArraysFrom for WorktreeStoreEvent`
- `impl ArraysInto for WorktreeStoreEvent`
- `impl ComponentsFrom for WorktreeStoreEvent`
- `impl TryComponentsInto for WorktreeStoreEvent`
- `impl UintsFrom for WorktreeStoreEvent`
- `impl UintsInto for WorktreeStoreEvent`
- `impl AdaptIntoUnclamped for WorktreeStoreEvent`
- `impl AdaptInto for WorktreeStoreEvent`
- `impl IntoColor for WorktreeStoreEvent`
- `impl IntoColorUnclamped for WorktreeStoreEvent`
- `impl TryIntoColor for WorktreeStoreEvent`
- `impl FromStimulus for WorktreeStoreEvent`
- `impl IntoStimulus for WorktreeStoreEvent`
- `impl Debug for WorktreeStoreEvent`
- `impl EventEmitter for WorktreeStoreEvent`

## Auto Trait Implementations

`Freeze` `Send` `Sync` `Unpin` `UnsafeUnpin`

