---
id: ConflictSetUpdate
title: ConflictSetUpdate
---

# Struct: ConflictSetUpdate

Defined in: [`packages/project/src/git_store/conflict_set.rs:11`](../../../../packages/project/src/git_store/conflict_set.rs#L11)

## Definition

```rust
pub struct ConflictSetUpdate
{
    pub buffer_range: Option<Range<Anchor>>,
    pub old_range: Range<usize>,
    pub new_range: Range<usize>,
}
```

## Trait Implementations

- `impl Borrow for ConflictSetUpdate`
- `impl BorrowMut for ConflictSetUpdate`
- `impl CloneToUninit for ConflictSetUpdate`
- `impl Into for ConflictSetUpdate`
- `impl From for ConflictSetUpdate`
- `impl TryInto for ConflictSetUpdate`
- `impl TryFrom for ConflictSetUpdate`
- `impl Any for ConflictSetUpdate`
- `impl ToOwned for ConflictSetUpdate`
- `impl Equivalent for ConflictSetUpdate`
- `impl DynClone for ConflictSetUpdate`
- `impl VZip for ConflictSetUpdate`
- `impl CastableFrom for ConflictSetUpdate`
- `impl CastableFrom for ConflictSetUpdate`
- `impl Read for ConflictSetUpdate`
- `impl IntoEither for ConflictSetUpdate`
- `impl ErasedDestructor for ConflictSetUpdate`
- `impl Same for ConflictSetUpdate`
- `impl Pointable for ConflictSetUpdate`
- `impl Instrument for ConflictSetUpdate`
- `impl WithSubscriber for ConflictSetUpdate`
- `impl FromAngle for ConflictSetUpdate`
- `impl IntoAngle for ConflictSetUpdate`
- `impl IntoCam16Unclamped for ConflictSetUpdate`
- `impl Cam16IntoUnclamped for ConflictSetUpdate`
- `impl ArraysFrom for ConflictSetUpdate`
- `impl ArraysInto for ConflictSetUpdate`
- `impl ComponentsFrom for ConflictSetUpdate`
- `impl TryComponentsInto for ConflictSetUpdate`
- `impl UintsFrom for ConflictSetUpdate`
- `impl UintsInto for ConflictSetUpdate`
- `impl AdaptIntoUnclamped for ConflictSetUpdate`
- `impl AdaptInto for ConflictSetUpdate`
- `impl IntoColor for ConflictSetUpdate`
- `impl IntoColorUnclamped for ConflictSetUpdate`
- `impl TryIntoColor for ConflictSetUpdate`
- `impl FromStimulus for ConflictSetUpdate`
- `impl IntoStimulus for ConflictSetUpdate`
- `impl Equivalent for ConflictSetUpdate`
- `impl Clone for ConflictSetUpdate`
- `impl Debug for ConflictSetUpdate`
- `impl StructuralPartialEq for ConflictSetUpdate`
- `impl PartialEq for ConflictSetUpdate`
- `impl Eq for ConflictSetUpdate`
- `impl EventEmitter for ConflictSetUpdate`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

