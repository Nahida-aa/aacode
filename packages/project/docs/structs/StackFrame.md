---
id: StackFrame
title: StackFrame
---

# Struct: StackFrame

Defined in: [`packages/project/src/debugger/session.rs:84`](../../../../packages/project/src/debugger/session.rs#L84)

## Definition

```rust
pub struct StackFrame {
    pub dap: StackFrame,
    pub scopes: Vec<Scope>,
}
```

## Trait Implementations

- `impl Borrow for StackFrame`
- `impl BorrowMut for StackFrame`
- `impl CloneToUninit for StackFrame`
- `impl Into for StackFrame`
- `impl From for StackFrame`
- `impl TryInto for StackFrame`
- `impl TryFrom for StackFrame`
- `impl Any for StackFrame`
- `impl ToOwned for StackFrame`
- `impl DynClone for StackFrame`
- `impl VZip for StackFrame`
- `impl CastableFrom for StackFrame`
- `impl CastableFrom for StackFrame`
- `impl Read for StackFrame`
- `impl IntoEither for StackFrame`
- `impl ErasedDestructor for StackFrame`
- `impl Same for StackFrame`
- `impl Pointable for StackFrame`
- `impl Instrument for StackFrame`
- `impl WithSubscriber for StackFrame`
- `impl FromAngle for StackFrame`
- `impl IntoAngle for StackFrame`
- `impl IntoCam16Unclamped for StackFrame`
- `impl Cam16IntoUnclamped for StackFrame`
- `impl ArraysFrom for StackFrame`
- `impl ArraysInto for StackFrame`
- `impl ComponentsFrom for StackFrame`
- `impl TryComponentsInto for StackFrame`
- `impl UintsFrom for StackFrame`
- `impl UintsInto for StackFrame`
- `impl AdaptIntoUnclamped for StackFrame`
- `impl AdaptInto for StackFrame`
- `impl IntoColor for StackFrame`
- `impl IntoColorUnclamped for StackFrame`
- `impl TryIntoColor for StackFrame`
- `impl FromStimulus for StackFrame`
- `impl IntoStimulus for StackFrame`
- `impl Clone for StackFrame`
- `impl Debug for StackFrame`
- `impl From for StackFrame`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

