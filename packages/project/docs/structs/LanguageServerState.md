---
id: LanguageServerState
title: LanguageServerState
---

# Struct: LanguageServerState

Defined in: [`packages/project/src/lsp_store/log_store.rs:135`](../../../../packages/project/src/lsp_store/log_store.rs#L135)

## Definition

```rust
pub struct LanguageServerState {
    // Distinguishes a re-registered key from the registration owned by an older view.
    pub generation: usize,
    pub name: Option<LanguageServerName>,
    pub worktree_id: Option<WorktreeId>,
    pub rpc_state: Option<LanguageServerRpcState>,
    pub trace_level: TraceValue,
    pub log_level: MessageType,
}
```

## Fields

### generation

Defined in: [`packages/project/src/lsp_store/log_store.rs:137`](../../../../packages/project/src/lsp_store/log_store.rs#L137)

Distinguishes a re-registered key from the registration owned by an older view.

_（存在非公开字段）_

## Implementations

### server()

```rust
pub fn server(&self) -> Option<Arc<LanguageServer>>
```

Defined in: [`packages/project/src/lsp_store/log_store.rs:152`](../../../../packages/project/src/lsp_store/log_store.rs#L152)

#### Returns

`Option<Arc<LanguageServer>>`

## Trait Implementations

- `impl Borrow for LanguageServerState`
- `impl BorrowMut for LanguageServerState`
- `impl Into for LanguageServerState`
- `impl From for LanguageServerState`
- `impl TryInto for LanguageServerState`
- `impl TryFrom for LanguageServerState`
- `impl Any for LanguageServerState`
- `impl VZip for LanguageServerState`
- `impl CastableFrom for LanguageServerState`
- `impl CastableFrom for LanguageServerState`
- `impl Read for LanguageServerState`
- `impl IntoEither for LanguageServerState`
- `impl ErasedDestructor for LanguageServerState`
- `impl Same for LanguageServerState`
- `impl Pointable for LanguageServerState`
- `impl Instrument for LanguageServerState`
- `impl WithSubscriber for LanguageServerState`
- `impl FromAngle for LanguageServerState`
- `impl IntoAngle for LanguageServerState`
- `impl IntoCam16Unclamped for LanguageServerState`
- `impl Cam16IntoUnclamped for LanguageServerState`
- `impl ArraysFrom for LanguageServerState`
- `impl ArraysInto for LanguageServerState`
- `impl ComponentsFrom for LanguageServerState`
- `impl TryComponentsInto for LanguageServerState`
- `impl UintsFrom for LanguageServerState`
- `impl UintsInto for LanguageServerState`
- `impl AdaptIntoUnclamped for LanguageServerState`
- `impl AdaptInto for LanguageServerState`
- `impl IntoColor for LanguageServerState`
- `impl IntoColorUnclamped for LanguageServerState`
- `impl TryIntoColor for LanguageServerState`
- `impl FromStimulus for LanguageServerState`
- `impl IntoStimulus for LanguageServerState`
- `impl Debug for LanguageServerState`

## Auto Trait Implementations

`Freeze` `Send` `Sync` `Unpin` `UnsafeUnpin`

