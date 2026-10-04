---
id: LocalLspStore
title: LocalLspStore
---

# Struct: LocalLspStore

Defined in: [`packages/project/src/lsp_store/mod.rs:312`](../../../../packages/project/src/lsp_store/mod.rs#L312)

## Definition

```rust
pub struct LocalLspStore
{
    pub worktree_store: Entity<WorktreeStore>,
    pub language_servers: HashMap<LanguageServerId, LanguageServerState>,
}
```

_（存在非公开字段）_

## Implementations

### running_language_server_for_id()

```rust
pub fn running_language_server_for_id(&self, id: LanguageServerId) -> Option<&Arc<LanguageServer>>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:370`](../../../../packages/project/src/lsp_store/mod.rs#L370)

Returns the running language server for the given ID. Note if the language server is starting, it will not be returned.

#### Parameters

##### id

`LanguageServerId`

#### Returns

`Option<&Arc<LanguageServer>>`


***

### format_ranges_via_lsp()

```rust
pub async fn format_ranges_via_lsp(this: &WeakEntity<LspStore>, buffer_handle: &Entity<Buffer>, ranges: &[Range<Anchor>], abs_path: &Path, adapter: &Arc<CachedLspAdapter>, language_server: &Arc<LanguageServer>, settings: &LanguageSettings, cx: &AsyncApp) -> Result<Option<Vec<(Range<Anchor>, Arc<str>)>>>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:2494`](../../../../packages/project/src/lsp_store/mod.rs#L2494)

#### Parameters

##### this

`&WeakEntity<LspStore>`

##### buffer_handle

`&Entity<Buffer>`

##### ranges

`&[Range<Anchor>]`

##### abs_path

`&Path`

##### adapter

`&Arc<CachedLspAdapter>`

##### language_server

`&Arc<LanguageServer>`

##### settings

`&LanguageSettings`

##### cx

`&AsyncApp`

#### Returns

`Result<Option<Vec<(Range<Anchor>, Arc<str>)>>>`


***

### execute_code_actions_on_server()

```rust
pub async fn execute_code_actions_on_server(lsp_store: &WeakEntity<LspStore>, language_server: &Arc<LanguageServer>, buffer: &Entity<Buffer>, actions: Vec<CodeAction>, push_to_history: bool, project_transaction: &ProjectTransaction, cx: &AsyncApp) -> Result<()>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:3484`](../../../../packages/project/src/lsp_store/mod.rs#L3484)

#### Parameters

##### lsp_store

`&WeakEntity<LspStore>`

##### language_server

`&Arc<LanguageServer>`

##### buffer

`&Entity<Buffer>`

##### actions

`Vec<CodeAction>`

##### push_to_history

`bool`

##### project_transaction

`&ProjectTransaction`

##### cx

`&AsyncApp`

#### Returns

`Result<()>`


***

### deserialize_text_edits()

```rust
pub async fn deserialize_text_edits(this: Entity<LspStore>, buffer_to_edit: Entity<Buffer>, edits: Vec<TextEdit>, push_to_history: bool, _: Arc<CachedLspAdapter>, language_server: Arc<LanguageServer>, cx: &AsyncApp) -> Result<Option<Transaction>>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:3589`](../../../../packages/project/src/lsp_store/mod.rs#L3589)

#### Parameters

##### this

`Entity<LspStore>`

##### buffer_to_edit

`Entity<Buffer>`

##### edits

`Vec<TextEdit>`

##### push_to_history

`bool`

##### _

`Arc<CachedLspAdapter>`

##### language_server

`Arc<LanguageServer>`

##### cx

`&AsyncApp`

#### Returns

`Result<Option<Transaction>>`


***

### edits_from_lsp()

```rust
pub fn edits_from_lsp<impl 'static + Send + IntoIterator<Item = lsp::TextEdit>: 'static + Send + IntoIterator>(&self, buffer: &Entity<Buffer>, lsp_edits: impl ? + ? + ?, server_id: LanguageServerId, version: Option<i32>, cx: &Context<'_, LspStore>) -> Task<Result<Vec<(Range<Anchor>, Arc<str>)>>>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:3632`](../../../../packages/project/src/lsp_store/mod.rs#L3632)

#### Parameters

##### buffer

`&Entity<Buffer>`

##### lsp_edits

`impl ? + ? + ?`

##### server_id

`LanguageServerId`

##### version

`Option<i32>`

##### cx

`&Context<'_, LspStore>`

#### Returns

`Task<Result<Vec<(Range<Anchor>, Arc<str>)>>>`

## Trait Implementations

- `impl Borrow for LocalLspStore`
- `impl BorrowMut for LocalLspStore`
- `impl Into for LocalLspStore`
- `impl From for LocalLspStore`
- `impl TryInto for LocalLspStore`
- `impl TryFrom for LocalLspStore`
- `impl Any for LocalLspStore`
- `impl VZip for LocalLspStore`
- `impl CastableFrom for LocalLspStore`
- `impl CastableFrom for LocalLspStore`
- `impl Read for LocalLspStore`
- `impl IntoEither for LocalLspStore`
- `impl ErasedDestructor for LocalLspStore`
- `impl Same for LocalLspStore`
- `impl Pointable for LocalLspStore`
- `impl Instrument for LocalLspStore`
- `impl WithSubscriber for LocalLspStore`
- `impl FromAngle for LocalLspStore`
- `impl IntoAngle for LocalLspStore`
- `impl IntoCam16Unclamped for LocalLspStore`
- `impl Cam16IntoUnclamped for LocalLspStore`
- `impl ArraysFrom for LocalLspStore`
- `impl ArraysInto for LocalLspStore`
- `impl ComponentsFrom for LocalLspStore`
- `impl TryComponentsInto for LocalLspStore`
- `impl UintsFrom for LocalLspStore`
- `impl UintsInto for LocalLspStore`
- `impl AdaptIntoUnclamped for LocalLspStore`
- `impl AdaptInto for LocalLspStore`
- `impl IntoColor for LocalLspStore`
- `impl IntoColorUnclamped for LocalLspStore`
- `impl TryIntoColor for LocalLspStore`
- `impl FromStimulus for LocalLspStore`
- `impl IntoStimulus for LocalLspStore`

## Auto Trait Implementations

`Freeze` `Unpin` `UnsafeUnpin`

