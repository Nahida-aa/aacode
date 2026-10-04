---
id: BreakpointSessionState
title: BreakpointSessionState
---

# Struct: BreakpointSessionState

Defined in: [`packages/project/src/debugger/breakpoint_store.rs:58`](../../../../packages/project/src/debugger/breakpoint_store.rs#L58)

## Definition

```rust
pub struct BreakpointSessionState
{
    // Session-specific identifier for the breakpoint, as assigned by Debug Adapter.
    pub id: u64,
    pub verified: bool,
}
```

## Fields

### id

Defined in: [`packages/project/src/debugger/breakpoint_store.rs:60`](../../../../packages/project/src/debugger/breakpoint_store.rs#L60)

Session-specific identifier for the breakpoint, as assigned by Debug Adapter.

## Trait Implementations

- `impl Borrow for BreakpointSessionState`
- `impl BorrowMut for BreakpointSessionState`
- `impl CloneToUninit for BreakpointSessionState`
- `impl Into for BreakpointSessionState`
- `impl From for BreakpointSessionState`
- `impl TryInto for BreakpointSessionState`
- `impl TryFrom for BreakpointSessionState`
- `impl Any for BreakpointSessionState`
- `impl ToOwned for BreakpointSessionState`
- `impl Equivalent for BreakpointSessionState`
- `impl DynClone for BreakpointSessionState`
- `impl VZip for BreakpointSessionState`
- `impl CastableFrom for BreakpointSessionState`
- `impl CastableFrom for BreakpointSessionState`
- `impl Read for BreakpointSessionState`
- `impl IntoEither for BreakpointSessionState`
- `impl ErasedDestructor for BreakpointSessionState`
- `impl Same for BreakpointSessionState`
- `impl Pointable for BreakpointSessionState`
- `impl Instrument for BreakpointSessionState`
- `impl WithSubscriber for BreakpointSessionState`
- `impl FromAngle for BreakpointSessionState`
- `impl IntoAngle for BreakpointSessionState`
- `impl IntoCam16Unclamped for BreakpointSessionState`
- `impl Cam16IntoUnclamped for BreakpointSessionState`
- `impl ArraysFrom for BreakpointSessionState`
- `impl ArraysInto for BreakpointSessionState`
- `impl ComponentsFrom for BreakpointSessionState`
- `impl TryComponentsInto for BreakpointSessionState`
- `impl UintsFrom for BreakpointSessionState`
- `impl UintsInto for BreakpointSessionState`
- `impl AdaptIntoUnclamped for BreakpointSessionState`
- `impl AdaptInto for BreakpointSessionState`
- `impl IntoColor for BreakpointSessionState`
- `impl IntoColorUnclamped for BreakpointSessionState`
- `impl TryIntoColor for BreakpointSessionState`
- `impl FromStimulus for BreakpointSessionState`
- `impl IntoStimulus for BreakpointSessionState`
- `impl Equivalent for BreakpointSessionState`
- `impl ResetDiscriminant for BreakpointSessionState`
- `impl Clone for BreakpointSessionState`
- `impl Copy for BreakpointSessionState`
- `impl Debug for BreakpointSessionState`
- `impl Hash for BreakpointSessionState`
- `impl StructuralPartialEq for BreakpointSessionState`
- `impl PartialEq for BreakpointSessionState`
- `impl Eq for BreakpointSessionState`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

