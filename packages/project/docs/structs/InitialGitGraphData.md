---
id: InitialGitGraphData
title: InitialGitGraphData
---

# Struct: InitialGitGraphData

Defined in: [`packages/project/src/git_store/mod.rs:661`](../../../../packages/project/src/git_store/mod.rs#L661)

## Definition

```rust
pub struct InitialGitGraphData {
    pub error: Option<SharedString>,
    pub commit_data: Vec<Arc<InitialGraphCommitData>>,
    pub commit_oid_to_index: HashMap<Oid, usize>,
}
```

_（存在非公开字段）_

## Trait Implementations

- `impl Borrow for InitialGitGraphData`
- `impl BorrowMut for InitialGitGraphData`
- `impl Into for InitialGitGraphData`
- `impl From for InitialGitGraphData`
- `impl TryInto for InitialGitGraphData`
- `impl TryFrom for InitialGitGraphData`
- `impl Any for InitialGitGraphData`
- `impl VZip for InitialGitGraphData`
- `impl CastableFrom for InitialGitGraphData`
- `impl CastableFrom for InitialGitGraphData`
- `impl Read for InitialGitGraphData`
- `impl IntoEither for InitialGitGraphData`
- `impl ErasedDestructor for InitialGitGraphData`
- `impl Same for InitialGitGraphData`
- `impl Pointable for InitialGitGraphData`
- `impl Instrument for InitialGitGraphData`
- `impl WithSubscriber for InitialGitGraphData`
- `impl FromAngle for InitialGitGraphData`
- `impl IntoAngle for InitialGitGraphData`
- `impl IntoCam16Unclamped for InitialGitGraphData`
- `impl Cam16IntoUnclamped for InitialGitGraphData`
- `impl ArraysFrom for InitialGitGraphData`
- `impl ArraysInto for InitialGitGraphData`
- `impl ComponentsFrom for InitialGitGraphData`
- `impl TryComponentsInto for InitialGitGraphData`
- `impl UintsFrom for InitialGitGraphData`
- `impl UintsInto for InitialGitGraphData`
- `impl AdaptIntoUnclamped for InitialGitGraphData`
- `impl AdaptInto for InitialGitGraphData`
- `impl IntoColor for InitialGitGraphData`
- `impl IntoColorUnclamped for InitialGitGraphData`
- `impl TryIntoColor for InitialGitGraphData`
- `impl FromStimulus for InitialGitGraphData`
- `impl IntoStimulus for InitialGitGraphData`

## Auto Trait Implementations

`Freeze` `Send` `Sync` `Unpin` `UnsafeUnpin`

