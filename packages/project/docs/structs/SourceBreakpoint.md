---
id: SourceBreakpoint
title: SourceBreakpoint
---

# Struct: SourceBreakpoint

Defined in: [`packages/project/src/debugger/breakpoint_store.rs:1056`](../../../../packages/project/src/debugger/breakpoint_store.rs#L1056)

Breakpoint for location within source code.

## Definition

```rust
pub struct SourceBreakpoint {
    pub row: u32,
    pub path: Arc<Path>,
    pub message: Option<Arc<str>>,
    pub condition: Option<Arc<str>>,
    pub hit_condition: Option<Arc<str>>,
    pub state: BreakpointState,
}
```

## Trait Implementations

- `impl Borrow for SourceBreakpoint`
- `impl BorrowMut for SourceBreakpoint`
- `impl CloneToUninit for SourceBreakpoint`
- `impl Into for SourceBreakpoint`
- `impl From for SourceBreakpoint`
- `impl TryInto for SourceBreakpoint`
- `impl TryFrom for SourceBreakpoint`
- `impl Any for SourceBreakpoint`
- `impl ToOwned for SourceBreakpoint`
- `impl Equivalent for SourceBreakpoint`
- `impl DynClone for SourceBreakpoint`
- `impl VZip for SourceBreakpoint`
- `impl CastableFrom for SourceBreakpoint`
- `impl CastableFrom for SourceBreakpoint`
- `impl Read for SourceBreakpoint`
- `impl IntoEither for SourceBreakpoint`
- `impl ErasedDestructor for SourceBreakpoint`
- `impl Same for SourceBreakpoint`
- `impl Pointable for SourceBreakpoint`
- `impl Instrument for SourceBreakpoint`
- `impl WithSubscriber for SourceBreakpoint`
- `impl FromAngle for SourceBreakpoint`
- `impl IntoAngle for SourceBreakpoint`
- `impl IntoCam16Unclamped for SourceBreakpoint`
- `impl Cam16IntoUnclamped for SourceBreakpoint`
- `impl ArraysFrom for SourceBreakpoint`
- `impl ArraysInto for SourceBreakpoint`
- `impl ComponentsFrom for SourceBreakpoint`
- `impl TryComponentsInto for SourceBreakpoint`
- `impl UintsFrom for SourceBreakpoint`
- `impl UintsInto for SourceBreakpoint`
- `impl AdaptIntoUnclamped for SourceBreakpoint`
- `impl AdaptInto for SourceBreakpoint`
- `impl IntoColor for SourceBreakpoint`
- `impl IntoColorUnclamped for SourceBreakpoint`
- `impl TryIntoColor for SourceBreakpoint`
- `impl FromStimulus for SourceBreakpoint`
- `impl IntoStimulus for SourceBreakpoint`
- `impl Equivalent for SourceBreakpoint`
- `impl Clone for SourceBreakpoint`
- `impl Debug for SourceBreakpoint`
- `impl Hash for SourceBreakpoint`
- `impl StructuralPartialEq for SourceBreakpoint`
- `impl PartialEq for SourceBreakpoint`
- `impl Eq for SourceBreakpoint`
- `impl From for SourceBreakpoint`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

