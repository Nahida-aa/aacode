---
id: AgentRegistryStore
title: AgentRegistryStore
---

# Struct: AgentRegistryStore

Defined in: [`packages/project/src/agent_registry_store.rs:121`](../../../../packages/project/src/agent_registry_store.rs#L121)

## Definition

```rust
pub struct AgentRegistryStore
```

_（存在非公开字段）_

## Implementations

### init_global()

```rust
pub fn init_global(cx: &App, fs: Arc<dyn Fs>, http_client: Arc<dyn HttpClient>) -> Entity<Self>
```

Defined in: [`packages/project/src/agent_registry_store.rs:138`](../../../../packages/project/src/agent_registry_store.rs#L138)

Initialize the global AgentRegistryStore.

This loads the cached registry from disk. If the cache is empty but there
are registry agents configured in settings, it will trigger a network fetch.
Otherwise, call `refresh()` explicitly when you need fresh data
(e.g., when opening the Agent Registry page).

#### Parameters

##### cx

`&App`

##### fs

`Arc<dyn Fs>`

##### http_client

`Arc<dyn HttpClient>`

#### Returns

`Entity<Self>`


***

### global()

```rust
pub fn global(cx: &App) -> Entity<Self>
```

Defined in: [`packages/project/src/agent_registry_store.rs:159`](../../../../packages/project/src/agent_registry_store.rs#L159)

#### Parameters

##### cx

`&App`

#### Returns

`Entity<Self>`


***

### try_global()

```rust
pub fn try_global(cx: &App) -> Option<Entity<Self>>
```

Defined in: [`packages/project/src/agent_registry_store.rs:163`](../../../../packages/project/src/agent_registry_store.rs#L163)

#### Parameters

##### cx

`&App`

#### Returns

`Option<Entity<Self>>`


***

### agents()

```rust
pub fn agents(&self) -> &[RegistryAgent]
```

Defined in: [`packages/project/src/agent_registry_store.rs:190`](../../../../packages/project/src/agent_registry_store.rs#L190)

#### Returns

`&[RegistryAgent]`


***

### agent()

```rust
pub fn agent(&self, id: &AgentId) -> Option<&RegistryAgent>
```

Defined in: [`packages/project/src/agent_registry_store.rs:194`](../../../../packages/project/src/agent_registry_store.rs#L194)

#### Parameters

##### id

`&AgentId`

#### Returns

`Option<&RegistryAgent>`


***

### is_fetching()

```rust
pub fn is_fetching(&self) -> bool
```

Defined in: [`packages/project/src/agent_registry_store.rs:198`](../../../../packages/project/src/agent_registry_store.rs#L198)

#### Returns

`bool`


***

### fetch_error()

```rust
pub fn fetch_error(&self) -> Option<SharedString>
```

Defined in: [`packages/project/src/agent_registry_store.rs:202`](../../../../packages/project/src/agent_registry_store.rs#L202)

#### Returns

`Option<SharedString>`


***

### refresh()

```rust
pub fn refresh(&self, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/agent_registry_store.rs:209`](../../../../packages/project/src/agent_registry_store.rs#L209)

Refresh the registry from the network.

This will fetch the latest registry data and update the cache.

#### Parameters

##### cx

`&Context<'_, Self>`


***

### refresh_if_stale()

```rust
pub fn refresh_if_stale(&self, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/agent_registry_store.rs:269`](../../../../packages/project/src/agent_registry_store.rs#L269)

Refresh the registry if it hasn't been refreshed recently.

This is useful to call when using a registry-based agent to check for
updates without making too many network requests. The refresh is
throttled to at most once per hour.

#### Parameters

##### cx

`&Context<'_, Self>`

## Trait Implementations

- `impl Borrow for AgentRegistryStore`
- `impl BorrowMut for AgentRegistryStore`
- `impl Into for AgentRegistryStore`
- `impl From for AgentRegistryStore`
- `impl TryInto for AgentRegistryStore`
- `impl TryFrom for AgentRegistryStore`
- `impl Any for AgentRegistryStore`
- `impl VZip for AgentRegistryStore`
- `impl CastableFrom for AgentRegistryStore`
- `impl CastableFrom for AgentRegistryStore`
- `impl Read for AgentRegistryStore`
- `impl IntoEither for AgentRegistryStore`
- `impl ErasedDestructor for AgentRegistryStore`
- `impl Same for AgentRegistryStore`
- `impl Pointable for AgentRegistryStore`
- `impl Instrument for AgentRegistryStore`
- `impl WithSubscriber for AgentRegistryStore`
- `impl FromAngle for AgentRegistryStore`
- `impl IntoAngle for AgentRegistryStore`
- `impl IntoCam16Unclamped for AgentRegistryStore`
- `impl Cam16IntoUnclamped for AgentRegistryStore`
- `impl ArraysFrom for AgentRegistryStore`
- `impl ArraysInto for AgentRegistryStore`
- `impl ComponentsFrom for AgentRegistryStore`
- `impl TryComponentsInto for AgentRegistryStore`
- `impl UintsFrom for AgentRegistryStore`
- `impl UintsInto for AgentRegistryStore`
- `impl AdaptIntoUnclamped for AgentRegistryStore`
- `impl AdaptInto for AgentRegistryStore`
- `impl IntoColor for AgentRegistryStore`
- `impl IntoColorUnclamped for AgentRegistryStore`
- `impl TryIntoColor for AgentRegistryStore`
- `impl FromStimulus for AgentRegistryStore`
- `impl IntoStimulus for AgentRegistryStore`

## Auto Trait Implementations

`Freeze` `Send` `Sync` `Unpin` `UnsafeUnpin`

