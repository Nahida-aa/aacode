---
id: ResolvedDocumentLink
title: ResolvedDocumentLink
---

# Enum: ResolvedDocumentLink

Defined in: [`packages/project/src/lsp_store/document_links.rs:79`](../../../../packages/project/src/lsp_store/document_links.rs#L79)

Mirror of [`crate::lsp_store::ResolvedHint`](ResolvedHint.md) for document links: callers
either get the resolved entry directly, an in-flight `Shared` task to await
(deduplicated across editors), or `None` when the cache no longer contains
a matching link.

## Definition

```rust
pub enum ResolvedDocumentLink {
    Resolved(LspDocumentLink),
    Resolving(DocumentLinkResolveTask),
}
```

## Trait Implementations

- `impl Borrow for ResolvedDocumentLink`
- `impl BorrowMut for ResolvedDocumentLink`
- `impl Into for ResolvedDocumentLink`
- `impl From for ResolvedDocumentLink`
- `impl TryInto for ResolvedDocumentLink`
- `impl TryFrom for ResolvedDocumentLink`
- `impl Any for ResolvedDocumentLink`
- `impl VZip for ResolvedDocumentLink`
- `impl CastableFrom for ResolvedDocumentLink`
- `impl CastableFrom for ResolvedDocumentLink`
- `impl Read for ResolvedDocumentLink`
- `impl IntoEither for ResolvedDocumentLink`
- `impl ErasedDestructor for ResolvedDocumentLink`
- `impl Same for ResolvedDocumentLink`
- `impl Pointable for ResolvedDocumentLink`
- `impl Instrument for ResolvedDocumentLink`
- `impl WithSubscriber for ResolvedDocumentLink`
- `impl FromAngle for ResolvedDocumentLink`
- `impl IntoAngle for ResolvedDocumentLink`
- `impl IntoCam16Unclamped for ResolvedDocumentLink`
- `impl Cam16IntoUnclamped for ResolvedDocumentLink`
- `impl ArraysFrom for ResolvedDocumentLink`
- `impl ArraysInto for ResolvedDocumentLink`
- `impl ComponentsFrom for ResolvedDocumentLink`
- `impl TryComponentsInto for ResolvedDocumentLink`
- `impl UintsFrom for ResolvedDocumentLink`
- `impl UintsInto for ResolvedDocumentLink`
- `impl AdaptIntoUnclamped for ResolvedDocumentLink`
- `impl AdaptInto for ResolvedDocumentLink`
- `impl IntoColor for ResolvedDocumentLink`
- `impl IntoColorUnclamped for ResolvedDocumentLink`
- `impl TryIntoColor for ResolvedDocumentLink`
- `impl FromStimulus for ResolvedDocumentLink`
- `impl IntoStimulus for ResolvedDocumentLink`

## Auto Trait Implementations

`Freeze` `Send` `Sync` `Unpin` `UnsafeUnpin`

