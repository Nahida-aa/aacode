---
id: MergeDetails
title: MergeDetails
---

# Struct: MergeDetails

Defined in: [`packages/project/src/git_store/mod.rs:588`](../../../../packages/project/src/git_store/mod.rs#L588)

## Definition

```rust
pub struct MergeDetails {
    pub merge_heads_by_conflicted_path: TreeMap<RepoPath, Vec<Option<SharedString>>>,
    pub message: Option<SharedString>,
}
```

## Trait Implementations

- `impl Borrow for MergeDetails`
- `impl BorrowMut for MergeDetails`
- `impl CloneToUninit for MergeDetails`
- `impl Into for MergeDetails`
- `impl From for MergeDetails`
- `impl TryInto for MergeDetails`
- `impl TryFrom for MergeDetails`
- `impl Any for MergeDetails`
- `impl ToOwned for MergeDetails`
- `impl Equivalent for MergeDetails`
- `impl DynClone for MergeDetails`
- `impl VZip for MergeDetails`
- `impl CastableFrom for MergeDetails`
- `impl CastableFrom for MergeDetails`
- `impl Read for MergeDetails`
- `impl IntoEither for MergeDetails`
- `impl ErasedDestructor for MergeDetails`
- `impl Same for MergeDetails`
- `impl ReadPrimitive for MergeDetails`
- `impl Pointable for MergeDetails`
- `impl Instrument for MergeDetails`
- `impl WithSubscriber for MergeDetails`
- `impl FromAngle for MergeDetails`
- `impl IntoAngle for MergeDetails`
- `impl IntoCam16Unclamped for MergeDetails`
- `impl Cam16IntoUnclamped for MergeDetails`
- `impl ArraysFrom for MergeDetails`
- `impl ArraysInto for MergeDetails`
- `impl ComponentsFrom for MergeDetails`
- `impl TryComponentsInto for MergeDetails`
- `impl UintsFrom for MergeDetails`
- `impl UintsInto for MergeDetails`
- `impl AdaptIntoUnclamped for MergeDetails`
- `impl AdaptInto for MergeDetails`
- `impl IntoColor for MergeDetails`
- `impl IntoColorUnclamped for MergeDetails`
- `impl TryIntoColor for MergeDetails`
- `impl FromStimulus for MergeDetails`
- `impl IntoStimulus for MergeDetails`
- `impl Equivalent for MergeDetails`
- `impl Clone for MergeDetails`
- `impl Debug for MergeDetails`
- `impl Default for MergeDetails`
- `impl StructuralPartialEq for MergeDetails`
- `impl PartialEq for MergeDetails`
- `impl Eq for MergeDetails`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

