---
id: VariablesCommand
title: VariablesCommand
---

# Struct: VariablesCommand

Defined in: [`packages/project/src/debugger/dap_command.rs:818`](../../../../packages/project/src/debugger/dap_command.rs#L818)

## Definition

```rust
pub struct VariablesCommand
{
    pub variables_reference: u64,
    pub filter: Option<VariablesArgumentsFilter>,
    pub start: Option<u64>,
    pub count: Option<u64>,
    pub format: Option<ValueFormat>,
}
```

## Trait Implementations

- `impl Borrow for VariablesCommand`
- `impl BorrowMut for VariablesCommand`
- `impl CloneToUninit for VariablesCommand`
- `impl Into for VariablesCommand`
- `impl From for VariablesCommand`
- `impl TryInto for VariablesCommand`
- `impl TryFrom for VariablesCommand`
- `impl Any for VariablesCommand`
- `impl ToOwned for VariablesCommand`
- `impl Equivalent for VariablesCommand`
- `impl DynClone for VariablesCommand`
- `impl VZip for VariablesCommand`
- `impl CastableFrom for VariablesCommand`
- `impl CastableFrom for VariablesCommand`
- `impl Read for VariablesCommand`
- `impl IntoEither for VariablesCommand`
- `impl ErasedDestructor for VariablesCommand`
- `impl Same for VariablesCommand`
- `impl Pointable for VariablesCommand`
- `impl Instrument for VariablesCommand`
- `impl WithSubscriber for VariablesCommand`
- `impl FromAngle for VariablesCommand`
- `impl IntoAngle for VariablesCommand`
- `impl IntoCam16Unclamped for VariablesCommand`
- `impl Cam16IntoUnclamped for VariablesCommand`
- `impl ArraysFrom for VariablesCommand`
- `impl ArraysInto for VariablesCommand`
- `impl ComponentsFrom for VariablesCommand`
- `impl TryComponentsInto for VariablesCommand`
- `impl UintsFrom for VariablesCommand`
- `impl UintsInto for VariablesCommand`
- `impl AdaptIntoUnclamped for VariablesCommand`
- `impl AdaptInto for VariablesCommand`
- `impl IntoColor for VariablesCommand`
- `impl IntoColorUnclamped for VariablesCommand`
- `impl TryIntoColor for VariablesCommand`
- `impl FromStimulus for VariablesCommand`
- `impl IntoStimulus for VariablesCommand`
- `impl Equivalent for VariablesCommand`
- `impl Clone for VariablesCommand`
- `impl Debug for VariablesCommand`
- `impl Hash for VariablesCommand`
- `impl StructuralPartialEq for VariablesCommand`
- `impl PartialEq for VariablesCommand`
- `impl Eq for VariablesCommand`
- `impl LocalDapCommand for VariablesCommand`
- `impl DapCommand for VariablesCommand`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

