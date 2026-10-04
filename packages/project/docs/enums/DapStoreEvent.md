---
id: DapStoreEvent
title: DapStoreEvent
---

# Enum: DapStoreEvent

Defined in: [`packages/project/src/debugger/dap_store.rs:59`](../../../../packages/project/src/debugger/dap_store.rs#L59)

## Definition

```rust
pub enum DapStoreEvent {
    DebugClientStarted(SessionId),
    DebugSessionInitialized(SessionId),
    DebugClientShutdown(SessionId),
    DebugClientEvent{ .. },
    Notification(String),
    RemoteHasInitialized,
}
```

## Trait Implementations

- `impl Borrow for DapStoreEvent`
- `impl BorrowMut for DapStoreEvent`
- `impl Into for DapStoreEvent`
- `impl From for DapStoreEvent`
- `impl TryInto for DapStoreEvent`
- `impl TryFrom for DapStoreEvent`
- `impl Any for DapStoreEvent`
- `impl VZip for DapStoreEvent`
- `impl CastableFrom for DapStoreEvent`
- `impl CastableFrom for DapStoreEvent`
- `impl Read for DapStoreEvent`
- `impl IntoEither for DapStoreEvent`
- `impl ErasedDestructor for DapStoreEvent`
- `impl Same for DapStoreEvent`
- `impl Pointable for DapStoreEvent`
- `impl Instrument for DapStoreEvent`
- `impl WithSubscriber for DapStoreEvent`
- `impl FromAngle for DapStoreEvent`
- `impl IntoAngle for DapStoreEvent`
- `impl IntoCam16Unclamped for DapStoreEvent`
- `impl Cam16IntoUnclamped for DapStoreEvent`
- `impl ArraysFrom for DapStoreEvent`
- `impl ArraysInto for DapStoreEvent`
- `impl ComponentsFrom for DapStoreEvent`
- `impl TryComponentsInto for DapStoreEvent`
- `impl UintsFrom for DapStoreEvent`
- `impl UintsInto for DapStoreEvent`
- `impl AdaptIntoUnclamped for DapStoreEvent`
- `impl AdaptInto for DapStoreEvent`
- `impl IntoColor for DapStoreEvent`
- `impl IntoColorUnclamped for DapStoreEvent`
- `impl TryIntoColor for DapStoreEvent`
- `impl FromStimulus for DapStoreEvent`
- `impl IntoStimulus for DapStoreEvent`
- `impl Debug for DapStoreEvent`
- `impl EventEmitter for DapStoreEvent`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

