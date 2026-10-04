---
id: LanguageServerStatus
title: LanguageServerStatus
---

# Struct: LanguageServerStatus

Defined in: [`packages/project/src/lsp_store/mod.rs:4705`](../../../../packages/project/src/lsp_store/mod.rs#L4705)

## Definition

```rust
pub struct LanguageServerStatus {
    pub name: LanguageServerName,
    pub language_name: Option<LanguageName>,
    pub server_version: Option<SharedString>,
    pub server_readable_version: Option<SharedString>,
    pub pending_work: BTreeMap<ProgressToken, LanguageServerProgress>,
    pub has_pending_diagnostic_updates: bool,
    pub progress_tokens: HashSet<ProgressToken>,
    pub worktree: Option<WorktreeId>,
    pub binary: Option<LanguageServerBinary>,
    pub configuration: Option<Value>,
    pub workspace_folders: BTreeSet<Uri>,
    pub process_id: Option<u32>,
}
```

## Trait Implementations

- `impl Borrow for LanguageServerStatus`
- `impl BorrowMut for LanguageServerStatus`
- `impl CloneToUninit for LanguageServerStatus`
- `impl Into for LanguageServerStatus`
- `impl From for LanguageServerStatus`
- `impl TryInto for LanguageServerStatus`
- `impl TryFrom for LanguageServerStatus`
- `impl Any for LanguageServerStatus`
- `impl ToOwned for LanguageServerStatus`
- `impl Serialize for LanguageServerStatus`
- `impl DynClone for LanguageServerStatus`
- `impl VZip for LanguageServerStatus`
- `impl CastableFrom for LanguageServerStatus`
- `impl CastableFrom for LanguageServerStatus`
- `impl Read for LanguageServerStatus`
- `impl IntoEither for LanguageServerStatus`
- `impl ErasedDestructor for LanguageServerStatus`
- `impl Same for LanguageServerStatus`
- `impl Pointable for LanguageServerStatus`
- `impl Instrument for LanguageServerStatus`
- `impl WithSubscriber for LanguageServerStatus`
- `impl FromAngle for LanguageServerStatus`
- `impl IntoAngle for LanguageServerStatus`
- `impl IntoCam16Unclamped for LanguageServerStatus`
- `impl Cam16IntoUnclamped for LanguageServerStatus`
- `impl ArraysFrom for LanguageServerStatus`
- `impl ArraysInto for LanguageServerStatus`
- `impl ComponentsFrom for LanguageServerStatus`
- `impl TryComponentsInto for LanguageServerStatus`
- `impl UintsFrom for LanguageServerStatus`
- `impl UintsInto for LanguageServerStatus`
- `impl AdaptIntoUnclamped for LanguageServerStatus`
- `impl AdaptInto for LanguageServerStatus`
- `impl IntoColor for LanguageServerStatus`
- `impl IntoColorUnclamped for LanguageServerStatus`
- `impl TryIntoColor for LanguageServerStatus`
- `impl FromStimulus for LanguageServerStatus`
- `impl IntoStimulus for LanguageServerStatus`
- `impl Clone for LanguageServerStatus`
- `impl Debug for LanguageServerStatus`
- `impl Serialize for LanguageServerStatus`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

