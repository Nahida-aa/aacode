---
id: DocumentLinkResolveTask
title: DocumentLinkResolveTask
---

# Type Alias: DocumentLinkResolveTask

Defined in: [`packages/project/src/lsp_store/document_links.rs:42`](../../../../packages/project/src/lsp_store/document_links.rs#L42)

## Definition

```rust
pub type DocumentLinkResolveTask = Shared<Task<Option<(DocumentLinkId, LspDocumentLink)>>>
```

