---
id: ChildEntriesGitIter
title: ChildEntriesGitIter
---

# Struct: ChildEntriesGitIter

Defined in: [`packages/project/src/git_store/git_traversal.rs:159`](../../../../packages/project/src/git_store/git_traversal.rs#L159)

## Definition

```rust
pub struct ChildEntriesGitIter<'a>
```

_（存在非公开字段）_

## Implementations

### new()

```rust
pub fn new(repo_snapshots: &'a HashMap<RepositoryId, RepositorySnapshot>, worktree_snapshot: &'a Snapshot, parent_path: &'a RelPath) -> Self
```

Defined in: [`packages/project/src/git_store/git_traversal.rs:165`](../../../../packages/project/src/git_store/git_traversal.rs#L165)

#### Parameters

##### repo_snapshots

`&'a HashMap<RepositoryId, RepositorySnapshot>`

##### worktree_snapshot

`&'a Snapshot`

##### parent_path

`&'a RelPath`

#### Returns

`Self`

## Trait Implementations

- `impl Borrow for ChildEntriesGitIter`
- `impl BorrowMut for ChildEntriesGitIter`
- `impl Into for ChildEntriesGitIter`
- `impl From for ChildEntriesGitIter`
- `impl TryInto for ChildEntriesGitIter`
- `impl TryFrom for ChildEntriesGitIter`
- `impl Any for ChildEntriesGitIter`
- `impl IntoIterator for ChildEntriesGitIter`
- `impl IteratorRandom for ChildEntriesGitIter`
- `impl VZip for ChildEntriesGitIter`
- `impl CastableFrom for ChildEntriesGitIter`
- `impl CastableFrom for ChildEntriesGitIter`
- `impl Read for ChildEntriesGitIter`
- `impl Itertools for ChildEntriesGitIter`
- `impl IntoEither for ChildEntriesGitIter`
- `impl ErasedDestructor for ChildEntriesGitIter`
- `impl Same for ChildEntriesGitIter`
- `impl TakeUntilExt for ChildEntriesGitIter`
- `impl Pointable for ChildEntriesGitIter`
- `impl Instrument for ChildEntriesGitIter`
- `impl WithSubscriber for ChildEntriesGitIter`
- `impl IntoStreamingIterator for ChildEntriesGitIter`
- `impl TryCollect for ChildEntriesGitIter`
- `impl IteratorRandom for ChildEntriesGitIter`
- `impl FromAngle for ChildEntriesGitIter`
- `impl IntoAngle for ChildEntriesGitIter`
- `impl IntoCam16Unclamped for ChildEntriesGitIter`
- `impl Cam16IntoUnclamped for ChildEntriesGitIter`
- `impl ArraysFrom for ChildEntriesGitIter`
- `impl ArraysInto for ChildEntriesGitIter`
- `impl ComponentsFrom for ChildEntriesGitIter`
- `impl TryComponentsInto for ChildEntriesGitIter`
- `impl UintsFrom for ChildEntriesGitIter`
- `impl UintsInto for ChildEntriesGitIter`
- `impl AdaptIntoUnclamped for ChildEntriesGitIter`
- `impl AdaptInto for ChildEntriesGitIter`
- `impl IntoColor for ChildEntriesGitIter`
- `impl IntoColorUnclamped for ChildEntriesGitIter`
- `impl TryIntoColor for ChildEntriesGitIter`
- `impl FromStimulus for ChildEntriesGitIter`
- `impl IntoStimulus for ChildEntriesGitIter`
- `impl Iterator for ChildEntriesGitIter`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

