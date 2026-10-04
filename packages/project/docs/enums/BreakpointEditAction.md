---
id: BreakpointEditAction
title: BreakpointEditAction
---

# Enum: BreakpointEditAction

Defined in: [`packages/project/src/debugger/breakpoint_store.rs:927`](../../../../packages/project/src/debugger/breakpoint_store.rs#L927)

## Definition

```rust
pub enum BreakpointEditAction {
    Toggle,
    InvertState,
    EditLogMessage(Arc<str>),
    EditCondition(Arc<str>),
    EditHitCondition(Arc<str>),
}
```

## Trait Implementations

- `impl Borrow for BreakpointEditAction`
- `impl BorrowMut for BreakpointEditAction`
- `impl CloneToUninit for BreakpointEditAction`
- `impl Into for BreakpointEditAction`
- `impl From for BreakpointEditAction`
- `impl TryInto for BreakpointEditAction`
- `impl TryFrom for BreakpointEditAction`
- `impl Any for BreakpointEditAction`
- `impl ToOwned for BreakpointEditAction`
- `impl DynClone for BreakpointEditAction`
- `impl VZip for BreakpointEditAction`
- `impl CastableFrom for BreakpointEditAction`
- `impl CastableFrom for BreakpointEditAction`
- `impl Read for BreakpointEditAction`
- `impl IntoEither for BreakpointEditAction`
- `impl ErasedDestructor for BreakpointEditAction`
- `impl Same for BreakpointEditAction`
- `impl Pointable for BreakpointEditAction`
- `impl Instrument for BreakpointEditAction`
- `impl WithSubscriber for BreakpointEditAction`
- `impl FromAngle for BreakpointEditAction`
- `impl IntoAngle for BreakpointEditAction`
- `impl IntoCam16Unclamped for BreakpointEditAction`
- `impl Cam16IntoUnclamped for BreakpointEditAction`
- `impl ArraysFrom for BreakpointEditAction`
- `impl ArraysInto for BreakpointEditAction`
- `impl ComponentsFrom for BreakpointEditAction`
- `impl TryComponentsInto for BreakpointEditAction`
- `impl UintsFrom for BreakpointEditAction`
- `impl UintsInto for BreakpointEditAction`
- `impl AdaptIntoUnclamped for BreakpointEditAction`
- `impl AdaptInto for BreakpointEditAction`
- `impl IntoColor for BreakpointEditAction`
- `impl IntoColorUnclamped for BreakpointEditAction`
- `impl TryIntoColor for BreakpointEditAction`
- `impl FromStimulus for BreakpointEditAction`
- `impl IntoStimulus for BreakpointEditAction`
- `impl Clone for BreakpointEditAction`
- `impl Debug for BreakpointEditAction`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

