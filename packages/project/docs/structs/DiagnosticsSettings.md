---
id: DiagnosticsSettings
title: DiagnosticsSettings
---

# Struct: DiagnosticsSettings

Defined in: [`packages/project/src/project_settings.rs:620`](../../../../packages/project/src/project_settings.rs#L620)

## Definition

```rust
pub struct DiagnosticsSettings
{
    // Whether to show the project diagnostics button in the status bar.
    pub button: bool,
    // Whether or not to include warning diagnostics.
    pub include_warnings: bool,
    // Settings for using LSP pull diagnostics mechanism in Zed.
    pub lsp_pull_diagnostics: LspPullDiagnosticsSettings,
    // Settings for showing inline diagnostics.
    pub inline: InlineDiagnosticsSettings,
}
```

## Fields

### button

Defined in: [`packages/project/src/project_settings.rs:622`](../../../../packages/project/src/project_settings.rs#L622)

Whether to show the project diagnostics button in the status bar.


***

### include_warnings

Defined in: [`packages/project/src/project_settings.rs:625`](../../../../packages/project/src/project_settings.rs#L625)

Whether or not to include warning diagnostics.


***

### lsp_pull_diagnostics

Defined in: [`packages/project/src/project_settings.rs:628`](../../../../packages/project/src/project_settings.rs#L628)

Settings for using LSP pull diagnostics mechanism in Zed.


***

### inline

Defined in: [`packages/project/src/project_settings.rs:631`](../../../../packages/project/src/project_settings.rs#L631)

Settings for showing inline diagnostics.

## Trait Implementations

- `impl Borrow for DiagnosticsSettings`
- `impl BorrowMut for DiagnosticsSettings`
- `impl CloneToUninit for DiagnosticsSettings`
- `impl Into for DiagnosticsSettings`
- `impl From for DiagnosticsSettings`
- `impl TryInto for DiagnosticsSettings`
- `impl TryFrom for DiagnosticsSettings`
- `impl Any for DiagnosticsSettings`
- `impl ToOwned for DiagnosticsSettings`
- `impl DynClone for DiagnosticsSettings`
- `impl VZip for DiagnosticsSettings`
- `impl CastableFrom for DiagnosticsSettings`
- `impl CastableFrom for DiagnosticsSettings`
- `impl Read for DiagnosticsSettings`
- `impl IntoEither for DiagnosticsSettings`
- `impl ErasedDestructor for DiagnosticsSettings`
- `impl Same for DiagnosticsSettings`
- `impl Pointable for DiagnosticsSettings`
- `impl Instrument for DiagnosticsSettings`
- `impl WithSubscriber for DiagnosticsSettings`
- `impl FromAngle for DiagnosticsSettings`
- `impl IntoAngle for DiagnosticsSettings`
- `impl IntoCam16Unclamped for DiagnosticsSettings`
- `impl Cam16IntoUnclamped for DiagnosticsSettings`
- `impl ArraysFrom for DiagnosticsSettings`
- `impl ArraysInto for DiagnosticsSettings`
- `impl ComponentsFrom for DiagnosticsSettings`
- `impl TryComponentsInto for DiagnosticsSettings`
- `impl UintsFrom for DiagnosticsSettings`
- `impl UintsInto for DiagnosticsSettings`
- `impl AdaptIntoUnclamped for DiagnosticsSettings`
- `impl AdaptInto for DiagnosticsSettings`
- `impl IntoColor for DiagnosticsSettings`
- `impl IntoColorUnclamped for DiagnosticsSettings`
- `impl TryIntoColor for DiagnosticsSettings`
- `impl FromStimulus for DiagnosticsSettings`
- `impl IntoStimulus for DiagnosticsSettings`
- `impl Clone for DiagnosticsSettings`
- `impl Debug for DiagnosticsSettings`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

