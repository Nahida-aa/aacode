---
id: QueryInsertionBehavior
title: QueryInsertionBehavior
---

# Enum: QueryInsertionBehavior

Defined in: [`packages/project/src/search_history.rs:5`](../../../../packages/project/src/search_history.rs#L5)

Determines the behavior to use when inserting a new query into the search history.

## Definition

```rust
pub enum QueryInsertionBehavior
{
    // Always insert the query to the search history.
    AlwaysInsert,
    // Replace the previous query in the search history, if the new query contains the previous query.
    ReplacePreviousIfContains,
}
```

## Variants

### AlwaysInsert

Defined in: [`packages/project/src/search_history.rs:8`](../../../../packages/project/src/search_history.rs#L8)

Always insert the query to the search history.


***

### ReplacePreviousIfContains

Defined in: [`packages/project/src/search_history.rs:10`](../../../../packages/project/src/search_history.rs#L10)

Replace the previous query in the search history, if the new query contains the previous query.

## Trait Implementations

- `impl Borrow for QueryInsertionBehavior`
- `impl BorrowMut for QueryInsertionBehavior`
- `impl CloneToUninit for QueryInsertionBehavior`
- `impl Into for QueryInsertionBehavior`
- `impl From for QueryInsertionBehavior`
- `impl TryInto for QueryInsertionBehavior`
- `impl TryFrom for QueryInsertionBehavior`
- `impl Any for QueryInsertionBehavior`
- `impl ToOwned for QueryInsertionBehavior`
- `impl DynClone for QueryInsertionBehavior`
- `impl VZip for QueryInsertionBehavior`
- `impl CastableFrom for QueryInsertionBehavior`
- `impl CastableFrom for QueryInsertionBehavior`
- `impl Read for QueryInsertionBehavior`
- `impl IntoEither for QueryInsertionBehavior`
- `impl ErasedDestructor for QueryInsertionBehavior`
- `impl Same for QueryInsertionBehavior`
- `impl ReadPrimitive for QueryInsertionBehavior`
- `impl Pointable for QueryInsertionBehavior`
- `impl Instrument for QueryInsertionBehavior`
- `impl WithSubscriber for QueryInsertionBehavior`
- `impl FromAngle for QueryInsertionBehavior`
- `impl IntoAngle for QueryInsertionBehavior`
- `impl IntoCam16Unclamped for QueryInsertionBehavior`
- `impl Cam16IntoUnclamped for QueryInsertionBehavior`
- `impl ArraysFrom for QueryInsertionBehavior`
- `impl ArraysInto for QueryInsertionBehavior`
- `impl ComponentsFrom for QueryInsertionBehavior`
- `impl TryComponentsInto for QueryInsertionBehavior`
- `impl UintsFrom for QueryInsertionBehavior`
- `impl UintsInto for QueryInsertionBehavior`
- `impl AdaptIntoUnclamped for QueryInsertionBehavior`
- `impl AdaptInto for QueryInsertionBehavior`
- `impl IntoColor for QueryInsertionBehavior`
- `impl IntoColorUnclamped for QueryInsertionBehavior`
- `impl TryIntoColor for QueryInsertionBehavior`
- `impl FromStimulus for QueryInsertionBehavior`
- `impl IntoStimulus for QueryInsertionBehavior`
- `impl Default for QueryInsertionBehavior`
- `impl Debug for QueryInsertionBehavior`
- `impl Clone for QueryInsertionBehavior`
- `impl StructuralPartialEq for QueryInsertionBehavior`
- `impl PartialEq for QueryInsertionBehavior`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

