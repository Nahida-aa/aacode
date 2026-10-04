---
id: DiagnosticSeverity
title: DiagnosticSeverity
---

# Enum: DiagnosticSeverity

Defined in: [`packages/project/src/project_settings.rs:351`](../../../../packages/project/src/project_settings.rs#L351)

## Definition

```rust
pub enum DiagnosticSeverity {
    Off,
    Error,
    Warning,
    Info,
    Hint,
}
```

## Implementations

### into_lsp()

```rust
pub fn into_lsp(self) -> Option<DiagnosticSeverity>
```

Defined in: [`packages/project/src/project_settings.rs:361`](../../../../packages/project/src/project_settings.rs#L361)

#### Returns

`Option<DiagnosticSeverity>`

## Trait Implementations

- `impl Borrow for DiagnosticSeverity`
- `impl BorrowMut for DiagnosticSeverity`
- `impl CloneToUninit for DiagnosticSeverity`
- `impl Into for DiagnosticSeverity`
- `impl From for DiagnosticSeverity`
- `impl TryInto for DiagnosticSeverity`
- `impl TryFrom for DiagnosticSeverity`
- `impl Any for DiagnosticSeverity`
- `impl ToOwned for DiagnosticSeverity`
- `impl Equivalent for DiagnosticSeverity`
- `impl Comparable for DiagnosticSeverity`
- `impl DynClone for DiagnosticSeverity`
- `impl VZip for DiagnosticSeverity`
- `impl CastableFrom for DiagnosticSeverity`
- `impl CastableFrom for DiagnosticSeverity`
- `impl Read for DiagnosticSeverity`
- `impl IntoEither for DiagnosticSeverity`
- `impl ErasedDestructor for DiagnosticSeverity`
- `impl Same for DiagnosticSeverity`
- `impl Pointable for DiagnosticSeverity`
- `impl MapSeekTarget for DiagnosticSeverity`
- `impl Instrument for DiagnosticSeverity`
- `impl WithSubscriber for DiagnosticSeverity`
- `impl FromAngle for DiagnosticSeverity`
- `impl IntoAngle for DiagnosticSeverity`
- `impl IntoCam16Unclamped for DiagnosticSeverity`
- `impl Cam16IntoUnclamped for DiagnosticSeverity`
- `impl ArraysFrom for DiagnosticSeverity`
- `impl ArraysInto for DiagnosticSeverity`
- `impl ComponentsFrom for DiagnosticSeverity`
- `impl TryComponentsInto for DiagnosticSeverity`
- `impl UintsFrom for DiagnosticSeverity`
- `impl UintsInto for DiagnosticSeverity`
- `impl AdaptIntoUnclamped for DiagnosticSeverity`
- `impl AdaptInto for DiagnosticSeverity`
- `impl IntoColor for DiagnosticSeverity`
- `impl IntoColorUnclamped for DiagnosticSeverity`
- `impl TryIntoColor for DiagnosticSeverity`
- `impl FromStimulus for DiagnosticSeverity`
- `impl IntoStimulus for DiagnosticSeverity`
- `impl Equivalent for DiagnosticSeverity`
- `impl ResetDiscriminant for DiagnosticSeverity`
- `impl Clone for DiagnosticSeverity`
- `impl Copy for DiagnosticSeverity`
- `impl Debug for DiagnosticSeverity`
- `impl Eq for DiagnosticSeverity`
- `impl StructuralPartialEq for DiagnosticSeverity`
- `impl PartialEq for DiagnosticSeverity`
- `impl Ord for DiagnosticSeverity`
- `impl PartialOrd for DiagnosticSeverity`
- `impl From for DiagnosticSeverity`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

