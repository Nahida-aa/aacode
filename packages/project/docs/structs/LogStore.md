---
id: LogStore
title: LogStore
---

# Struct: LogStore

Defined in: [`packages/project/src/lsp_store/log_store.rs:47`](../../../../packages/project/src/lsp_store/log_store.rs#L47)

## Definition

```rust
pub struct LogStore {
    pub language_servers: HashMap<LanguageServerLogKey, LanguageServerState>,
}
```

_（存在非公开字段）_

## Implementations

### new()

```rust
pub fn new(on_headless_host: bool, cx: &Context<'_, Self>) -> Self
```

Defined in: [`packages/project/src/lsp_store/log_store.rs:484`](../../../../packages/project/src/lsp_store/log_store.rs#L484)

#### Parameters

##### on_headless_host

`bool`

##### cx

`&Context<'_, Self>`

#### Returns

`Self`


***

### add_project()

```rust
pub fn add_project(&self, project: &Entity<Project>, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/lsp_store/log_store.rs:510`](../../../../packages/project/src/lsp_store/log_store.rs#L510)

#### Parameters

##### project

`&Entity<Project>`

##### cx

`&Context<'_, Self>`


***

### get_language_server_state()

```rust
pub fn get_language_server_state(&self, key: &LanguageServerLogKey) -> Option<&LanguageServerState>
```

Defined in: [`packages/project/src/lsp_store/log_store.rs:731`](../../../../packages/project/src/lsp_store/log_store.rs#L731)

#### Parameters

##### key

`&LanguageServerLogKey`

#### Returns

`Option<&LanguageServerState>`


***

### add_language_server()

```rust
pub fn add_language_server(&self, kind: LanguageServerKind, server_id: LanguageServerId, name: Option<LanguageServerName>, worktree_id: Option<WorktreeId>, server: Option<Arc<LanguageServer>>, cx: &Context<'_, Self>) -> Option<&LanguageServerState>
```

Defined in: [`packages/project/src/lsp_store/log_store.rs:738`](../../../../packages/project/src/lsp_store/log_store.rs#L738)

#### Parameters

##### kind

[`LanguageServerKind`](../enums/LanguageServerKind.md)

##### server_id

`LanguageServerId`

##### name

`Option<LanguageServerName>`

##### worktree_id

`Option<WorktreeId>`

##### server

`Option<Arc<LanguageServer>>`

##### cx

`&Context<'_, Self>`

#### Returns

`Option<&LanguageServerState>`


***

### add_language_server_log()

```rust
pub fn add_language_server_log(&self, key: &LanguageServerLogKey, typ: MessageType, message: &str, cx: &Context<'_, Self>) -> Option<()>
```

Defined in: [`packages/project/src/lsp_store/log_store.rs:806`](../../../../packages/project/src/lsp_store/log_store.rs#L806)

#### Parameters

##### key

`&LanguageServerLogKey`

##### typ

`MessageType`

##### message

`&str`

##### cx

`&Context<'_, Self>`

#### Returns

`Option<()>`


***

### remove_language_server()

```rust
pub fn remove_language_server(&self, key: &LanguageServerLogKey, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/lsp_store/log_store.rs:973`](../../../../packages/project/src/lsp_store/log_store.rs#L973)

#### Parameters

##### key

`&LanguageServerLogKey`

##### cx

`&Context<'_, Self>`


***

### server_logs()

```rust
pub fn server_logs(&self, key: &LanguageServerLogKey) -> Option<&VecDeque<LogMessage>>
```

Defined in: [`packages/project/src/lsp_store/log_store.rs:978`](../../../../packages/project/src/lsp_store/log_store.rs#L978)

#### Parameters

##### key

`&LanguageServerLogKey`

#### Returns

`Option<&VecDeque<LogMessage>>`


***

### server_trace()

```rust
pub fn server_trace(&self, key: &LanguageServerLogKey) -> Option<&VecDeque<TraceMessage>>
```

Defined in: [`packages/project/src/lsp_store/log_store.rs:982`](../../../../packages/project/src/lsp_store/log_store.rs#L982)

#### Parameters

##### key

`&LanguageServerLogKey`

#### Returns

`Option<&VecDeque<TraceMessage>>`


***

### server_keys_for_project()

```rust
pub fn server_keys_for_project<'a>(&self, project: &'a WeakEntity<Project>, lsp_store: &'a WeakEntity<LspStore>) -> impl ? + ?
```

Defined in: [`packages/project/src/lsp_store/log_store.rs:986`](../../../../packages/project/src/lsp_store/log_store.rs#L986)

#### Parameters

##### project

`&'a WeakEntity<Project>`

##### lsp_store

`&'a WeakEntity<LspStore>`

#### Returns

`impl ? + ?`


***

### has_server_logs()

```rust
pub fn has_server_logs(&self, server: &LanguageServerSelector, project: &WeakEntity<Project>, lsp_store: &WeakEntity<LspStore>) -> bool
```

Defined in: [`packages/project/src/lsp_store/log_store.rs:997`](../../../../packages/project/src/lsp_store/log_store.rs#L997)

#### Parameters

##### server

`&LanguageServerSelector`

##### project

`&WeakEntity<Project>`

##### lsp_store

`&WeakEntity<LspStore>`

#### Returns

`bool`


***

### retain_view_log_stream()

```rust
pub fn retain_view_log_stream(&self, key: &LanguageServerLogKey, log_kind: LogKind, cx: &Context<'_, Self>) -> Option<()>
```

Defined in: [`packages/project/src/lsp_store/log_store.rs:1072`](../../../../packages/project/src/lsp_store/log_store.rs#L1072)

#### Parameters

##### key

`&LanguageServerLogKey`

##### log_kind

[`LogKind`](../enums/LogKind.md)

##### cx

`&Context<'_, Self>`

#### Returns

`Option<()>`


***

### release_view_log_stream()

```rust
pub fn release_view_log_stream(&self, key: &LanguageServerLogKey, log_kind: LogKind, cx: &Context<'_, Self>) -> Option<()>
```

Defined in: [`packages/project/src/lsp_store/log_store.rs:1088`](../../../../packages/project/src/lsp_store/log_store.rs#L1088)

#### Parameters

##### key

`&LanguageServerLogKey`

##### log_kind

[`LogKind`](../enums/LogKind.md)

##### cx

`&Context<'_, Self>`

#### Returns

`Option<()>`


***

### set_downstream_log_stream()

```rust
pub fn set_downstream_log_stream(&self, key: &LanguageServerLogKey, peer_id: PeerId, log_kind: LogKind, enabled: bool, cx: &Context<'_, Self>) -> Option<()>
```

Defined in: [`packages/project/src/lsp_store/log_store.rs:1108`](../../../../packages/project/src/lsp_store/log_store.rs#L1108)

#### Parameters

##### key

`&LanguageServerLogKey`

##### peer_id

`PeerId`

##### log_kind

[`LogKind`](../enums/LogKind.md)

##### enabled

`bool`

##### cx

`&Context<'_, Self>`

#### Returns

`Option<()>`


***

### sync_copilot_for_project()

```rust
pub fn sync_copilot_for_project(&self, project: &WeakEntity<Project>, server: Option<Arc<LanguageServer>>, cx: &Context<'_, Self>) -> Option<()>
```

Defined in: [`packages/project/src/lsp_store/log_store.rs:1250`](../../../../packages/project/src/lsp_store/log_store.rs#L1250)

#### Parameters

##### project

`&WeakEntity<Project>`

##### server

`Option<Arc<LanguageServer>>`

##### cx

`&Context<'_, Self>`

#### Returns

`Option<()>`

## Trait Implementations

- `impl Borrow for LogStore`
- `impl BorrowMut for LogStore`
- `impl Into for LogStore`
- `impl From for LogStore`
- `impl TryInto for LogStore`
- `impl TryFrom for LogStore`
- `impl Any for LogStore`
- `impl VZip for LogStore`
- `impl CastableFrom for LogStore`
- `impl CastableFrom for LogStore`
- `impl Read for LogStore`
- `impl IntoEither for LogStore`
- `impl ErasedDestructor for LogStore`
- `impl Same for LogStore`
- `impl Pointable for LogStore`
- `impl Instrument for LogStore`
- `impl WithSubscriber for LogStore`
- `impl FromAngle for LogStore`
- `impl IntoAngle for LogStore`
- `impl IntoCam16Unclamped for LogStore`
- `impl Cam16IntoUnclamped for LogStore`
- `impl ArraysFrom for LogStore`
- `impl ArraysInto for LogStore`
- `impl ComponentsFrom for LogStore`
- `impl TryComponentsInto for LogStore`
- `impl UintsFrom for LogStore`
- `impl UintsInto for LogStore`
- `impl AdaptIntoUnclamped for LogStore`
- `impl AdaptInto for LogStore`
- `impl IntoColor for LogStore`
- `impl IntoColorUnclamped for LogStore`
- `impl TryIntoColor for LogStore`
- `impl FromStimulus for LogStore`
- `impl IntoStimulus for LogStore`
- `impl EventEmitter for LogStore`

## Auto Trait Implementations

`Freeze` `Unpin` `UnsafeUnpin`

