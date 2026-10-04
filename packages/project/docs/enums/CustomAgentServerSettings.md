---
id: CustomAgentServerSettings
title: CustomAgentServerSettings
---

# Enum: CustomAgentServerSettings

Defined in: [`packages/project/src/agent_server_store.rs:1534`](../../../../packages/project/src/agent_server_store.rs#L1534)

## Definition

```rust
pub enum CustomAgentServerSettings {
    Custom{ .. },
    Registry{ .. },
}
```

## Implementations

### command()

```rust
pub fn command(&self) -> Option<&AgentServerCommand>
```

Defined in: [`packages/project/src/agent_server_store.rs:1583`](../../../../packages/project/src/agent_server_store.rs#L1583)

#### Returns

`Option<&AgentServerCommand>`


***

### default_mode()

```rust
pub fn default_mode(&self) -> Option<&str>
```

Defined in: [`packages/project/src/agent_server_store.rs:1590`](../../../../packages/project/src/agent_server_store.rs#L1590)

#### Returns

`Option<&str>`


***

### default_config_option()

```rust
pub fn default_config_option(&self, config_id: &str) -> Option<&AgentConfigOptionValue>
```

Defined in: [`packages/project/src/agent_server_store.rs:1597`](../../../../packages/project/src/agent_server_store.rs#L1597)

#### Parameters

##### config_id

`&str`

#### Returns

`Option<&AgentConfigOptionValue>`


***

### favorite_config_option_values()

```rust
pub fn favorite_config_option_values(&self, config_id: &str) -> Option<&[String]>
```

Defined in: [`packages/project/src/agent_server_store.rs:1610`](../../../../packages/project/src/agent_server_store.rs#L1610)

#### Parameters

##### config_id

`&str`

#### Returns

`Option<&[String]>`

## Trait Implementations

- `impl Borrow for CustomAgentServerSettings`
- `impl BorrowMut for CustomAgentServerSettings`
- `impl CloneToUninit for CustomAgentServerSettings`
- `impl Into for CustomAgentServerSettings`
- `impl From for CustomAgentServerSettings`
- `impl TryInto for CustomAgentServerSettings`
- `impl TryFrom for CustomAgentServerSettings`
- `impl Any for CustomAgentServerSettings`
- `impl ToOwned for CustomAgentServerSettings`
- `impl DynClone for CustomAgentServerSettings`
- `impl VZip for CustomAgentServerSettings`
- `impl CastableFrom for CustomAgentServerSettings`
- `impl CastableFrom for CustomAgentServerSettings`
- `impl Read for CustomAgentServerSettings`
- `impl IntoEither for CustomAgentServerSettings`
- `impl ErasedDestructor for CustomAgentServerSettings`
- `impl Same for CustomAgentServerSettings`
- `impl Pointable for CustomAgentServerSettings`
- `impl Instrument for CustomAgentServerSettings`
- `impl WithSubscriber for CustomAgentServerSettings`
- `impl FromAngle for CustomAgentServerSettings`
- `impl IntoAngle for CustomAgentServerSettings`
- `impl IntoCam16Unclamped for CustomAgentServerSettings`
- `impl Cam16IntoUnclamped for CustomAgentServerSettings`
- `impl ArraysFrom for CustomAgentServerSettings`
- `impl ArraysInto for CustomAgentServerSettings`
- `impl ComponentsFrom for CustomAgentServerSettings`
- `impl TryComponentsInto for CustomAgentServerSettings`
- `impl UintsFrom for CustomAgentServerSettings`
- `impl UintsInto for CustomAgentServerSettings`
- `impl AdaptIntoUnclamped for CustomAgentServerSettings`
- `impl AdaptInto for CustomAgentServerSettings`
- `impl IntoColor for CustomAgentServerSettings`
- `impl IntoColorUnclamped for CustomAgentServerSettings`
- `impl TryIntoColor for CustomAgentServerSettings`
- `impl FromStimulus for CustomAgentServerSettings`
- `impl IntoStimulus for CustomAgentServerSettings`
- `impl Clone for CustomAgentServerSettings`
- `impl JsonSchema for CustomAgentServerSettings`
- `impl Debug for CustomAgentServerSettings`
- `impl StructuralPartialEq for CustomAgentServerSettings`
- `impl PartialEq for CustomAgentServerSettings`
- `impl From for CustomAgentServerSettings`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

