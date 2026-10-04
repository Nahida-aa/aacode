---
id: SignatureHelpData
title: SignatureHelpData
---

# Struct: SignatureHelpData

Defined in: [`packages/project/src/lsp_command/signature_help.rs:18`](../../../../packages/project/src/lsp_command/signature_help.rs#L18)

## Definition

```rust
pub struct SignatureHelpData
{
    pub label: SharedString,
    pub documentation: Option<Entity<Markdown>>,
    pub highlights: Vec<(Range<usize>, HighlightStyle)>,
    pub active_parameter: Option<usize>,
    pub parameters: Vec<ParameterInfo>,
}
```

## Trait Implementations

- `impl Borrow for SignatureHelpData`
- `impl BorrowMut for SignatureHelpData`
- `impl CloneToUninit for SignatureHelpData`
- `impl Into for SignatureHelpData`
- `impl From for SignatureHelpData`
- `impl TryInto for SignatureHelpData`
- `impl TryFrom for SignatureHelpData`
- `impl Any for SignatureHelpData`
- `impl ToOwned for SignatureHelpData`
- `impl DynClone for SignatureHelpData`
- `impl VZip for SignatureHelpData`
- `impl CastableFrom for SignatureHelpData`
- `impl CastableFrom for SignatureHelpData`
- `impl Read for SignatureHelpData`
- `impl IntoEither for SignatureHelpData`
- `impl ErasedDestructor for SignatureHelpData`
- `impl Same for SignatureHelpData`
- `impl Pointable for SignatureHelpData`
- `impl Instrument for SignatureHelpData`
- `impl WithSubscriber for SignatureHelpData`
- `impl FromAngle for SignatureHelpData`
- `impl IntoAngle for SignatureHelpData`
- `impl IntoCam16Unclamped for SignatureHelpData`
- `impl Cam16IntoUnclamped for SignatureHelpData`
- `impl ArraysFrom for SignatureHelpData`
- `impl ArraysInto for SignatureHelpData`
- `impl ComponentsFrom for SignatureHelpData`
- `impl TryComponentsInto for SignatureHelpData`
- `impl UintsFrom for SignatureHelpData`
- `impl UintsInto for SignatureHelpData`
- `impl AdaptIntoUnclamped for SignatureHelpData`
- `impl AdaptInto for SignatureHelpData`
- `impl IntoColor for SignatureHelpData`
- `impl IntoColorUnclamped for SignatureHelpData`
- `impl TryIntoColor for SignatureHelpData`
- `impl FromStimulus for SignatureHelpData`
- `impl IntoStimulus for SignatureHelpData`
- `impl Debug for SignatureHelpData`
- `impl Clone for SignatureHelpData`

## Auto Trait Implementations

`Freeze` `Send` `Sync` `Unpin` `UnsafeUnpin`

