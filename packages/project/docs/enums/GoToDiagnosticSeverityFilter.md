---
id: GoToDiagnosticSeverityFilter
title: GoToDiagnosticSeverityFilter
---

# Enum: GoToDiagnosticSeverityFilter

Defined in: [`packages/project/src/project_settings.rs:424`](../../../../packages/project/src/project_settings.rs#L424)

Allows filtering diagnostics that should be moved to.

## Definition

```rust
pub enum GoToDiagnosticSeverityFilter {
    // Move to diagnostics of a specific severity.
    Only(GoToDiagnosticSeverity),
    // Specify a range of severities to include.
    Range{ .. },
}
```

## Variants

### Only

Defined in: [`packages/project/src/project_settings.rs:426`](../../../../packages/project/src/project_settings.rs#L426)

Move to diagnostics of a specific severity.


***

### Range

Defined in: [`packages/project/src/project_settings.rs:429`](../../../../packages/project/src/project_settings.rs#L429)

Specify a range of severities to include.

## Implementations

### matches()

```rust
pub fn matches(&self, severity: DiagnosticSeverity) -> bool
```

Defined in: [`packages/project/src/project_settings.rs:449`](../../../../packages/project/src/project_settings.rs#L449)

#### Parameters

##### severity

`DiagnosticSeverity`

#### Returns

`bool`

## Trait Implementations

- `impl Borrow for GoToDiagnosticSeverityFilter`
- `impl BorrowMut for GoToDiagnosticSeverityFilter`
- `impl CloneToUninit for GoToDiagnosticSeverityFilter`
- `impl Into for GoToDiagnosticSeverityFilter`
- `impl From for GoToDiagnosticSeverityFilter`
- `impl TryInto for GoToDiagnosticSeverityFilter`
- `impl TryFrom for GoToDiagnosticSeverityFilter`
- `impl Any for GoToDiagnosticSeverityFilter`
- `impl ToOwned for GoToDiagnosticSeverityFilter`
- `impl DeserializeOwned for GoToDiagnosticSeverityFilter`
- `impl DynClone for GoToDiagnosticSeverityFilter`
- `impl VZip for GoToDiagnosticSeverityFilter`
- `impl CastableFrom for GoToDiagnosticSeverityFilter`
- `impl CastableFrom for GoToDiagnosticSeverityFilter`
- `impl Read for GoToDiagnosticSeverityFilter`
- `impl IntoEither for GoToDiagnosticSeverityFilter`
- `impl ErasedDestructor for GoToDiagnosticSeverityFilter`
- `impl Same for GoToDiagnosticSeverityFilter`
- `impl ReadPrimitive for GoToDiagnosticSeverityFilter`
- `impl Pointable for GoToDiagnosticSeverityFilter`
- `impl Instrument for GoToDiagnosticSeverityFilter`
- `impl WithSubscriber for GoToDiagnosticSeverityFilter`
- `impl FromAngle for GoToDiagnosticSeverityFilter`
- `impl IntoAngle for GoToDiagnosticSeverityFilter`
- `impl IntoCam16Unclamped for GoToDiagnosticSeverityFilter`
- `impl Cam16IntoUnclamped for GoToDiagnosticSeverityFilter`
- `impl ArraysFrom for GoToDiagnosticSeverityFilter`
- `impl ArraysInto for GoToDiagnosticSeverityFilter`
- `impl ComponentsFrom for GoToDiagnosticSeverityFilter`
- `impl TryComponentsInto for GoToDiagnosticSeverityFilter`
- `impl UintsFrom for GoToDiagnosticSeverityFilter`
- `impl UintsInto for GoToDiagnosticSeverityFilter`
- `impl AdaptIntoUnclamped for GoToDiagnosticSeverityFilter`
- `impl AdaptInto for GoToDiagnosticSeverityFilter`
- `impl IntoColor for GoToDiagnosticSeverityFilter`
- `impl IntoColorUnclamped for GoToDiagnosticSeverityFilter`
- `impl TryIntoColor for GoToDiagnosticSeverityFilter`
- `impl FromStimulus for GoToDiagnosticSeverityFilter`
- `impl IntoStimulus for GoToDiagnosticSeverityFilter`
- `impl ResetDiscriminant for GoToDiagnosticSeverityFilter`
- `impl StructuralPartialEq for GoToDiagnosticSeverityFilter`
- `impl PartialEq for GoToDiagnosticSeverityFilter`
- `impl Clone for GoToDiagnosticSeverityFilter`
- `impl Copy for GoToDiagnosticSeverityFilter`
- `impl Debug for GoToDiagnosticSeverityFilter`
- `impl Deserialize for GoToDiagnosticSeverityFilter`
- `impl JsonSchema for GoToDiagnosticSeverityFilter`
- `impl Default for GoToDiagnosticSeverityFilter`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

