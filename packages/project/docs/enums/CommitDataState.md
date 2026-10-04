---
id: CommitDataState
title: CommitDataState
---

# Enum: CommitDataState

Defined in: [`packages/project/src/git_store/mod.rs:594`](../../../../packages/project/src/git_store/mod.rs#L594)

## Definition

```rust
pub enum CommitDataState
{
    Loading(Option<Shared<Receiver<Arc<CommitData>>>>),
    Loaded(Arc<CommitData>),
}
```

## Trait Implementations

- `impl Borrow for CommitDataState`
- `impl BorrowMut for CommitDataState`
- `impl CloneToUninit for CommitDataState`
- `impl Into for CommitDataState`
- `impl From for CommitDataState`
- `impl TryInto for CommitDataState`
- `impl TryFrom for CommitDataState`
- `impl Any for CommitDataState`
- `impl ToOwned for CommitDataState`
- `impl DynClone for CommitDataState`
- `impl VZip for CommitDataState`
- `impl CastableFrom for CommitDataState`
- `impl CastableFrom for CommitDataState`
- `impl Read for CommitDataState`
- `impl IntoEither for CommitDataState`
- `impl ErasedDestructor for CommitDataState`
- `impl Same for CommitDataState`
- `impl Pointable for CommitDataState`
- `impl Instrument for CommitDataState`
- `impl WithSubscriber for CommitDataState`
- `impl FromAngle for CommitDataState`
- `impl IntoAngle for CommitDataState`
- `impl IntoCam16Unclamped for CommitDataState`
- `impl Cam16IntoUnclamped for CommitDataState`
- `impl ArraysFrom for CommitDataState`
- `impl ArraysInto for CommitDataState`
- `impl ComponentsFrom for CommitDataState`
- `impl TryComponentsInto for CommitDataState`
- `impl UintsFrom for CommitDataState`
- `impl UintsInto for CommitDataState`
- `impl AdaptIntoUnclamped for CommitDataState`
- `impl AdaptInto for CommitDataState`
- `impl IntoColor for CommitDataState`
- `impl IntoColorUnclamped for CommitDataState`
- `impl TryIntoColor for CommitDataState`
- `impl FromStimulus for CommitDataState`
- `impl IntoStimulus for CommitDataState`
- `impl Clone for CommitDataState`

## Auto Trait Implementations

`Freeze` `Send` `Sync` `Unpin` `UnsafeUnpin`

