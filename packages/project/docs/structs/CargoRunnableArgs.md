---
id: CargoRunnableArgs
title: CargoRunnableArgs
---

# Struct: CargoRunnableArgs

Defined in: [`packages/project/src/lsp_store/lsp_ext_command.rs:606`](../../../../packages/project/src/lsp_store/lsp_ext_command.rs#L606)

## Definition

```rust
pub struct CargoRunnableArgs {
    pub environment: HashMap<String, String>,
    pub cwd: PathBuf,
    // Command to be executed instead of cargo
    pub override_cargo: Option<String>,
    pub workspace_root: Option<PathBuf>,
    pub cargo_args: Vec<String>,
    pub executable_args: Vec<String>,
}
```

## Fields

### override_cargo

Defined in: [`packages/project/src/lsp_store/lsp_ext_command.rs:612`](../../../../packages/project/src/lsp_store/lsp_ext_command.rs#L612)

Command to be executed instead of cargo

## Trait Implementations

- `impl Borrow for CargoRunnableArgs`
- `impl BorrowMut for CargoRunnableArgs`
- `impl CloneToUninit for CargoRunnableArgs`
- `impl Into for CargoRunnableArgs`
- `impl From for CargoRunnableArgs`
- `impl TryInto for CargoRunnableArgs`
- `impl TryFrom for CargoRunnableArgs`
- `impl Any for CargoRunnableArgs`
- `impl ToOwned for CargoRunnableArgs`
- `impl DeserializeOwned for CargoRunnableArgs`
- `impl Serialize for CargoRunnableArgs`
- `impl DynClone for CargoRunnableArgs`
- `impl VZip for CargoRunnableArgs`
- `impl CastableFrom for CargoRunnableArgs`
- `impl CastableFrom for CargoRunnableArgs`
- `impl Read for CargoRunnableArgs`
- `impl IntoEither for CargoRunnableArgs`
- `impl ErasedDestructor for CargoRunnableArgs`
- `impl Same for CargoRunnableArgs`
- `impl Pointable for CargoRunnableArgs`
- `impl Instrument for CargoRunnableArgs`
- `impl WithSubscriber for CargoRunnableArgs`
- `impl FromAngle for CargoRunnableArgs`
- `impl IntoAngle for CargoRunnableArgs`
- `impl IntoCam16Unclamped for CargoRunnableArgs`
- `impl Cam16IntoUnclamped for CargoRunnableArgs`
- `impl ArraysFrom for CargoRunnableArgs`
- `impl ArraysInto for CargoRunnableArgs`
- `impl ComponentsFrom for CargoRunnableArgs`
- `impl TryComponentsInto for CargoRunnableArgs`
- `impl UintsFrom for CargoRunnableArgs`
- `impl UintsInto for CargoRunnableArgs`
- `impl AdaptIntoUnclamped for CargoRunnableArgs`
- `impl AdaptInto for CargoRunnableArgs`
- `impl IntoColor for CargoRunnableArgs`
- `impl IntoColorUnclamped for CargoRunnableArgs`
- `impl TryIntoColor for CargoRunnableArgs`
- `impl FromStimulus for CargoRunnableArgs`
- `impl IntoStimulus for CargoRunnableArgs`
- `impl Deserialize for CargoRunnableArgs`
- `impl Serialize for CargoRunnableArgs`
- `impl Debug for CargoRunnableArgs`
- `impl Clone for CargoRunnableArgs`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

