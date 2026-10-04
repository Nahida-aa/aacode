---
id: LspStoreEvent
title: LspStoreEvent
---

# Enum: LspStoreEvent

Defined in: [`packages/project/src/lsp_store/mod.rs:4644`](../../../../packages/project/src/lsp_store/mod.rs#L4644)

## Definition

```rust
pub enum LspStoreEvent
{
    LanguageServerAdded(LanguageServerId, LanguageServerName, Option<WorktreeId>),
    SupplementaryLanguageServerAdded(LanguageServerId, LanguageServerName),
    LanguageServerRemoved(LanguageServerId),
    SupplementaryLanguageServerRemoved(LanguageServerId),
    LanguageServerUpdate{ .. },
    LanguageServerLog(LanguageServerId, LanguageServerLogType, String),
    LanguageServerPrompt(LanguageServerPromptRequest),
    LanguageDetected{ .. },
    Notification(String),
    LanguageServerShowDocument(LanguageServerShowDocumentRequest),
    RefreshInlayHints{ .. },
    RefreshSemanticTokens{ .. },
    RefreshCodeLens{ .. },
    RefreshDocumentColors{ .. },
    RefreshDocumentLinks{ .. },
    RefreshDocumentHighlights{ .. },
    RefreshFoldingRanges{ .. },
    RefreshDocumentSymbols{ .. },
    DiagnosticsUpdated{ .. },
    DiskBasedDiagnosticsStarted{ .. },
    DiskBasedDiagnosticsFinished{ .. },
    SnippetEdit{ .. },
    WorkspaceEditApplied(ProjectTransaction),
}
```

## Trait Implementations

- `impl Borrow for LspStoreEvent`
- `impl BorrowMut for LspStoreEvent`
- `impl Into for LspStoreEvent`
- `impl From for LspStoreEvent`
- `impl TryInto for LspStoreEvent`
- `impl TryFrom for LspStoreEvent`
- `impl Any for LspStoreEvent`
- `impl VZip for LspStoreEvent`
- `impl CastableFrom for LspStoreEvent`
- `impl CastableFrom for LspStoreEvent`
- `impl Read for LspStoreEvent`
- `impl IntoEither for LspStoreEvent`
- `impl ErasedDestructor for LspStoreEvent`
- `impl Same for LspStoreEvent`
- `impl Pointable for LspStoreEvent`
- `impl Instrument for LspStoreEvent`
- `impl WithSubscriber for LspStoreEvent`
- `impl FromAngle for LspStoreEvent`
- `impl IntoAngle for LspStoreEvent`
- `impl IntoCam16Unclamped for LspStoreEvent`
- `impl Cam16IntoUnclamped for LspStoreEvent`
- `impl ArraysFrom for LspStoreEvent`
- `impl ArraysInto for LspStoreEvent`
- `impl ComponentsFrom for LspStoreEvent`
- `impl TryComponentsInto for LspStoreEvent`
- `impl UintsFrom for LspStoreEvent`
- `impl UintsInto for LspStoreEvent`
- `impl AdaptIntoUnclamped for LspStoreEvent`
- `impl AdaptInto for LspStoreEvent`
- `impl IntoColor for LspStoreEvent`
- `impl IntoColorUnclamped for LspStoreEvent`
- `impl TryIntoColor for LspStoreEvent`
- `impl FromStimulus for LspStoreEvent`
- `impl IntoStimulus for LspStoreEvent`
- `impl Debug for LspStoreEvent`
- `impl EventEmitter for LspStoreEvent`

## Auto Trait Implementations

`Freeze` `Send` `Sync` `Unpin` `UnsafeUnpin`

