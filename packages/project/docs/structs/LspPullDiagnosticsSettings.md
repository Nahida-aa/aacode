---
id: LspPullDiagnosticsSettings
title: LspPullDiagnosticsSettings
---

# Struct: LspPullDiagnosticsSettings

Defined in: [`packages/project/src/project_settings.rs:661`](../../../../packages/project/src/project_settings.rs#L661)

## Definition

```rust
pub struct LspPullDiagnosticsSettings {
    // Whether to pull for diagnostics or not. Default: true
    pub enabled: bool,
    // Minimum time to wait before pulling diagnostics from the language server(s). 0 turns the debounce off. Default: 50
    pub debounce_ms: u64,
}
```

## Fields

### enabled

Defined in: [`packages/project/src/project_settings.rs:665`](../../../../packages/project/src/project_settings.rs#L665)

Whether to pull for diagnostics or not.

Default: true


***

### debounce_ms

Defined in: [`packages/project/src/project_settings.rs:670`](../../../../packages/project/src/project_settings.rs#L670)

Minimum time to wait before pulling diagnostics from the language server(s).
0 turns the debounce off.

Default: 50

## Trait Implementations

- `impl Borrow for LspPullDiagnosticsSettings`
- `impl BorrowMut for LspPullDiagnosticsSettings`
- `impl CloneToUninit for LspPullDiagnosticsSettings`
- `impl Into for LspPullDiagnosticsSettings`
- `impl From for LspPullDiagnosticsSettings`
- `impl TryInto for LspPullDiagnosticsSettings`
- `impl TryFrom for LspPullDiagnosticsSettings`
- `impl Any for LspPullDiagnosticsSettings`
- `impl ToOwned for LspPullDiagnosticsSettings`
- `impl DeserializeOwned for LspPullDiagnosticsSettings`
- `impl Equivalent for LspPullDiagnosticsSettings`
- `impl Serialize for LspPullDiagnosticsSettings`
- `impl DynClone for LspPullDiagnosticsSettings`
- `impl VZip for LspPullDiagnosticsSettings`
- `impl CastableFrom for LspPullDiagnosticsSettings`
- `impl CastableFrom for LspPullDiagnosticsSettings`
- `impl Read for LspPullDiagnosticsSettings`
- `impl IntoEither for LspPullDiagnosticsSettings`
- `impl ErasedDestructor for LspPullDiagnosticsSettings`
- `impl Same for LspPullDiagnosticsSettings`
- `impl Pointable for LspPullDiagnosticsSettings`
- `impl Instrument for LspPullDiagnosticsSettings`
- `impl WithSubscriber for LspPullDiagnosticsSettings`
- `impl FromAngle for LspPullDiagnosticsSettings`
- `impl IntoAngle for LspPullDiagnosticsSettings`
- `impl IntoCam16Unclamped for LspPullDiagnosticsSettings`
- `impl Cam16IntoUnclamped for LspPullDiagnosticsSettings`
- `impl ArraysFrom for LspPullDiagnosticsSettings`
- `impl ArraysInto for LspPullDiagnosticsSettings`
- `impl ComponentsFrom for LspPullDiagnosticsSettings`
- `impl TryComponentsInto for LspPullDiagnosticsSettings`
- `impl UintsFrom for LspPullDiagnosticsSettings`
- `impl UintsInto for LspPullDiagnosticsSettings`
- `impl AdaptIntoUnclamped for LspPullDiagnosticsSettings`
- `impl AdaptInto for LspPullDiagnosticsSettings`
- `impl IntoColor for LspPullDiagnosticsSettings`
- `impl IntoColorUnclamped for LspPullDiagnosticsSettings`
- `impl TryIntoColor for LspPullDiagnosticsSettings`
- `impl FromStimulus for LspPullDiagnosticsSettings`
- `impl IntoStimulus for LspPullDiagnosticsSettings`
- `impl Equivalent for LspPullDiagnosticsSettings`
- `impl ResetDiscriminant for LspPullDiagnosticsSettings`
- `impl Clone for LspPullDiagnosticsSettings`
- `impl Copy for LspPullDiagnosticsSettings`
- `impl Debug for LspPullDiagnosticsSettings`
- `impl Serialize for LspPullDiagnosticsSettings`
- `impl Deserialize for LspPullDiagnosticsSettings`
- `impl JsonSchema for LspPullDiagnosticsSettings`
- `impl StructuralPartialEq for LspPullDiagnosticsSettings`
- `impl PartialEq for LspPullDiagnosticsSettings`
- `impl Eq for LspPullDiagnosticsSettings`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

