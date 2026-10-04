---
id: StepCommand
title: StepCommand
---

# Struct: StepCommand

Defined in: [`packages/project/src/debugger/dap_command.rs:109`](../../../../packages/project/src/debugger/dap_command.rs#L109)

## Definition

```rust
pub struct StepCommand
{
    pub thread_id: i64,
    pub granularity: Option<SteppingGranularity>,
    pub single_thread: Option<bool>,
}
```

## Trait Implementations

- `impl Borrow for StepCommand`
- `impl BorrowMut for StepCommand`
- `impl Into for StepCommand`
- `impl From for StepCommand`
- `impl TryInto for StepCommand`
- `impl TryFrom for StepCommand`
- `impl Any for StepCommand`
- `impl Equivalent for StepCommand`
- `impl VZip for StepCommand`
- `impl CastableFrom for StepCommand`
- `impl CastableFrom for StepCommand`
- `impl Read for StepCommand`
- `impl IntoEither for StepCommand`
- `impl ErasedDestructor for StepCommand`
- `impl Same for StepCommand`
- `impl Pointable for StepCommand`
- `impl Instrument for StepCommand`
- `impl WithSubscriber for StepCommand`
- `impl FromAngle for StepCommand`
- `impl IntoAngle for StepCommand`
- `impl IntoCam16Unclamped for StepCommand`
- `impl Cam16IntoUnclamped for StepCommand`
- `impl ArraysFrom for StepCommand`
- `impl ArraysInto for StepCommand`
- `impl ComponentsFrom for StepCommand`
- `impl TryComponentsInto for StepCommand`
- `impl UintsFrom for StepCommand`
- `impl UintsInto for StepCommand`
- `impl AdaptIntoUnclamped for StepCommand`
- `impl AdaptInto for StepCommand`
- `impl IntoColor for StepCommand`
- `impl IntoColorUnclamped for StepCommand`
- `impl TryIntoColor for StepCommand`
- `impl FromStimulus for StepCommand`
- `impl IntoStimulus for StepCommand`
- `impl Equivalent for StepCommand`
- `impl Debug for StepCommand`
- `impl Hash for StepCommand`
- `impl StructuralPartialEq for StepCommand`
- `impl PartialEq for StepCommand`
- `impl Eq for StepCommand`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

