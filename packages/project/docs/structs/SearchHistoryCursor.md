---
id: SearchHistoryCursor
title: SearchHistoryCursor
---

# Struct: SearchHistoryCursor

Defined in: [`packages/project/src/search_history.rs:20`](../../../../packages/project/src/search_history.rs#L20)

A cursor that stores an index to the currently selected query in the search history.
This can be passed to the search history to update the selection accordingly,
e.g. when using the up and down arrow keys to navigate the search history.

Note: The cursor can point to the wrong query, if the maximum length of the history is exceeded
and the old query is overwritten.

## Definition

```rust
pub struct SearchHistoryCursor
```

_（存在非公开字段）_

## Implementations

### reset()

```rust
pub fn reset(&self)
```

Defined in: [`packages/project/src/search_history.rs:27`](../../../../packages/project/src/search_history.rs#L27)

Resets the selection to `None` and clears the draft.


***

### take_draft()

```rust
pub fn take_draft(&self) -> Option<String>
```

Defined in: [`packages/project/src/search_history.rs:33`](../../../../packages/project/src/search_history.rs#L33)

Takes the stored draft query, if any.

#### Returns

`Option<String>`

## Trait Implementations

- `impl Borrow for SearchHistoryCursor`
- `impl BorrowMut for SearchHistoryCursor`
- `impl CloneToUninit for SearchHistoryCursor`
- `impl Into for SearchHistoryCursor`
- `impl From for SearchHistoryCursor`
- `impl TryInto for SearchHistoryCursor`
- `impl TryFrom for SearchHistoryCursor`
- `impl Any for SearchHistoryCursor`
- `impl ToOwned for SearchHistoryCursor`
- `impl Equivalent for SearchHistoryCursor`
- `impl DynClone for SearchHistoryCursor`
- `impl VZip for SearchHistoryCursor`
- `impl CastableFrom for SearchHistoryCursor`
- `impl CastableFrom for SearchHistoryCursor`
- `impl Read for SearchHistoryCursor`
- `impl IntoEither for SearchHistoryCursor`
- `impl ErasedDestructor for SearchHistoryCursor`
- `impl Same for SearchHistoryCursor`
- `impl ReadPrimitive for SearchHistoryCursor`
- `impl Pointable for SearchHistoryCursor`
- `impl Instrument for SearchHistoryCursor`
- `impl WithSubscriber for SearchHistoryCursor`
- `impl FromAngle for SearchHistoryCursor`
- `impl IntoAngle for SearchHistoryCursor`
- `impl IntoCam16Unclamped for SearchHistoryCursor`
- `impl Cam16IntoUnclamped for SearchHistoryCursor`
- `impl ArraysFrom for SearchHistoryCursor`
- `impl ArraysInto for SearchHistoryCursor`
- `impl ComponentsFrom for SearchHistoryCursor`
- `impl TryComponentsInto for SearchHistoryCursor`
- `impl UintsFrom for SearchHistoryCursor`
- `impl UintsInto for SearchHistoryCursor`
- `impl AdaptIntoUnclamped for SearchHistoryCursor`
- `impl AdaptInto for SearchHistoryCursor`
- `impl IntoColor for SearchHistoryCursor`
- `impl IntoColorUnclamped for SearchHistoryCursor`
- `impl TryIntoColor for SearchHistoryCursor`
- `impl FromStimulus for SearchHistoryCursor`
- `impl IntoStimulus for SearchHistoryCursor`
- `impl Equivalent for SearchHistoryCursor`
- `impl Default for SearchHistoryCursor`
- `impl Debug for SearchHistoryCursor`
- `impl Clone for SearchHistoryCursor`
- `impl StructuralPartialEq for SearchHistoryCursor`
- `impl PartialEq for SearchHistoryCursor`
- `impl Eq for SearchHistoryCursor`
- `impl Hash for SearchHistoryCursor`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

