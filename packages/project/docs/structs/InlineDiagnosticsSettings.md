---
id: InlineDiagnosticsSettings
title: InlineDiagnosticsSettings
---

# Struct: InlineDiagnosticsSettings

Defined in: [`packages/project/src/project_settings.rs:635`](../../../../packages/project/src/project_settings.rs#L635)

## Definition

```rust
pub struct InlineDiagnosticsSettings
{
    // Whether or not to show inline diagnostics Default: false
    pub enabled: bool,
    // Whether to only show the inline diagnostics after a delay after the last editor event. Default: 150
    pub update_debounce_ms: u64,
    // The amount of padding between the end of the source line and the start of the inline diagnostic in units of columns. Default: 4
    pub padding: u32,
    // The minimum column to display inline diagnostics. This setting can be used to horizontally align inline diagnostics at some position. Lines longer than this value will still push diagnostics further to the right. Default: 0
    pub min_column: u32,
    pub max_severity: Option<DiagnosticSeverity>,
}
```

## Fields

### enabled

Defined in: [`packages/project/src/project_settings.rs:639`](../../../../packages/project/src/project_settings.rs#L639)

Whether or not to show inline diagnostics

Default: false


***

### update_debounce_ms

Defined in: [`packages/project/src/project_settings.rs:644`](../../../../packages/project/src/project_settings.rs#L644)

Whether to only show the inline diagnostics after a delay after the
last editor event.

Default: 150


***

### padding

Defined in: [`packages/project/src/project_settings.rs:649`](../../../../packages/project/src/project_settings.rs#L649)

The amount of padding between the end of the source line and the start
of the inline diagnostic in units of columns.

Default: 4


***

### min_column

Defined in: [`packages/project/src/project_settings.rs:655`](../../../../packages/project/src/project_settings.rs#L655)

The minimum column to display inline diagnostics. This setting can be
used to horizontally align inline diagnostics at some position. Lines
longer than this value will still push diagnostics further to the right.

Default: 0

## Trait Implementations

- `impl Borrow for InlineDiagnosticsSettings`
- `impl BorrowMut for InlineDiagnosticsSettings`
- `impl CloneToUninit for InlineDiagnosticsSettings`
- `impl Into for InlineDiagnosticsSettings`
- `impl From for InlineDiagnosticsSettings`
- `impl TryInto for InlineDiagnosticsSettings`
- `impl TryFrom for InlineDiagnosticsSettings`
- `impl Any for InlineDiagnosticsSettings`
- `impl ToOwned for InlineDiagnosticsSettings`
- `impl Equivalent for InlineDiagnosticsSettings`
- `impl DynClone for InlineDiagnosticsSettings`
- `impl VZip for InlineDiagnosticsSettings`
- `impl CastableFrom for InlineDiagnosticsSettings`
- `impl CastableFrom for InlineDiagnosticsSettings`
- `impl Read for InlineDiagnosticsSettings`
- `impl IntoEither for InlineDiagnosticsSettings`
- `impl ErasedDestructor for InlineDiagnosticsSettings`
- `impl Same for InlineDiagnosticsSettings`
- `impl Pointable for InlineDiagnosticsSettings`
- `impl Instrument for InlineDiagnosticsSettings`
- `impl WithSubscriber for InlineDiagnosticsSettings`
- `impl FromAngle for InlineDiagnosticsSettings`
- `impl IntoAngle for InlineDiagnosticsSettings`
- `impl IntoCam16Unclamped for InlineDiagnosticsSettings`
- `impl Cam16IntoUnclamped for InlineDiagnosticsSettings`
- `impl ArraysFrom for InlineDiagnosticsSettings`
- `impl ArraysInto for InlineDiagnosticsSettings`
- `impl ComponentsFrom for InlineDiagnosticsSettings`
- `impl TryComponentsInto for InlineDiagnosticsSettings`
- `impl UintsFrom for InlineDiagnosticsSettings`
- `impl UintsInto for InlineDiagnosticsSettings`
- `impl AdaptIntoUnclamped for InlineDiagnosticsSettings`
- `impl AdaptInto for InlineDiagnosticsSettings`
- `impl IntoColor for InlineDiagnosticsSettings`
- `impl IntoColorUnclamped for InlineDiagnosticsSettings`
- `impl TryIntoColor for InlineDiagnosticsSettings`
- `impl FromStimulus for InlineDiagnosticsSettings`
- `impl IntoStimulus for InlineDiagnosticsSettings`
- `impl Equivalent for InlineDiagnosticsSettings`
- `impl ResetDiscriminant for InlineDiagnosticsSettings`
- `impl Clone for InlineDiagnosticsSettings`
- `impl Copy for InlineDiagnosticsSettings`
- `impl Debug for InlineDiagnosticsSettings`
- `impl StructuralPartialEq for InlineDiagnosticsSettings`
- `impl PartialEq for InlineDiagnosticsSettings`
- `impl Eq for InlineDiagnosticsSettings`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

