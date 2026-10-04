---
id: BreakpointStoreEvent
title: BreakpointStoreEvent
---

# Enum: BreakpointStoreEvent

Defined in: [`packages/project/src/debugger/breakpoint_store.rs:915`](../../../../packages/project/src/debugger/breakpoint_store.rs#L915)

## Definition

```rust
pub enum BreakpointStoreEvent
{
    SetDebugLine,
    ClearDebugLines,
    BreakpointsUpdated(Arc<Path>, BreakpointUpdatedReason),
    BreakpointsCleared(Vec<Arc<Path>>),
}
```

## Trait Implementations

- `impl Borrow for BreakpointStoreEvent`
- `impl BorrowMut for BreakpointStoreEvent`
- `impl Into for BreakpointStoreEvent`
- `impl From for BreakpointStoreEvent`
- `impl TryInto for BreakpointStoreEvent`
- `impl TryFrom for BreakpointStoreEvent`
- `impl Any for BreakpointStoreEvent`
- `impl VZip for BreakpointStoreEvent`
- `impl CastableFrom for BreakpointStoreEvent`
- `impl CastableFrom for BreakpointStoreEvent`
- `impl Read for BreakpointStoreEvent`
- `impl IntoEither for BreakpointStoreEvent`
- `impl ErasedDestructor for BreakpointStoreEvent`
- `impl Same for BreakpointStoreEvent`
- `impl Pointable for BreakpointStoreEvent`
- `impl Instrument for BreakpointStoreEvent`
- `impl WithSubscriber for BreakpointStoreEvent`
- `impl FromAngle for BreakpointStoreEvent`
- `impl IntoAngle for BreakpointStoreEvent`
- `impl IntoCam16Unclamped for BreakpointStoreEvent`
- `impl Cam16IntoUnclamped for BreakpointStoreEvent`
- `impl ArraysFrom for BreakpointStoreEvent`
- `impl ArraysInto for BreakpointStoreEvent`
- `impl ComponentsFrom for BreakpointStoreEvent`
- `impl TryComponentsInto for BreakpointStoreEvent`
- `impl UintsFrom for BreakpointStoreEvent`
- `impl UintsInto for BreakpointStoreEvent`
- `impl AdaptIntoUnclamped for BreakpointStoreEvent`
- `impl AdaptInto for BreakpointStoreEvent`
- `impl IntoColor for BreakpointStoreEvent`
- `impl IntoColorUnclamped for BreakpointStoreEvent`
- `impl TryIntoColor for BreakpointStoreEvent`
- `impl FromStimulus for BreakpointStoreEvent`
- `impl IntoStimulus for BreakpointStoreEvent`
- `impl EventEmitter for BreakpointStoreEvent`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

