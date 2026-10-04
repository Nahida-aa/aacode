---
id: SearchResults
title: SearchResults
---

# Struct: SearchResults

Defined in: [`packages/project/src/project_search.rs:72`](../../../../packages/project/src/project_search.rs#L72)

## Definition

```rust
pub struct SearchResults<T> {
    pub task_handle: Task<()>,
    pub rx: Receiver<T>,
}
```

## Trait Implementations

- `impl Borrow for SearchResults`
- `impl BorrowMut for SearchResults`
- `impl Into for SearchResults`
- `impl From for SearchResults`
- `impl TryInto for SearchResults`
- `impl TryFrom for SearchResults`
- `impl Any for SearchResults`
- `impl VZip for SearchResults`
- `impl CastableFrom for SearchResults`
- `impl CastableFrom for SearchResults`
- `impl Read for SearchResults`
- `impl IntoEither for SearchResults`
- `impl ErasedDestructor for SearchResults`
- `impl Same for SearchResults`
- `impl Pointable for SearchResults`
- `impl Instrument for SearchResults`
- `impl WithSubscriber for SearchResults`
- `impl FromAngle for SearchResults`
- `impl IntoAngle for SearchResults`
- `impl IntoCam16Unclamped for SearchResults`
- `impl Cam16IntoUnclamped for SearchResults`
- `impl ArraysFrom for SearchResults`
- `impl ArraysInto for SearchResults`
- `impl ComponentsFrom for SearchResults`
- `impl TryComponentsInto for SearchResults`
- `impl UintsFrom for SearchResults`
- `impl UintsInto for SearchResults`
- `impl AdaptIntoUnclamped for SearchResults`
- `impl AdaptInto for SearchResults`
- `impl IntoColor for SearchResults`
- `impl IntoColorUnclamped for SearchResults`
- `impl TryIntoColor for SearchResults`
- `impl FromStimulus for SearchResults`
- `impl IntoStimulus for SearchResults`

## Auto Trait Implementations

`Freeze` `Send` `Sync`

