---
id: ServerStatusChangedEvent
title: ServerStatusChangedEvent
---

# Struct: ServerStatusChangedEvent

Defined in: [`packages/project/src/context_server_store/mod.rs:309`](../../../../packages/project/src/context_server_store/mod.rs#L309)

## Definition

```rust
pub struct ServerStatusChangedEvent
{
    pub server_id: ContextServerId,
    pub status: ContextServerStatus,
}
```

## Trait Implementations

- `impl Borrow for ServerStatusChangedEvent`
- `impl BorrowMut for ServerStatusChangedEvent`
- `impl Into for ServerStatusChangedEvent`
- `impl From for ServerStatusChangedEvent`
- `impl TryInto for ServerStatusChangedEvent`
- `impl TryFrom for ServerStatusChangedEvent`
- `impl Any for ServerStatusChangedEvent`
- `impl VZip for ServerStatusChangedEvent`
- `impl CastableFrom for ServerStatusChangedEvent`
- `impl CastableFrom for ServerStatusChangedEvent`
- `impl Read for ServerStatusChangedEvent`
- `impl IntoEither for ServerStatusChangedEvent`
- `impl ErasedDestructor for ServerStatusChangedEvent`
- `impl Same for ServerStatusChangedEvent`
- `impl Pointable for ServerStatusChangedEvent`
- `impl Instrument for ServerStatusChangedEvent`
- `impl WithSubscriber for ServerStatusChangedEvent`
- `impl FromAngle for ServerStatusChangedEvent`
- `impl IntoAngle for ServerStatusChangedEvent`
- `impl IntoCam16Unclamped for ServerStatusChangedEvent`
- `impl Cam16IntoUnclamped for ServerStatusChangedEvent`
- `impl ArraysFrom for ServerStatusChangedEvent`
- `impl ArraysInto for ServerStatusChangedEvent`
- `impl ComponentsFrom for ServerStatusChangedEvent`
- `impl TryComponentsInto for ServerStatusChangedEvent`
- `impl UintsFrom for ServerStatusChangedEvent`
- `impl UintsInto for ServerStatusChangedEvent`
- `impl AdaptIntoUnclamped for ServerStatusChangedEvent`
- `impl AdaptInto for ServerStatusChangedEvent`
- `impl IntoColor for ServerStatusChangedEvent`
- `impl IntoColorUnclamped for ServerStatusChangedEvent`
- `impl TryIntoColor for ServerStatusChangedEvent`
- `impl FromStimulus for ServerStatusChangedEvent`
- `impl IntoStimulus for ServerStatusChangedEvent`
- `impl EventEmitter for ServerStatusChangedEvent`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

