---
id: LocalLspAdapterDelegate
title: LocalLspAdapterDelegate
---

# Struct: LocalLspAdapterDelegate

Defined in: [`packages/project/src/lsp_store/mod.rs:16402`](../../../../packages/project/src/lsp_store/mod.rs#L16402)

## Definition

```rust
pub struct LocalLspAdapterDelegate
```

_（存在非公开字段）_

## Implementations

### new()

```rust
pub fn new(language_registry: Arc<LanguageRegistry>, environment: &Entity<ProjectEnvironment>, lsp_store: WeakEntity<LspStore>, worktree: &Entity<Worktree>, http_client: Arc<dyn HttpClient>, fs: Arc<dyn Fs>, cx: &App) -> Arc<Self>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:16412`](../../../../packages/project/src/lsp_store/mod.rs#L16412)

#### Parameters

##### language_registry

`Arc<LanguageRegistry>`

##### environment

`&Entity<ProjectEnvironment>`

##### lsp_store

`WeakEntity<LspStore>`

##### worktree

`&Entity<Worktree>`

##### http_client

`Arc<dyn HttpClient>`

##### fs

`Arc<dyn Fs>`

##### cx

`&App`

#### Returns

`Arc<Self>`


***

### from_local_lsp()

```rust
pub fn from_local_lsp(local: &LocalLspStore, worktree: &Entity<Worktree>, cx: &App) -> Arc<Self>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:16434`](../../../../packages/project/src/lsp_store/mod.rs#L16434)

#### Parameters

##### local

`&LocalLspStore`

##### worktree

`&Entity<Worktree>`

##### cx

`&App`

#### Returns

`Arc<Self>`

## Trait Implementations

- `impl Borrow for LocalLspAdapterDelegate`
- `impl BorrowMut for LocalLspAdapterDelegate`
- `impl Into for LocalLspAdapterDelegate`
- `impl From for LocalLspAdapterDelegate`
- `impl TryInto for LocalLspAdapterDelegate`
- `impl TryFrom for LocalLspAdapterDelegate`
- `impl Any for LocalLspAdapterDelegate`
- `impl VZip for LocalLspAdapterDelegate`
- `impl CastableFrom for LocalLspAdapterDelegate`
- `impl CastableFrom for LocalLspAdapterDelegate`
- `impl Read for LocalLspAdapterDelegate`
- `impl IntoEither for LocalLspAdapterDelegate`
- `impl ErasedDestructor for LocalLspAdapterDelegate`
- `impl Same for LocalLspAdapterDelegate`
- `impl Pointable for LocalLspAdapterDelegate`
- `impl Instrument for LocalLspAdapterDelegate`
- `impl WithSubscriber for LocalLspAdapterDelegate`
- `impl FromAngle for LocalLspAdapterDelegate`
- `impl IntoAngle for LocalLspAdapterDelegate`
- `impl IntoCam16Unclamped for LocalLspAdapterDelegate`
- `impl Cam16IntoUnclamped for LocalLspAdapterDelegate`
- `impl ArraysFrom for LocalLspAdapterDelegate`
- `impl ArraysInto for LocalLspAdapterDelegate`
- `impl ComponentsFrom for LocalLspAdapterDelegate`
- `impl TryComponentsInto for LocalLspAdapterDelegate`
- `impl UintsFrom for LocalLspAdapterDelegate`
- `impl UintsInto for LocalLspAdapterDelegate`
- `impl AdaptIntoUnclamped for LocalLspAdapterDelegate`
- `impl AdaptInto for LocalLspAdapterDelegate`
- `impl IntoColor for LocalLspAdapterDelegate`
- `impl IntoColorUnclamped for LocalLspAdapterDelegate`
- `impl TryIntoColor for LocalLspAdapterDelegate`
- `impl FromStimulus for LocalLspAdapterDelegate`
- `impl IntoStimulus for LocalLspAdapterDelegate`
- `impl LspAdapterDelegate for LocalLspAdapterDelegate`

## Auto Trait Implementations

`Freeze` `Send` `Sync` `Unpin` `UnsafeUnpin`

