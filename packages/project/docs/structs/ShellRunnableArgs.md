---
id: ShellRunnableArgs
title: ShellRunnableArgs
---

# Struct: ShellRunnableArgs

Defined in: [`packages/project/src/lsp_store/lsp_ext_command.rs:625`](../../../../packages/project/src/lsp_store/lsp_ext_command.rs#L625)

## Definition

```rust
pub struct ShellRunnableArgs {
    pub environment: HashMap<String, String>,
    pub cwd: PathBuf,
    pub program: String,
    pub args: Vec<String>,
}
```

## Trait Implementations

- `impl Borrow for ShellRunnableArgs`
- `impl BorrowMut for ShellRunnableArgs`
- `impl CloneToUninit for ShellRunnableArgs`
- `impl Into for ShellRunnableArgs`
- `impl From for ShellRunnableArgs`
- `impl TryInto for ShellRunnableArgs`
- `impl TryFrom for ShellRunnableArgs`
- `impl Any for ShellRunnableArgs`
- `impl ToOwned for ShellRunnableArgs`
- `impl DeserializeOwned for ShellRunnableArgs`
- `impl Serialize for ShellRunnableArgs`
- `impl DynClone for ShellRunnableArgs`
- `impl VZip for ShellRunnableArgs`
- `impl CastableFrom for ShellRunnableArgs`
- `impl CastableFrom for ShellRunnableArgs`
- `impl Read for ShellRunnableArgs`
- `impl IntoEither for ShellRunnableArgs`
- `impl ErasedDestructor for ShellRunnableArgs`
- `impl Same for ShellRunnableArgs`
- `impl Pointable for ShellRunnableArgs`
- `impl Instrument for ShellRunnableArgs`
- `impl WithSubscriber for ShellRunnableArgs`
- `impl FromAngle for ShellRunnableArgs`
- `impl IntoAngle for ShellRunnableArgs`
- `impl IntoCam16Unclamped for ShellRunnableArgs`
- `impl Cam16IntoUnclamped for ShellRunnableArgs`
- `impl ArraysFrom for ShellRunnableArgs`
- `impl ArraysInto for ShellRunnableArgs`
- `impl ComponentsFrom for ShellRunnableArgs`
- `impl TryComponentsInto for ShellRunnableArgs`
- `impl UintsFrom for ShellRunnableArgs`
- `impl UintsInto for ShellRunnableArgs`
- `impl AdaptIntoUnclamped for ShellRunnableArgs`
- `impl AdaptInto for ShellRunnableArgs`
- `impl IntoColor for ShellRunnableArgs`
- `impl IntoColorUnclamped for ShellRunnableArgs`
- `impl TryIntoColor for ShellRunnableArgs`
- `impl FromStimulus for ShellRunnableArgs`
- `impl IntoStimulus for ShellRunnableArgs`
- `impl Deserialize for ShellRunnableArgs`
- `impl Serialize for ShellRunnableArgs`
- `impl Debug for ShellRunnableArgs`
- `impl Clone for ShellRunnableArgs`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

