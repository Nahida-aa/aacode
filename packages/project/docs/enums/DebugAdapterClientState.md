---
id: DebugAdapterClientState
title: DebugAdapterClientState
---

# Enum: DebugAdapterClientState

Defined in: [`packages/project/src/project/state.rs:107`](../../../../packages/project/src/project/state.rs#L107)

## Definition

```rust
pub enum DebugAdapterClientState {
    Starting(Task<Option<Arc<DebugAdapterClient>>>),
    Running(Arc<DebugAdapterClient>),
}
```

## Trait Implementations

- `impl Borrow for DebugAdapterClientState`
- `impl BorrowMut for DebugAdapterClientState`
- `impl Into for DebugAdapterClientState`
- `impl From for DebugAdapterClientState`
- `impl TryInto for DebugAdapterClientState`
- `impl TryFrom for DebugAdapterClientState`
- `impl Any for DebugAdapterClientState`
- `impl VZip for DebugAdapterClientState`
- `impl CastableFrom for DebugAdapterClientState`
- `impl CastableFrom for DebugAdapterClientState`
- `impl Read for DebugAdapterClientState`
- `impl IntoEither for DebugAdapterClientState`
- `impl ErasedDestructor for DebugAdapterClientState`
- `impl Same for DebugAdapterClientState`
- `impl Pointable for DebugAdapterClientState`
- `impl Instrument for DebugAdapterClientState`
- `impl WithSubscriber for DebugAdapterClientState`
- `impl FromAngle for DebugAdapterClientState`
- `impl IntoAngle for DebugAdapterClientState`
- `impl IntoCam16Unclamped for DebugAdapterClientState`
- `impl Cam16IntoUnclamped for DebugAdapterClientState`
- `impl ArraysFrom for DebugAdapterClientState`
- `impl ArraysInto for DebugAdapterClientState`
- `impl ComponentsFrom for DebugAdapterClientState`
- `impl TryComponentsInto for DebugAdapterClientState`
- `impl UintsFrom for DebugAdapterClientState`
- `impl UintsInto for DebugAdapterClientState`
- `impl AdaptIntoUnclamped for DebugAdapterClientState`
- `impl AdaptInto for DebugAdapterClientState`
- `impl IntoColor for DebugAdapterClientState`
- `impl IntoColorUnclamped for DebugAdapterClientState`
- `impl TryIntoColor for DebugAdapterClientState`
- `impl FromStimulus for DebugAdapterClientState`
- `impl IntoStimulus for DebugAdapterClientState`

## Auto Trait Implementations

`Freeze` `Send` `Sync` `Unpin` `UnsafeUnpin`

