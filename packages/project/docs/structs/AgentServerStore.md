---
id: AgentServerStore
title: AgentServerStore
---

# Struct: AgentServerStore

Defined in: [`packages/project/src/agent_server_store.rs:187`](../../../../packages/project/src/agent_server_store.rs#L187)

## Definition

```rust
pub struct AgentServerStore
{
    pub external_agents: HashMap<AgentId, ExternalAgentEntry>,
}
```

_（存在非公开字段）_

## Implementations

### migrate_agent_server_from_extensions()

```rust
pub fn migrate_agent_server_from_extensions(&self, id: Arc<str>, fs: Arc<dyn Fs>, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/agent_server_store.rs:213`](../../../../packages/project/src/agent_server_store.rs#L213)

#### Parameters

##### id

`Arc<str>`

##### fs

`Arc<dyn Fs>`

##### cx

`&Context<'_, Self>`


***

### agent_icon()

```rust
pub fn agent_icon(&self, id: &AgentId) -> Option<SharedString>
```

Defined in: [`packages/project/src/agent_server_store.rs:244`](../../../../packages/project/src/agent_server_store.rs#L244)

#### Parameters

##### id

`&AgentId`

#### Returns

`Option<SharedString>`


***

### agent_source()

```rust
pub fn agent_source(&self, name: &AgentId) -> Option<ExternalAgentSource>
```

Defined in: [`packages/project/src/agent_server_store.rs:250`](../../../../packages/project/src/agent_server_store.rs#L250)

#### Parameters

##### name

`&AgentId`

#### Returns

`Option<ExternalAgentSource>`


***

### agent_display_name()

```rust
pub fn agent_display_name(&self, name: &AgentId) -> Option<SharedString>
```

Defined in: [`packages/project/src/agent_server_store.rs:256`](../../../../packages/project/src/agent_server_store.rs#L256)

#### Parameters

##### name

`&AgentId`

#### Returns

`Option<SharedString>`


***

### init_remote()

```rust
pub fn init_remote(session: &AnyProtoClient)
```

Defined in: [`packages/project/src/agent_server_store.rs:262`](../../../../packages/project/src/agent_server_store.rs#L262)

#### Parameters

##### session

`&AnyProtoClient`


***

### init_headless()

```rust
pub fn init_headless(session: &AnyProtoClient)
```

Defined in: [`packages/project/src/agent_server_store.rs:268`](../../../../packages/project/src/agent_server_store.rs#L268)

#### Parameters

##### session

`&AnyProtoClient`


***

### node_runtime()

```rust
pub fn node_runtime(&self) -> Option<NodeRuntime>
```

Defined in: [`packages/project/src/agent_server_store.rs:492`](../../../../packages/project/src/agent_server_store.rs#L492)

#### Returns

`Option<NodeRuntime>`


***

### local()

```rust
pub fn local(node_runtime: NodeRuntime, fs: Arc<dyn Fs>, project_environment: Entity<ProjectEnvironment>, http_client: Arc<dyn HttpClient>, cx: &Context<'_, Self>) -> Self
```

Defined in: [`packages/project/src/agent_server_store.rs:499`](../../../../packages/project/src/agent_server_store.rs#L499)

#### Parameters

##### node_runtime

`NodeRuntime`

##### fs

`Arc<dyn Fs>`

##### project_environment

`Entity<ProjectEnvironment>`

##### http_client

`Arc<dyn HttpClient>`

##### cx

`&Context<'_, Self>`

#### Returns

`Self`


***

### collab()

```rust
pub fn collab() -> Self
```

Defined in: [`packages/project/src/agent_server_store.rs:545`](../../../../packages/project/src/agent_server_store.rs#L545)

#### Returns

`Self`


***

### shared()

```rust
pub fn shared(&self, project_id: u64, client: AnyProtoClient, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/agent_server_store.rs:552`](../../../../packages/project/src/agent_server_store.rs#L552)

#### Parameters

##### project_id

`u64`

##### client

`AnyProtoClient`

##### cx

`&Context<'_, Self>`


***

### get_external_agent()

```rust
pub fn get_external_agent(&self, name: &AgentId) -> Option<&dyn ExternalAgentServer>
```

Defined in: [`packages/project/src/agent_server_store.rs:586`](../../../../packages/project/src/agent_server_store.rs#L586)

#### Parameters

##### name

`&AgentId`

#### Returns

`Option<&dyn ExternalAgentServer>`


***

### no_browser()

```rust
pub fn no_browser(&self) -> bool
```

Defined in: [`packages/project/src/agent_server_store.rs:595`](../../../../packages/project/src/agent_server_store.rs#L595)

#### Returns

`bool`


***

### has_external_agents()

```rust
pub fn has_external_agents(&self) -> bool
```

Defined in: [`packages/project/src/agent_server_store.rs:606`](../../../../packages/project/src/agent_server_store.rs#L606)

#### Returns

`bool`


***

### external_agents()

```rust
pub fn external_agents(&self) -> impl ?
```

Defined in: [`packages/project/src/agent_server_store.rs:610`](../../../../packages/project/src/agent_server_store.rs#L610)

#### Returns

`impl ?`

## Trait Implementations

- `impl Borrow for AgentServerStore`
- `impl BorrowMut for AgentServerStore`
- `impl Into for AgentServerStore`
- `impl From for AgentServerStore`
- `impl TryInto for AgentServerStore`
- `impl TryFrom for AgentServerStore`
- `impl Any for AgentServerStore`
- `impl VZip for AgentServerStore`
- `impl CastableFrom for AgentServerStore`
- `impl CastableFrom for AgentServerStore`
- `impl Read for AgentServerStore`
- `impl IntoEither for AgentServerStore`
- `impl ErasedDestructor for AgentServerStore`
- `impl Same for AgentServerStore`
- `impl Pointable for AgentServerStore`
- `impl Instrument for AgentServerStore`
- `impl WithSubscriber for AgentServerStore`
- `impl FromAngle for AgentServerStore`
- `impl IntoAngle for AgentServerStore`
- `impl IntoCam16Unclamped for AgentServerStore`
- `impl Cam16IntoUnclamped for AgentServerStore`
- `impl ArraysFrom for AgentServerStore`
- `impl ArraysInto for AgentServerStore`
- `impl ComponentsFrom for AgentServerStore`
- `impl TryComponentsInto for AgentServerStore`
- `impl UintsFrom for AgentServerStore`
- `impl UintsInto for AgentServerStore`
- `impl AdaptIntoUnclamped for AgentServerStore`
- `impl AdaptInto for AgentServerStore`
- `impl IntoColor for AgentServerStore`
- `impl IntoColorUnclamped for AgentServerStore`
- `impl TryIntoColor for AgentServerStore`
- `impl FromStimulus for AgentServerStore`
- `impl IntoStimulus for AgentServerStore`
- `impl EventEmitter for AgentServerStore`

## Auto Trait Implementations

`Freeze` `Unpin` `UnsafeUnpin`

