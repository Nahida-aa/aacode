---
id: ConflictRegion
title: ConflictRegion
---

# Struct: ConflictRegion

Defined in: [`packages/project/src/git_store/conflict_set.rs:105`](../../../../packages/project/src/git_store/conflict_set.rs#L105)

## Definition

```rust
pub struct ConflictRegion
{
    pub ours_branch_name: SharedString,
    pub theirs_branch_name: SharedString,
    pub range: Range<Anchor>,
    pub ours: Range<Anchor>,
    pub theirs: Range<Anchor>,
    pub base: Option<Range<Anchor>>,
}
```

## Implementations

### resolve()

```rust
pub fn resolve(&self, buffer: Entity<Buffer>, ranges: &[Range<Anchor>], cx: &App)
```

Defined in: [`packages/project/src/git_store/conflict_set.rs:115`](../../../../packages/project/src/git_store/conflict_set.rs#L115)

#### Parameters

##### buffer

`Entity<Buffer>`

##### ranges

`&[Range<Anchor>]`

##### cx

`&App`

## Trait Implementations

- `impl Borrow for ConflictRegion`
- `impl BorrowMut for ConflictRegion`
- `impl CloneToUninit for ConflictRegion`
- `impl Into for ConflictRegion`
- `impl From for ConflictRegion`
- `impl TryInto for ConflictRegion`
- `impl TryFrom for ConflictRegion`
- `impl Any for ConflictRegion`
- `impl ToOwned for ConflictRegion`
- `impl Equivalent for ConflictRegion`
- `impl DynClone for ConflictRegion`
- `impl VZip for ConflictRegion`
- `impl CastableFrom for ConflictRegion`
- `impl CastableFrom for ConflictRegion`
- `impl Read for ConflictRegion`
- `impl IntoEither for ConflictRegion`
- `impl ErasedDestructor for ConflictRegion`
- `impl Same for ConflictRegion`
- `impl Pointable for ConflictRegion`
- `impl Instrument for ConflictRegion`
- `impl WithSubscriber for ConflictRegion`
- `impl FromAngle for ConflictRegion`
- `impl IntoAngle for ConflictRegion`
- `impl IntoCam16Unclamped for ConflictRegion`
- `impl Cam16IntoUnclamped for ConflictRegion`
- `impl ArraysFrom for ConflictRegion`
- `impl ArraysInto for ConflictRegion`
- `impl ComponentsFrom for ConflictRegion`
- `impl TryComponentsInto for ConflictRegion`
- `impl UintsFrom for ConflictRegion`
- `impl UintsInto for ConflictRegion`
- `impl AdaptIntoUnclamped for ConflictRegion`
- `impl AdaptInto for ConflictRegion`
- `impl IntoColor for ConflictRegion`
- `impl IntoColorUnclamped for ConflictRegion`
- `impl TryIntoColor for ConflictRegion`
- `impl FromStimulus for ConflictRegion`
- `impl IntoStimulus for ConflictRegion`
- `impl Equivalent for ConflictRegion`
- `impl Debug for ConflictRegion`
- `impl Clone for ConflictRegion`
- `impl StructuralPartialEq for ConflictRegion`
- `impl PartialEq for ConflictRegion`
- `impl Eq for ConflictRegion`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

