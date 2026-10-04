---
id: BufferStoreEvent
title: BufferStoreEvent
---

# Enum: BufferStoreEvent

Defined in: [`packages/project/src/buffer_store.rs:89`](../../../../packages/project/src/buffer_store.rs#L89)

## Definition

```rust
pub enum BufferStoreEvent {
    BufferAdded(Entity<Buffer>),
    SharedBufferClosed(PeerId, BufferId),
    BufferDropped(BufferId),
    BufferChangedFilePath{ .. },
}
```

## Trait Implementations

- `impl Borrow for BufferStoreEvent`
- `impl BorrowMut for BufferStoreEvent`
- `impl Into for BufferStoreEvent`
- `impl From for BufferStoreEvent`
- `impl TryInto for BufferStoreEvent`
- `impl TryFrom for BufferStoreEvent`
- `impl Any for BufferStoreEvent`
- `impl VZip for BufferStoreEvent`
- `impl CastableFrom for BufferStoreEvent`
- `impl CastableFrom for BufferStoreEvent`
- `impl Read for BufferStoreEvent`
- `impl IntoEither for BufferStoreEvent`
- `impl ErasedDestructor for BufferStoreEvent`
- `impl Same for BufferStoreEvent`
- `impl Pointable for BufferStoreEvent`
- `impl Instrument for BufferStoreEvent`
- `impl WithSubscriber for BufferStoreEvent`
- `impl FromAngle for BufferStoreEvent`
- `impl IntoAngle for BufferStoreEvent`
- `impl IntoCam16Unclamped for BufferStoreEvent`
- `impl Cam16IntoUnclamped for BufferStoreEvent`
- `impl ArraysFrom for BufferStoreEvent`
- `impl ArraysInto for BufferStoreEvent`
- `impl ComponentsFrom for BufferStoreEvent`
- `impl TryComponentsInto for BufferStoreEvent`
- `impl UintsFrom for BufferStoreEvent`
- `impl UintsInto for BufferStoreEvent`
- `impl AdaptIntoUnclamped for BufferStoreEvent`
- `impl AdaptInto for BufferStoreEvent`
- `impl IntoColor for BufferStoreEvent`
- `impl IntoColorUnclamped for BufferStoreEvent`
- `impl TryIntoColor for BufferStoreEvent`
- `impl FromStimulus for BufferStoreEvent`
- `impl IntoStimulus for BufferStoreEvent`
- `impl EventEmitter for BufferStoreEvent`

## Auto Trait Implementations

`Freeze` `Send` `Sync` `Unpin` `UnsafeUnpin`

