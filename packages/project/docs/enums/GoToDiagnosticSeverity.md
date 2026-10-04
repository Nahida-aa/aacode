---
id: GoToDiagnosticSeverity
title: GoToDiagnosticSeverity
---

# Enum: GoToDiagnosticSeverity

Defined in: [`packages/project/src/project_settings.rs:388`](../../../../packages/project/src/project_settings.rs#L388)

Determines the severity of the diagnostic that should be moved to.

## Definition

```rust
pub enum GoToDiagnosticSeverity
{
    // Errors
    Error,
    // Warnings
    Warning,
    // Information
    Information,
    // Hints
    Hint,
}
```

## Variants

### Error

Defined in: [`packages/project/src/project_settings.rs:390`](../../../../packages/project/src/project_settings.rs#L390)

Errors


***

### Warning

Defined in: [`packages/project/src/project_settings.rs:392`](../../../../packages/project/src/project_settings.rs#L392)

Warnings


***

### Information

Defined in: [`packages/project/src/project_settings.rs:394`](../../../../packages/project/src/project_settings.rs#L394)

Information


***

### Hint

Defined in: [`packages/project/src/project_settings.rs:396`](../../../../packages/project/src/project_settings.rs#L396)

Hints

## Implementations

### min()

```rust
pub fn min() -> Self
```

Defined in: [`packages/project/src/project_settings.rs:412`](../../../../packages/project/src/project_settings.rs#L412)

#### Returns

`Self`


***

### max()

```rust
pub fn max() -> Self
```

Defined in: [`packages/project/src/project_settings.rs:416`](../../../../packages/project/src/project_settings.rs#L416)

#### Returns

`Self`

## Trait Implementations

- `impl Borrow for GoToDiagnosticSeverity`
- `impl BorrowMut for GoToDiagnosticSeverity`
- `impl CloneToUninit for GoToDiagnosticSeverity`
- `impl Into for GoToDiagnosticSeverity`
- `impl From for GoToDiagnosticSeverity`
- `impl TryInto for GoToDiagnosticSeverity`
- `impl TryFrom for GoToDiagnosticSeverity`
- `impl Any for GoToDiagnosticSeverity`
- `impl ToOwned for GoToDiagnosticSeverity`
- `impl DeserializeOwned for GoToDiagnosticSeverity`
- `impl Equivalent for GoToDiagnosticSeverity`
- `impl DynClone for GoToDiagnosticSeverity`
- `impl VZip for GoToDiagnosticSeverity`
- `impl CastableFrom for GoToDiagnosticSeverity`
- `impl CastableFrom for GoToDiagnosticSeverity`
- `impl Read for GoToDiagnosticSeverity`
- `impl IntoEither for GoToDiagnosticSeverity`
- `impl ErasedDestructor for GoToDiagnosticSeverity`
- `impl Same for GoToDiagnosticSeverity`
- `impl Pointable for GoToDiagnosticSeverity`
- `impl Instrument for GoToDiagnosticSeverity`
- `impl WithSubscriber for GoToDiagnosticSeverity`
- `impl FromAngle for GoToDiagnosticSeverity`
- `impl IntoAngle for GoToDiagnosticSeverity`
- `impl IntoCam16Unclamped for GoToDiagnosticSeverity`
- `impl Cam16IntoUnclamped for GoToDiagnosticSeverity`
- `impl ArraysFrom for GoToDiagnosticSeverity`
- `impl ArraysInto for GoToDiagnosticSeverity`
- `impl ComponentsFrom for GoToDiagnosticSeverity`
- `impl TryComponentsInto for GoToDiagnosticSeverity`
- `impl UintsFrom for GoToDiagnosticSeverity`
- `impl UintsInto for GoToDiagnosticSeverity`
- `impl AdaptIntoUnclamped for GoToDiagnosticSeverity`
- `impl AdaptInto for GoToDiagnosticSeverity`
- `impl IntoColor for GoToDiagnosticSeverity`
- `impl IntoColorUnclamped for GoToDiagnosticSeverity`
- `impl TryIntoColor for GoToDiagnosticSeverity`
- `impl FromStimulus for GoToDiagnosticSeverity`
- `impl IntoStimulus for GoToDiagnosticSeverity`
- `impl Equivalent for GoToDiagnosticSeverity`
- `impl ResetDiscriminant for GoToDiagnosticSeverity`
- `impl StructuralPartialEq for GoToDiagnosticSeverity`
- `impl PartialEq for GoToDiagnosticSeverity`
- `impl PartialOrd for GoToDiagnosticSeverity`
- `impl Clone for GoToDiagnosticSeverity`
- `impl Copy for GoToDiagnosticSeverity`
- `impl Debug for GoToDiagnosticSeverity`
- `impl Eq for GoToDiagnosticSeverity`
- `impl Deserialize for GoToDiagnosticSeverity`
- `impl JsonSchema for GoToDiagnosticSeverity`
- `impl From for GoToDiagnosticSeverity`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

