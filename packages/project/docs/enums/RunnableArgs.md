---
id: RunnableArgs
title: RunnableArgs
---

# Enum: RunnableArgs

Defined in: [`packages/project/src/lsp_store/lsp_ext_command.rs:599`](../../../../packages/project/src/lsp_store/lsp_ext_command.rs#L599)

The `kind` field in the JSON determines which variant is deserialized; see
comment on `Runnables` above for more discussion.

## Definition

```rust
pub enum RunnableArgs {
    Cargo(CargoRunnableArgs),
    Shell(ShellRunnableArgs),
}
```

## Trait Implementations

- `impl Borrow for RunnableArgs`
- `impl BorrowMut for RunnableArgs`
- `impl CloneToUninit for RunnableArgs`
- `impl Into for RunnableArgs`
- `impl From for RunnableArgs`
- `impl TryInto for RunnableArgs`
- `impl TryFrom for RunnableArgs`
- `impl Any for RunnableArgs`
- `impl ToOwned for RunnableArgs`
- `impl DeserializeOwned for RunnableArgs`
- `impl Serialize for RunnableArgs`
- `impl DynClone for RunnableArgs`
- `impl VZip for RunnableArgs`
- `impl CastableFrom for RunnableArgs`
- `impl CastableFrom for RunnableArgs`
- `impl Read for RunnableArgs`
- `impl IntoEither for RunnableArgs`
- `impl ErasedDestructor for RunnableArgs`
- `impl Same for RunnableArgs`
- `impl Pointable for RunnableArgs`
- `impl Instrument for RunnableArgs`
- `impl WithSubscriber for RunnableArgs`
- `impl FromAngle for RunnableArgs`
- `impl IntoAngle for RunnableArgs`
- `impl IntoCam16Unclamped for RunnableArgs`
- `impl Cam16IntoUnclamped for RunnableArgs`
- `impl ArraysFrom for RunnableArgs`
- `impl ArraysInto for RunnableArgs`
- `impl ComponentsFrom for RunnableArgs`
- `impl TryComponentsInto for RunnableArgs`
- `impl UintsFrom for RunnableArgs`
- `impl UintsInto for RunnableArgs`
- `impl AdaptIntoUnclamped for RunnableArgs`
- `impl AdaptInto for RunnableArgs`
- `impl IntoColor for RunnableArgs`
- `impl IntoColorUnclamped for RunnableArgs`
- `impl TryIntoColor for RunnableArgs`
- `impl FromStimulus for RunnableArgs`
- `impl IntoStimulus for RunnableArgs`
- `impl Deserialize for RunnableArgs`
- `impl Serialize for RunnableArgs`
- `impl Debug for RunnableArgs`
- `impl Clone for RunnableArgs`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

