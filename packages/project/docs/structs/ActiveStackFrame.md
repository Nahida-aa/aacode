---
id: ActiveStackFrame
title: ActiveStackFrame
---

# Struct: ActiveStackFrame

Defined in: [`packages/project/src/debugger/breakpoint_store.rs:145`](../../../../packages/project/src/debugger/breakpoint_store.rs#L145)

## Definition

```rust
pub struct ActiveStackFrame {
    pub session_id: SessionId,
    pub thread_id: ThreadId,
    pub stack_frame_id: StackFrameId,
    pub path: Arc<Path>,
    pub position: Anchor,
}
```

## Trait Implementations

- `impl Borrow for ActiveStackFrame`
- `impl BorrowMut for ActiveStackFrame`
- `impl CloneToUninit for ActiveStackFrame`
- `impl Into for ActiveStackFrame`
- `impl From for ActiveStackFrame`
- `impl TryInto for ActiveStackFrame`
- `impl TryFrom for ActiveStackFrame`
- `impl Any for ActiveStackFrame`
- `impl ToOwned for ActiveStackFrame`
- `impl DynClone for ActiveStackFrame`
- `impl VZip for ActiveStackFrame`
- `impl CastableFrom for ActiveStackFrame`
- `impl CastableFrom for ActiveStackFrame`
- `impl Read for ActiveStackFrame`
- `impl IntoEither for ActiveStackFrame`
- `impl ErasedDestructor for ActiveStackFrame`
- `impl Same for ActiveStackFrame`
- `impl Pointable for ActiveStackFrame`
- `impl Instrument for ActiveStackFrame`
- `impl WithSubscriber for ActiveStackFrame`
- `impl FromAngle for ActiveStackFrame`
- `impl IntoAngle for ActiveStackFrame`
- `impl IntoCam16Unclamped for ActiveStackFrame`
- `impl Cam16IntoUnclamped for ActiveStackFrame`
- `impl ArraysFrom for ActiveStackFrame`
- `impl ArraysInto for ActiveStackFrame`
- `impl ComponentsFrom for ActiveStackFrame`
- `impl TryComponentsInto for ActiveStackFrame`
- `impl UintsFrom for ActiveStackFrame`
- `impl UintsInto for ActiveStackFrame`
- `impl AdaptIntoUnclamped for ActiveStackFrame`
- `impl AdaptInto for ActiveStackFrame`
- `impl IntoColor for ActiveStackFrame`
- `impl IntoColorUnclamped for ActiveStackFrame`
- `impl TryIntoColor for ActiveStackFrame`
- `impl FromStimulus for ActiveStackFrame`
- `impl IntoStimulus for ActiveStackFrame`
- `impl Clone for ActiveStackFrame`
- `impl StructuralPartialEq for ActiveStackFrame`
- `impl PartialEq for ActiveStackFrame`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

