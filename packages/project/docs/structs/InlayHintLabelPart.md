---
id: InlayHintLabelPart
title: InlayHintLabelPart
---

# Struct: InlayHintLabelPart

Defined in: [`packages/project/src/types/inlay_hint.rs:59`](../../../../packages/project/src/types/inlay_hint.rs#L59)

## Definition

```rust
pub struct InlayHintLabelPart {
    pub value: String,
    pub tooltip: Option<InlayHintLabelPartTooltip>,
    pub location: Option<(LanguageServerId, Location)>,
    pub command: Option<(LanguageServerId, Command)>,
}
```

## Trait Implementations

- `impl Borrow for InlayHintLabelPart`
- `impl BorrowMut for InlayHintLabelPart`
- `impl CloneToUninit for InlayHintLabelPart`
- `impl Into for InlayHintLabelPart`
- `impl From for InlayHintLabelPart`
- `impl TryInto for InlayHintLabelPart`
- `impl TryFrom for InlayHintLabelPart`
- `impl Any for InlayHintLabelPart`
- `impl ToOwned for InlayHintLabelPart`
- `impl DynClone for InlayHintLabelPart`
- `impl VZip for InlayHintLabelPart`
- `impl CastableFrom for InlayHintLabelPart`
- `impl CastableFrom for InlayHintLabelPart`
- `impl Read for InlayHintLabelPart`
- `impl IntoEither for InlayHintLabelPart`
- `impl ErasedDestructor for InlayHintLabelPart`
- `impl Same for InlayHintLabelPart`
- `impl Pointable for InlayHintLabelPart`
- `impl Instrument for InlayHintLabelPart`
- `impl WithSubscriber for InlayHintLabelPart`
- `impl FromAngle for InlayHintLabelPart`
- `impl IntoAngle for InlayHintLabelPart`
- `impl IntoCam16Unclamped for InlayHintLabelPart`
- `impl Cam16IntoUnclamped for InlayHintLabelPart`
- `impl ArraysFrom for InlayHintLabelPart`
- `impl ArraysInto for InlayHintLabelPart`
- `impl ComponentsFrom for InlayHintLabelPart`
- `impl TryComponentsInto for InlayHintLabelPart`
- `impl UintsFrom for InlayHintLabelPart`
- `impl UintsInto for InlayHintLabelPart`
- `impl AdaptIntoUnclamped for InlayHintLabelPart`
- `impl AdaptInto for InlayHintLabelPart`
- `impl IntoColor for InlayHintLabelPart`
- `impl IntoColorUnclamped for InlayHintLabelPart`
- `impl TryIntoColor for InlayHintLabelPart`
- `impl FromStimulus for InlayHintLabelPart`
- `impl IntoStimulus for InlayHintLabelPart`
- `impl Debug for InlayHintLabelPart`
- `impl Clone for InlayHintLabelPart`
- `impl StructuralPartialEq for InlayHintLabelPart`
- `impl PartialEq for InlayHintLabelPart`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

