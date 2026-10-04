---
id: Event
title: Event
---

# Enum: Event

Defined in: [`packages/project/src/event.rs:31`](../../../../packages/project/src/event.rs#L31)

Semantics-aware entity that is relevant to one or more [`Worktree`] with the files.
`Project` is responsible for tasks, LSP and collab queries, synchronizing worktree states accordingly.
Maps [`Worktree`] entries with its own logic using `ProjectEntryId` and [`ProjectPath`](../structs/ProjectPath.md) structs.

Can be either local (for the project opened on the same host) or remote.(for collab projects, browsed by multiple remote users).

## Definition

```rust
pub enum Event {
    LanguageServerAdded(LanguageServerId, LanguageServerName, Option<WorktreeId>),
    SupplementaryLanguageServerAdded(LanguageServerId, LanguageServerName),
    LanguageServerRemoved(LanguageServerId),
    SupplementaryLanguageServerRemoved(LanguageServerId),
    LanguageServerLog(LanguageServerId, LanguageServerLogType, String),
    LanguageServerBufferRegistered{ .. },
    ToggleLspLogs{ .. },
    Toast{ .. },
    HideToast{ .. },
    LanguageServerPrompt(LanguageServerPromptRequest),
    LanguageServerShowDocument(LanguageServerShowDocumentRequest),
    LanguageNotFound(Entity<Buffer>),
    ActiveEntryChanged(Option<ProjectEntryId>),
    ActivateProjectPanel,
    WorktreeAdded(WorktreeId),
    WorktreeOrderChanged,
    WorktreeRemoved(WorktreeId),
    WorktreeUpdatedEntries(WorktreeId, UpdatedEntriesSet),
    WorktreeUpdatedRootRepoCommonDir(WorktreeId),
    WorktreePathsChanged{ .. },
    DiskBasedDiagnosticsStarted{ .. },
    DiskBasedDiagnosticsFinished{ .. },
    DiagnosticsUpdated{ .. },
    RemoteIdChanged(Option<u64>),
    DisconnectedFromHost,
    DisconnectedFromRemote{ .. },
    Closed,
    DeletedEntry(WorktreeId, ProjectEntryId),
    CollaboratorUpdated{ .. },
    CollaboratorJoined(PeerId),
    CollaboratorLeft(PeerId),
    HostReshared,
    Reshared,
    Rejoined,
    RefreshInlayHints{ .. },
    RefreshSemanticTokens{ .. },
    RefreshCodeLens{ .. },
    RefreshDocumentColors{ .. },
    RefreshDocumentLinks{ .. },
    RefreshDocumentHighlights{ .. },
    RefreshFoldingRanges{ .. },
    RefreshDocumentSymbols{ .. },
    RevealInProjectPanel(ProjectEntryId),
    SnippetEdit(BufferId, Vec<(Range, Snippet)>),
    ExpandedAllForEntry(WorktreeId, ProjectEntryId),
    EntryRenamed{ .. },
    WorkspaceEditApplied(ProjectTransaction),
    AgentLocationChanged,
    BufferEdited{ .. },
}
```

## Trait Implementations

- `impl Borrow for Event`
- `impl BorrowMut for Event`
- `impl CloneToUninit for Event`
- `impl Into for Event`
- `impl From for Event`
- `impl TryInto for Event`
- `impl TryFrom for Event`
- `impl Any for Event`
- `impl ToOwned for Event`
- `impl DynClone for Event`
- `impl VZip for Event`
- `impl CastableFrom for Event`
- `impl CastableFrom for Event`
- `impl Read for Event`
- `impl IntoEither for Event`
- `impl ErasedDestructor for Event`
- `impl Same for Event`
- `impl Pointable for Event`
- `impl Instrument for Event`
- `impl WithSubscriber for Event`
- `impl FromAngle for Event`
- `impl IntoAngle for Event`
- `impl IntoCam16Unclamped for Event`
- `impl Cam16IntoUnclamped for Event`
- `impl ArraysFrom for Event`
- `impl ArraysInto for Event`
- `impl ComponentsFrom for Event`
- `impl TryComponentsInto for Event`
- `impl UintsFrom for Event`
- `impl UintsInto for Event`
- `impl AdaptIntoUnclamped for Event`
- `impl AdaptInto for Event`
- `impl IntoColor for Event`
- `impl IntoColorUnclamped for Event`
- `impl TryIntoColor for Event`
- `impl FromStimulus for Event`
- `impl IntoStimulus for Event`
- `impl EventEmitter for Event`
- `impl Clone for Event`
- `impl Debug for Event`
- `impl StructuralPartialEq for Event`
- `impl PartialEq for Event`
- `impl EventEmitter for Event`

## Auto Trait Implementations

`Freeze` `Send` `Sync` `Unpin` `UnsafeUnpin`

