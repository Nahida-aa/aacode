---
id: LspDocumentLink
title: LspDocumentLink
---

# Struct: LspDocumentLink

Defined in: [`packages/project/src/lsp_store/document_links.rs:29`](../../../../packages/project/src/lsp_store/document_links.rs#L29)

## Definition

```rust
pub struct LspDocumentLink
{
    pub range: Range<Anchor>,
    pub target: Option<SharedString>,
    pub tooltip: Option<SharedString>,
    pub data: Option<Value>,
    pub resolved: bool,
}
```

## Trait Implementations

- `impl Borrow for LspDocumentLink`
- `impl BorrowMut for LspDocumentLink`
- `impl CloneToUninit for LspDocumentLink`
- `impl Into for LspDocumentLink`
- `impl From for LspDocumentLink`
- `impl TryInto for LspDocumentLink`
- `impl TryFrom for LspDocumentLink`
- `impl Any for LspDocumentLink`
- `impl ToOwned for LspDocumentLink`
- `impl DynClone for LspDocumentLink`
- `impl VZip for LspDocumentLink`
- `impl CastableFrom for LspDocumentLink`
- `impl CastableFrom for LspDocumentLink`
- `impl Read for LspDocumentLink`
- `impl IntoEither for LspDocumentLink`
- `impl ErasedDestructor for LspDocumentLink`
- `impl Same for LspDocumentLink`
- `impl Pointable for LspDocumentLink`
- `impl Instrument for LspDocumentLink`
- `impl WithSubscriber for LspDocumentLink`
- `impl FromAngle for LspDocumentLink`
- `impl IntoAngle for LspDocumentLink`
- `impl IntoCam16Unclamped for LspDocumentLink`
- `impl Cam16IntoUnclamped for LspDocumentLink`
- `impl ArraysFrom for LspDocumentLink`
- `impl ArraysInto for LspDocumentLink`
- `impl ComponentsFrom for LspDocumentLink`
- `impl TryComponentsInto for LspDocumentLink`
- `impl UintsFrom for LspDocumentLink`
- `impl UintsInto for LspDocumentLink`
- `impl AdaptIntoUnclamped for LspDocumentLink`
- `impl AdaptInto for LspDocumentLink`
- `impl IntoColor for LspDocumentLink`
- `impl IntoColorUnclamped for LspDocumentLink`
- `impl TryIntoColor for LspDocumentLink`
- `impl FromStimulus for LspDocumentLink`
- `impl IntoStimulus for LspDocumentLink`
- `impl Clone for LspDocumentLink`
- `impl Debug for LspDocumentLink`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

