---
id: DataBreakpointContext
title: DataBreakpointContext
---

# Enum: DataBreakpointContext

Defined in: [`packages/project/src/debugger/dap_command.rs:1673`](../../../../packages/project/src/debugger/dap_command.rs#L1673)

## Definition

```rust
pub enum DataBreakpointContext {
    Variable{ .. },
    Expression{ .. },
    Address{ .. },
}
```

## Implementations

### human_readable_label()

```rust
pub fn human_readable_label(&self) -> String
```

Defined in: [`packages/project/src/debugger/dap_command.rs:1690`](../../../../packages/project/src/debugger/dap_command.rs#L1690)

#### Returns

`String`

## Trait Implementations

- `impl Borrow for DataBreakpointContext`
- `impl BorrowMut for DataBreakpointContext`
- `impl CloneToUninit for DataBreakpointContext`
- `impl Into for DataBreakpointContext`
- `impl From for DataBreakpointContext`
- `impl TryInto for DataBreakpointContext`
- `impl TryFrom for DataBreakpointContext`
- `impl Any for DataBreakpointContext`
- `impl ToOwned for DataBreakpointContext`
- `impl Equivalent for DataBreakpointContext`
- `impl DynClone for DataBreakpointContext`
- `impl VZip for DataBreakpointContext`
- `impl CastableFrom for DataBreakpointContext`
- `impl CastableFrom for DataBreakpointContext`
- `impl Read for DataBreakpointContext`
- `impl IntoEither for DataBreakpointContext`
- `impl ErasedDestructor for DataBreakpointContext`
- `impl Same for DataBreakpointContext`
- `impl Pointable for DataBreakpointContext`
- `impl Instrument for DataBreakpointContext`
- `impl WithSubscriber for DataBreakpointContext`
- `impl FromAngle for DataBreakpointContext`
- `impl IntoAngle for DataBreakpointContext`
- `impl IntoCam16Unclamped for DataBreakpointContext`
- `impl Cam16IntoUnclamped for DataBreakpointContext`
- `impl ArraysFrom for DataBreakpointContext`
- `impl ArraysInto for DataBreakpointContext`
- `impl ComponentsFrom for DataBreakpointContext`
- `impl TryComponentsInto for DataBreakpointContext`
- `impl UintsFrom for DataBreakpointContext`
- `impl UintsInto for DataBreakpointContext`
- `impl AdaptIntoUnclamped for DataBreakpointContext`
- `impl AdaptInto for DataBreakpointContext`
- `impl IntoColor for DataBreakpointContext`
- `impl IntoColorUnclamped for DataBreakpointContext`
- `impl TryIntoColor for DataBreakpointContext`
- `impl FromStimulus for DataBreakpointContext`
- `impl IntoStimulus for DataBreakpointContext`
- `impl Equivalent for DataBreakpointContext`
- `impl Clone for DataBreakpointContext`
- `impl Debug for DataBreakpointContext`
- `impl Hash for DataBreakpointContext`
- `impl StructuralPartialEq for DataBreakpointContext`
- `impl PartialEq for DataBreakpointContext`
- `impl Eq for DataBreakpointContext`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

