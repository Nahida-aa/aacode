---
id: OpenedBufferEvent
title: OpenedBufferEvent
---

# Enum: OpenedBufferEvent

Defined in: [`packages/project/src/event.rs:18`](../../../../packages/project/src/event.rs#L18)

## Definition

```rust
pub enum OpenedBufferEvent {
    Disconnected,
    Ok(BufferId),
    Err(BufferId, Arc<Error>),
}
```

## Trait Implementations

- `impl Borrow for OpenedBufferEvent`
- `impl BorrowMut for OpenedBufferEvent`
- `impl Into for OpenedBufferEvent`
- `impl From for OpenedBufferEvent`
- `impl TryInto for OpenedBufferEvent`
- `impl TryFrom for OpenedBufferEvent`
- `impl Any for OpenedBufferEvent`
- `impl VZip for OpenedBufferEvent`
- `impl CastableFrom for OpenedBufferEvent`
- `impl CastableFrom for OpenedBufferEvent`
- `impl Read for OpenedBufferEvent`
- `impl IntoEither for OpenedBufferEvent`
- `impl ErasedDestructor for OpenedBufferEvent`
- `impl Same for OpenedBufferEvent`
- `impl Pointable for OpenedBufferEvent`
- `impl Instrument for OpenedBufferEvent`
- `impl WithSubscriber for OpenedBufferEvent`
- `impl FromAngle for OpenedBufferEvent`
- `impl IntoAngle for OpenedBufferEvent`
- `impl IntoCam16Unclamped for OpenedBufferEvent`
- `impl Cam16IntoUnclamped for OpenedBufferEvent`
- `impl ArraysFrom for OpenedBufferEvent`
- `impl ArraysInto for OpenedBufferEvent`
- `impl ComponentsFrom for OpenedBufferEvent`
- `impl TryComponentsInto for OpenedBufferEvent`
- `impl UintsFrom for OpenedBufferEvent`
- `impl UintsInto for OpenedBufferEvent`
- `impl AdaptIntoUnclamped for OpenedBufferEvent`
- `impl AdaptInto for OpenedBufferEvent`
- `impl IntoColor for OpenedBufferEvent`
- `impl IntoColorUnclamped for OpenedBufferEvent`
- `impl TryIntoColor for OpenedBufferEvent`
- `impl FromStimulus for OpenedBufferEvent`
- `impl IntoStimulus for OpenedBufferEvent`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

