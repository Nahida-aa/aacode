---
id: LocalRepositoryState
title: LocalRepositoryState
---

# Struct: LocalRepositoryState

Defined in: [`packages/project/src/git_store/mod.rs:752`](../../../../packages/project/src/git_store/mod.rs#L752)

## Definition

```rust
pub struct LocalRepositoryState
{
    pub fs: Arc<dyn Fs>,
    pub backend: Arc<dyn GitRepository>,
    pub environment: Arc<HashMap<String, String>>,
}
```

## Trait Implementations

- `impl Borrow for LocalRepositoryState`
- `impl BorrowMut for LocalRepositoryState`
- `impl CloneToUninit for LocalRepositoryState`
- `impl Into for LocalRepositoryState`
- `impl From for LocalRepositoryState`
- `impl TryInto for LocalRepositoryState`
- `impl TryFrom for LocalRepositoryState`
- `impl Any for LocalRepositoryState`
- `impl ToOwned for LocalRepositoryState`
- `impl DynClone for LocalRepositoryState`
- `impl VZip for LocalRepositoryState`
- `impl CastableFrom for LocalRepositoryState`
- `impl CastableFrom for LocalRepositoryState`
- `impl Read for LocalRepositoryState`
- `impl IntoEither for LocalRepositoryState`
- `impl ErasedDestructor for LocalRepositoryState`
- `impl Same for LocalRepositoryState`
- `impl Pointable for LocalRepositoryState`
- `impl Instrument for LocalRepositoryState`
- `impl WithSubscriber for LocalRepositoryState`
- `impl FromAngle for LocalRepositoryState`
- `impl IntoAngle for LocalRepositoryState`
- `impl IntoCam16Unclamped for LocalRepositoryState`
- `impl Cam16IntoUnclamped for LocalRepositoryState`
- `impl ArraysFrom for LocalRepositoryState`
- `impl ArraysInto for LocalRepositoryState`
- `impl ComponentsFrom for LocalRepositoryState`
- `impl TryComponentsInto for LocalRepositoryState`
- `impl UintsFrom for LocalRepositoryState`
- `impl UintsInto for LocalRepositoryState`
- `impl AdaptIntoUnclamped for LocalRepositoryState`
- `impl AdaptInto for LocalRepositoryState`
- `impl IntoColor for LocalRepositoryState`
- `impl IntoColorUnclamped for LocalRepositoryState`
- `impl TryIntoColor for LocalRepositoryState`
- `impl FromStimulus for LocalRepositoryState`
- `impl IntoStimulus for LocalRepositoryState`
- `impl Clone for LocalRepositoryState`

## Auto Trait Implementations

`Freeze` `Send` `Sync` `Unpin` `UnsafeUnpin`

