---
id: InlayHint
title: InlayHint
---

# Struct: InlayHint

Defined in: [`packages/project/src/types/inlay_hint.rs:26`](../../../../packages/project/src/types/inlay_hint.rs#L26)

## Definition

```rust
pub struct InlayHint {
    pub position: Anchor,
    pub label: InlayHintLabel,
    pub kind: Option<InlayHintKind>,
    pub padding_left: bool,
    pub padding_right: bool,
    pub tooltip: Option<InlayHintTooltip>,
    pub resolve_state: ResolveState,
}
```

## Implementations

### text()

```rust
pub fn text(&self) -> Rope
```

Defined in: [`packages/project/src/types/inlay_hint.rs:44`](../../../../packages/project/src/types/inlay_hint.rs#L44)

#### Returns

`Rope`

## Trait Implementations

- `impl Borrow for InlayHint`
- `impl BorrowMut for InlayHint`
- `impl CloneToUninit for InlayHint`
- `impl Into for InlayHint`
- `impl From for InlayHint`
- `impl TryInto for InlayHint`
- `impl TryFrom for InlayHint`
- `impl Any for InlayHint`
- `impl ToOwned for InlayHint`
- `impl DynClone for InlayHint`
- `impl VZip for InlayHint`
- `impl CastableFrom for InlayHint`
- `impl CastableFrom for InlayHint`
- `impl Read for InlayHint`
- `impl IntoEither for InlayHint`
- `impl ErasedDestructor for InlayHint`
- `impl Same for InlayHint`
- `impl Pointable for InlayHint`
- `impl Instrument for InlayHint`
- `impl WithSubscriber for InlayHint`
- `impl FromAngle for InlayHint`
- `impl IntoAngle for InlayHint`
- `impl IntoCam16Unclamped for InlayHint`
- `impl Cam16IntoUnclamped for InlayHint`
- `impl ArraysFrom for InlayHint`
- `impl ArraysInto for InlayHint`
- `impl ComponentsFrom for InlayHint`
- `impl TryComponentsInto for InlayHint`
- `impl UintsFrom for InlayHint`
- `impl UintsInto for InlayHint`
- `impl AdaptIntoUnclamped for InlayHint`
- `impl AdaptInto for InlayHint`
- `impl IntoColor for InlayHint`
- `impl IntoColorUnclamped for InlayHint`
- `impl TryIntoColor for InlayHint`
- `impl FromStimulus for InlayHint`
- `impl IntoStimulus for InlayHint`
- `impl Debug for InlayHint`
- `impl Clone for InlayHint`
- `impl StructuralPartialEq for InlayHint`
- `impl PartialEq for InlayHint`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

