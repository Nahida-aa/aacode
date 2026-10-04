---
id: LspFoldingRange
title: LspFoldingRange
---

# Struct: LspFoldingRange

Defined in: [`packages/project/src/lsp_store/folding_ranges.rs:26`](../../../../packages/project/src/lsp_store/folding_ranges.rs#L26)

## Definition

```rust
pub struct LspFoldingRange {
    pub range: Range<Anchor>,
    pub collapsed_text: Option<SharedString>,
}
```

## Trait Implementations

- `impl Borrow for LspFoldingRange`
- `impl BorrowMut for LspFoldingRange`
- `impl CloneToUninit for LspFoldingRange`
- `impl Into for LspFoldingRange`
- `impl From for LspFoldingRange`
- `impl TryInto for LspFoldingRange`
- `impl TryFrom for LspFoldingRange`
- `impl Any for LspFoldingRange`
- `impl ToOwned for LspFoldingRange`
- `impl DynClone for LspFoldingRange`
- `impl VZip for LspFoldingRange`
- `impl CastableFrom for LspFoldingRange`
- `impl CastableFrom for LspFoldingRange`
- `impl Read for LspFoldingRange`
- `impl IntoEither for LspFoldingRange`
- `impl ErasedDestructor for LspFoldingRange`
- `impl Same for LspFoldingRange`
- `impl Pointable for LspFoldingRange`
- `impl Instrument for LspFoldingRange`
- `impl WithSubscriber for LspFoldingRange`
- `impl FromAngle for LspFoldingRange`
- `impl IntoAngle for LspFoldingRange`
- `impl IntoCam16Unclamped for LspFoldingRange`
- `impl Cam16IntoUnclamped for LspFoldingRange`
- `impl ArraysFrom for LspFoldingRange`
- `impl ArraysInto for LspFoldingRange`
- `impl ComponentsFrom for LspFoldingRange`
- `impl TryComponentsInto for LspFoldingRange`
- `impl UintsFrom for LspFoldingRange`
- `impl UintsInto for LspFoldingRange`
- `impl AdaptIntoUnclamped for LspFoldingRange`
- `impl AdaptInto for LspFoldingRange`
- `impl IntoColor for LspFoldingRange`
- `impl IntoColorUnclamped for LspFoldingRange`
- `impl TryIntoColor for LspFoldingRange`
- `impl FromStimulus for LspFoldingRange`
- `impl IntoStimulus for LspFoldingRange`
- `impl Clone for LspFoldingRange`
- `impl Debug for LspFoldingRange`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

