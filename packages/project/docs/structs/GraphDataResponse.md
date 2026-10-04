---
id: GraphDataResponse
title: GraphDataResponse
---

# Struct: GraphDataResponse

Defined in: [`packages/project/src/git_store/mod.rs:669`](../../../../packages/project/src/git_store/mod.rs#L669)

## Definition

```rust
pub struct GraphDataResponse<'a>
{
    pub commits: &'a [Arc<InitialGraphCommitData>],
    pub is_loading: bool,
    pub error: Option<SharedString>,
}
```

## Trait Implementations

- `impl Borrow for GraphDataResponse`
- `impl BorrowMut for GraphDataResponse`
- `impl Into for GraphDataResponse`
- `impl From for GraphDataResponse`
- `impl TryInto for GraphDataResponse`
- `impl TryFrom for GraphDataResponse`
- `impl Any for GraphDataResponse`
- `impl VZip for GraphDataResponse`
- `impl CastableFrom for GraphDataResponse`
- `impl CastableFrom for GraphDataResponse`
- `impl Read for GraphDataResponse`
- `impl IntoEither for GraphDataResponse`
- `impl ErasedDestructor for GraphDataResponse`
- `impl Same for GraphDataResponse`
- `impl Pointable for GraphDataResponse`
- `impl Instrument for GraphDataResponse`
- `impl WithSubscriber for GraphDataResponse`
- `impl FromAngle for GraphDataResponse`
- `impl IntoAngle for GraphDataResponse`
- `impl IntoCam16Unclamped for GraphDataResponse`
- `impl Cam16IntoUnclamped for GraphDataResponse`
- `impl ArraysFrom for GraphDataResponse`
- `impl ArraysInto for GraphDataResponse`
- `impl ComponentsFrom for GraphDataResponse`
- `impl TryComponentsInto for GraphDataResponse`
- `impl UintsFrom for GraphDataResponse`
- `impl UintsInto for GraphDataResponse`
- `impl AdaptIntoUnclamped for GraphDataResponse`
- `impl AdaptInto for GraphDataResponse`
- `impl IntoColor for GraphDataResponse`
- `impl IntoColorUnclamped for GraphDataResponse`
- `impl TryIntoColor for GraphDataResponse`
- `impl FromStimulus for GraphDataResponse`
- `impl IntoStimulus for GraphDataResponse`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

