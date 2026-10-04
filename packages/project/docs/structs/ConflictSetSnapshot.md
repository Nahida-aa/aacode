---
id: ConflictSetSnapshot
title: ConflictSetSnapshot
---

# Struct: ConflictSetSnapshot

Defined in: [`packages/project/src/git_store/conflict_set.rs:18`](../../../../packages/project/src/git_store/conflict_set.rs#L18)

## Definition

```rust
pub struct ConflictSetSnapshot {
    pub buffer_id: BufferId,
    pub conflicts: Arc<[ConflictRegion]>,
}
```

## Implementations

### conflicts_in_range()

```rust
pub fn conflicts_in_range(&self, range: Range<Anchor>, buffer: &BufferSnapshot) -> &[ConflictRegion]
```

Defined in: [`packages/project/src/git_store/conflict_set.rs:24`](../../../../packages/project/src/git_store/conflict_set.rs#L24)

#### Parameters

##### range

`Range<Anchor>`

##### buffer

`&BufferSnapshot`

#### Returns

`&[ConflictRegion]`


***

### compare()

```rust
pub fn compare(&self, other: &Self, buffer: &BufferSnapshot) -> ConflictSetUpdate
```

Defined in: [`packages/project/src/git_store/conflict_set.rs:52`](../../../../packages/project/src/git_store/conflict_set.rs#L52)

#### Parameters

##### other

`&Self`

##### buffer

`&BufferSnapshot`

#### Returns

[`ConflictSetUpdate`](ConflictSetUpdate.md)

## Trait Implementations

- `impl Borrow for ConflictSetSnapshot`
- `impl BorrowMut for ConflictSetSnapshot`
- `impl CloneToUninit for ConflictSetSnapshot`
- `impl Into for ConflictSetSnapshot`
- `impl From for ConflictSetSnapshot`
- `impl TryInto for ConflictSetSnapshot`
- `impl TryFrom for ConflictSetSnapshot`
- `impl Any for ConflictSetSnapshot`
- `impl ToOwned for ConflictSetSnapshot`
- `impl DynClone for ConflictSetSnapshot`
- `impl VZip for ConflictSetSnapshot`
- `impl CastableFrom for ConflictSetSnapshot`
- `impl CastableFrom for ConflictSetSnapshot`
- `impl Read for ConflictSetSnapshot`
- `impl IntoEither for ConflictSetSnapshot`
- `impl ErasedDestructor for ConflictSetSnapshot`
- `impl Same for ConflictSetSnapshot`
- `impl Pointable for ConflictSetSnapshot`
- `impl Instrument for ConflictSetSnapshot`
- `impl WithSubscriber for ConflictSetSnapshot`
- `impl FromAngle for ConflictSetSnapshot`
- `impl IntoAngle for ConflictSetSnapshot`
- `impl IntoCam16Unclamped for ConflictSetSnapshot`
- `impl Cam16IntoUnclamped for ConflictSetSnapshot`
- `impl ArraysFrom for ConflictSetSnapshot`
- `impl ArraysInto for ConflictSetSnapshot`
- `impl ComponentsFrom for ConflictSetSnapshot`
- `impl TryComponentsInto for ConflictSetSnapshot`
- `impl UintsFrom for ConflictSetSnapshot`
- `impl UintsInto for ConflictSetSnapshot`
- `impl AdaptIntoUnclamped for ConflictSetSnapshot`
- `impl AdaptInto for ConflictSetSnapshot`
- `impl IntoColor for ConflictSetSnapshot`
- `impl IntoColorUnclamped for ConflictSetSnapshot`
- `impl TryIntoColor for ConflictSetSnapshot`
- `impl FromStimulus for ConflictSetSnapshot`
- `impl IntoStimulus for ConflictSetSnapshot`
- `impl Debug for ConflictSetSnapshot`
- `impl Clone for ConflictSetSnapshot`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

