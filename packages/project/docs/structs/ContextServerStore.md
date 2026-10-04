---
id: ContextServerStore
title: ContextServerStore
---

# Struct: ContextServerStore

Defined in: [`packages/project/src/context_server_store/mod.rs:287`](../../../../packages/project/src/context_server_store/mod.rs#L287)

## Definition

```rust
pub struct ContextServerStore
```

_（存在非公开字段）_

## Implementations

### local()

```rust
pub fn local(worktree_store: Entity<WorktreeStore>, weak_project: Option<WeakEntity<Project>>, headless: bool, cx: &Context<'_, Self>) -> Self
```

Defined in: [`packages/project/src/context_server_store/mod.rs:317`](../../../../packages/project/src/context_server_store/mod.rs#L317)

#### Parameters

##### worktree_store

`Entity<WorktreeStore>`

##### weak_project

`Option<WeakEntity<Project>>`

##### headless

`bool`

##### cx

`&Context<'_, Self>`

#### Returns

`Self`


***

### remote()

```rust
pub fn remote(project_id: u64, upstream_client: Entity<RemoteClient>, worktree_store: Entity<WorktreeStore>, weak_project: Option<WeakEntity<Project>>, cx: &Context<'_, Self>) -> Self
```

Defined in: [`packages/project/src/context_server_store/mod.rs:337`](../../../../packages/project/src/context_server_store/mod.rs#L337)

#### Parameters

##### project_id

`u64`

##### upstream_client

`Entity<RemoteClient>`

##### worktree_store

`Entity<WorktreeStore>`

##### weak_project

`Option<WeakEntity<Project>>`

##### cx

`&Context<'_, Self>`

#### Returns

`Self`


***

### init_headless()

```rust
pub fn init_headless(session: &AnyProtoClient)
```

Defined in: [`packages/project/src/context_server_store/mod.rs:358`](../../../../packages/project/src/context_server_store/mod.rs#L358)

#### Parameters

##### session

`&AnyProtoClient`


***

### shared()

```rust
pub fn shared(&self, project_id: u64, client: AnyProtoClient)
```

Defined in: [`packages/project/src/context_server_store/mod.rs:362`](../../../../packages/project/src/context_server_store/mod.rs#L362)

#### Parameters

##### project_id

`u64`

##### client

`AnyProtoClient`


***

### is_remote_project()

```rust
pub fn is_remote_project(&self) -> bool
```

Defined in: [`packages/project/src/context_server_store/mod.rs:371`](../../../../packages/project/src/context_server_store/mod.rs#L371)

#### Returns

`bool`


***

### configured_server_ids()

```rust
pub fn configured_server_ids(&self) -> Vec<ContextServerId>
```

Defined in: [`packages/project/src/context_server_store/mod.rs:376`](../../../../packages/project/src/context_server_store/mod.rs#L376)

Returns all configured context server ids, excluding the ones that are disabled

#### Returns

`Vec<ContextServerId>`


***

### get_server()

```rust
pub fn get_server(&self, id: &ContextServerId) -> Option<Arc<ContextServer>>
```

Defined in: [`packages/project/src/context_server_store/mod.rs:527`](../../../../packages/project/src/context_server_store/mod.rs#L527)

#### Parameters

##### id

`&ContextServerId`

#### Returns

`Option<Arc<ContextServer>>`


***

### get_running_server()

```rust
pub fn get_running_server(&self, id: &ContextServerId) -> Option<Arc<ContextServer>>
```

Defined in: [`packages/project/src/context_server_store/mod.rs:531`](../../../../packages/project/src/context_server_store/mod.rs#L531)

#### Parameters

##### id

`&ContextServerId`

#### Returns

`Option<Arc<ContextServer>>`


***

### status_for_server()

```rust
pub fn status_for_server(&self, id: &ContextServerId) -> Option<ContextServerStatus>
```

Defined in: [`packages/project/src/context_server_store/mod.rs:539`](../../../../packages/project/src/context_server_store/mod.rs#L539)

#### Parameters

##### id

`&ContextServerId`

#### Returns

`Option<ContextServerStatus>`


***

### configuration_for_server()

```rust
pub fn configuration_for_server(&self, id: &ContextServerId) -> Option<Arc<ContextServerConfiguration>>
```

Defined in: [`packages/project/src/context_server_store/mod.rs:543`](../../../../packages/project/src/context_server_store/mod.rs#L543)

#### Parameters

##### id

`&ContextServerId`

#### Returns

`Option<Arc<ContextServerConfiguration>>`


***

### settings_for_server()

```rust
pub fn settings_for_server(&self, id: &ContextServerId) -> Option<&ContextServerSettings>
```

Defined in: [`packages/project/src/context_server_store/mod.rs:553`](../../../../packages/project/src/context_server_store/mod.rs#L553)

Returns the configured settings for a server, if it is present in the user
or project settings. This is available regardless of whether the server is
currently running, unlike [`Self::configuration_for_server`](ContextServerStore.md).

#### Parameters

##### id

`&ContextServerId`

#### Returns

`Option<&ContextServerSettings>`


***

### is_extension_provided()

```rust
pub fn is_extension_provided(&self, id: &ContextServerId, cx: &App) -> bool
```

Defined in: [`packages/project/src/context_server_store/mod.rs:565`](../../../../packages/project/src/context_server_store/mod.rs#L565)

Returns whether a server is provided by an extension (as opposed to a
custom Stdio/HTTP server configured directly in settings).

This is derived from the configured settings rather than the runtime
configuration, so it stays correct even when a custom server is disabled
or has not been started yet (in which case it has no runtime state).

#### Parameters

##### id

`&ContextServerId`

##### cx

`&App`

#### Returns

`bool`


***

### is_server_enabled()

```rust
pub fn is_server_enabled(&self, id: &ContextServerId, cx: &App) -> bool
```

Defined in: [`packages/project/src/context_server_store/mod.rs:583`](../../../../packages/project/src/context_server_store/mod.rs#L583)

Returns whether a server is enabled.
Servers with no settings entry only originate from an extension
descriptor in the registry, and those are enabled by default
([`ContextServerSettings::default_extension`](../enums/ContextServerSettings.md)).

#### Parameters

##### id

`&ContextServerId`

##### cx

`&App`

#### Returns

`bool`


***

### server_ids()

```rust
pub fn server_ids(&self) -> &[ContextServerId]
```

Defined in: [`packages/project/src/context_server_store/mod.rs:597`](../../../../packages/project/src/context_server_store/mod.rs#L597)

Returns a sorted slice of available unique context server IDs. Within the
slice, context servers which have `mcp-server-` as a prefix in their ID will
appear after servers that do not have this prefix in their ID.

#### Returns

`&[ContextServerId]`


***

### running_servers()

```rust
pub fn running_servers(&self) -> Vec<Arc<ContextServer>>
```

Defined in: [`packages/project/src/context_server_store/mod.rs:634`](../../../../packages/project/src/context_server_store/mod.rs#L634)

#### Returns

`Vec<Arc<ContextServer>>`


***

### start_server()

```rust
pub fn start_server(&self, server: Arc<ContextServer>, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/context_server_store/mod.rs:647`](../../../../packages/project/src/context_server_store/mod.rs#L647)

#### Parameters

##### server

`Arc<ContextServer>`

##### cx

`&Context<'_, Self>`


***

### stop_server()

```rust
pub fn stop_server(&self, id: &ContextServerId, cx: &Context<'_, Self>) -> Result<()>
```

Defined in: [`packages/project/src/context_server_store/mod.rs:682`](../../../../packages/project/src/context_server_store/mod.rs#L682)

#### Parameters

##### id

`&ContextServerId`

##### cx

`&Context<'_, Self>`

#### Returns

`Result<()>`


***

### create_context_server()

```rust
pub async fn create_context_server(this: WeakEntity<Self>, id: ContextServerId, configuration: Arc<ContextServerConfiguration>, cx: &AsyncApp) -> Result<(Arc<ContextServer>, Arc<ContextServerConfiguration>)>
```

Defined in: [`packages/project/src/context_server_store/mod.rs:911`](../../../../packages/project/src/context_server_store/mod.rs#L911)

#### Parameters

##### this

`WeakEntity<Self>`

##### id

`ContextServerId`

##### configuration

`Arc<ContextServerConfiguration>`

##### cx

`&AsyncApp`

#### Returns

`Result<(Arc<ContextServer>, Arc<ContextServerConfiguration>)>`


***

### authenticate_server()

```rust
pub fn authenticate_server(&self, id: &ContextServerId, cx: &Context<'_, Self>) -> Result<()>
```

Defined in: [`packages/project/src/context_server_store/mod.rs:1207`](../../../../packages/project/src/context_server_store/mod.rs#L1207)

Initiate the OAuth browser flow for a server in the `AuthRequired` state.

This starts a loopback HTTP callback server on an ephemeral port, builds
the authorization URL, opens the user's browser, waits for the callback,
exchanges the code for tokens, persists them in the keychain, and restarts
the server with the new token provider.

#### Parameters

##### id

`&ContextServerId`

##### cx

`&Context<'_, Self>`

#### Returns

`Result<()>`


***

### submit_client_secret()

```rust
pub fn submit_client_secret(&self, id: &ContextServerId, secret: String, cx: &Context<'_, Self>) -> Result<()>
```

Defined in: [`packages/project/src/context_server_store/mod.rs:1309`](../../../../packages/project/src/context_server_store/mod.rs#L1309)

Store the client secret and proceed with authentication.

#### Parameters

##### id

`&ContextServerId`

##### secret

`String`

##### cx

`&Context<'_, Self>`

#### Returns

`Result<()>`


***

### store_client_secret()

```rust
pub async fn store_client_secret(credentials_provider: &Arc<dyn CredentialsProvider>, server_url: &Url, secret: &str, cx: &AsyncApp) -> Result<()>
```

Defined in: [`packages/project/src/context_server_store/mod.rs:1628`](../../../../packages/project/src/context_server_store/mod.rs#L1628)

#### Parameters

##### credentials_provider

`&Arc<dyn CredentialsProvider>`

##### server_url

`&Url`

##### secret

`&str`

##### cx

`&AsyncApp`

#### Returns

`Result<()>`


***

### logout_server()

```rust
pub fn logout_server(&self, id: &ContextServerId, cx: &Context<'_, Self>) -> Result<()>
```

Defined in: [`packages/project/src/context_server_store/mod.rs:1651`](../../../../packages/project/src/context_server_store/mod.rs#L1651)

Log out of an OAuth-authenticated MCP server: clear the stored OAuth
session from the keychain and stop the server.

#### Parameters

##### id

`&ContextServerId`

##### cx

`&Context<'_, Self>`

#### Returns

`Result<()>`

## Trait Implementations

- `impl Borrow for ContextServerStore`
- `impl BorrowMut for ContextServerStore`
- `impl Into for ContextServerStore`
- `impl From for ContextServerStore`
- `impl TryInto for ContextServerStore`
- `impl TryFrom for ContextServerStore`
- `impl Any for ContextServerStore`
- `impl VZip for ContextServerStore`
- `impl CastableFrom for ContextServerStore`
- `impl CastableFrom for ContextServerStore`
- `impl Read for ContextServerStore`
- `impl IntoEither for ContextServerStore`
- `impl ErasedDestructor for ContextServerStore`
- `impl Same for ContextServerStore`
- `impl Pointable for ContextServerStore`
- `impl Instrument for ContextServerStore`
- `impl WithSubscriber for ContextServerStore`
- `impl FromAngle for ContextServerStore`
- `impl IntoAngle for ContextServerStore`
- `impl IntoCam16Unclamped for ContextServerStore`
- `impl Cam16IntoUnclamped for ContextServerStore`
- `impl ArraysFrom for ContextServerStore`
- `impl ArraysInto for ContextServerStore`
- `impl ComponentsFrom for ContextServerStore`
- `impl TryComponentsInto for ContextServerStore`
- `impl UintsFrom for ContextServerStore`
- `impl UintsInto for ContextServerStore`
- `impl AdaptIntoUnclamped for ContextServerStore`
- `impl AdaptInto for ContextServerStore`
- `impl IntoColor for ContextServerStore`
- `impl IntoColorUnclamped for ContextServerStore`
- `impl TryIntoColor for ContextServerStore`
- `impl FromStimulus for ContextServerStore`
- `impl IntoStimulus for ContextServerStore`
- `impl EventEmitter for ContextServerStore`

## Auto Trait Implementations

`Freeze` `Unpin` `UnsafeUnpin`

