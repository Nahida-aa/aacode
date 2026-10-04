---
id: DataBreakpointState
title: DataBreakpointState
---

# Struct: DataBreakpointState

Defined in: [`packages/project/src/debugger/session.rs:148`](../../../../packages/project/src/debugger/session.rs#L148)

## Definition

```rust
pub struct DataBreakpointState
{
    pub dap: DataBreakpoint,
    pub is_enabled: bool,
    pub context: Arc<DataBreakpointContext>,
}
```

## Trait Implementations

- `impl Borrow for DataBreakpointState`
- `impl BorrowMut for DataBreakpointState`
- `impl CloneToUninit for DataBreakpointState`
- `impl Into for DataBreakpointState`
- `impl From for DataBreakpointState`
- `impl TryInto for DataBreakpointState`
- `impl TryFrom for DataBreakpointState`
- `impl Any for DataBreakpointState`
- `impl ToOwned for DataBreakpointState`
- `impl DynClone for DataBreakpointState`
- `impl VZip for DataBreakpointState`
- `impl CastableFrom for DataBreakpointState`
- `impl CastableFrom for DataBreakpointState`
- `impl Read for DataBreakpointState`
- `impl IntoEither for DataBreakpointState`
- `impl ErasedDestructor for DataBreakpointState`
- `impl Same for DataBreakpointState`
- `impl Pointable for DataBreakpointState`
- `impl Instrument for DataBreakpointState`
- `impl WithSubscriber for DataBreakpointState`
- `impl FromAngle for DataBreakpointState`
- `impl IntoAngle for DataBreakpointState`
- `impl IntoCam16Unclamped for DataBreakpointState`
- `impl Cam16IntoUnclamped for DataBreakpointState`
- `impl ArraysFrom for DataBreakpointState`
- `impl ArraysInto for DataBreakpointState`
- `impl ComponentsFrom for DataBreakpointState`
- `impl TryComponentsInto for DataBreakpointState`
- `impl UintsFrom for DataBreakpointState`
- `impl UintsInto for DataBreakpointState`
- `impl AdaptIntoUnclamped for DataBreakpointState`
- `impl AdaptInto for DataBreakpointState`
- `impl IntoColor for DataBreakpointState`
- `impl IntoColorUnclamped for DataBreakpointState`
- `impl TryIntoColor for DataBreakpointState`
- `impl FromStimulus for DataBreakpointState`
- `impl IntoStimulus for DataBreakpointState`
- `impl Debug for DataBreakpointState`
- `impl Clone for DataBreakpointState`
- `impl StructuralPartialEq for DataBreakpointState`
- `impl PartialEq for DataBreakpointState`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

