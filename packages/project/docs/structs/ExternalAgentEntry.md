---
id: ExternalAgentEntry
title: ExternalAgentEntry
---

# Struct: ExternalAgentEntry

Defined in: [`packages/project/src/agent_server_store.rs:164`](../../../../packages/project/src/agent_server_store.rs#L164)

## Definition

```rust
pub struct ExternalAgentEntry
{
    pub source: ExternalAgentSource,
}
```

_（存在非公开字段）_

## Implementations

### new()

```rust
pub fn new(server: Box<dyn ExternalAgentServer>, source: ExternalAgentSource, icon: Option<SharedString>, display_name: Option<SharedString>) -> Self
```

Defined in: [`packages/project/src/agent_server_store.rs:172`](../../../../packages/project/src/agent_server_store.rs#L172)

#### Parameters

##### server

`Box<dyn ExternalAgentServer>`

##### source

[`ExternalAgentSource`](../enums/ExternalAgentSource.md)

##### icon

`Option<SharedString>`

##### display_name

`Option<SharedString>`

#### Returns

`Self`

## Trait Implementations

- `impl Borrow for ExternalAgentEntry`
- `impl BorrowMut for ExternalAgentEntry`
- `impl Into for ExternalAgentEntry`
- `impl From for ExternalAgentEntry`
- `impl TryInto for ExternalAgentEntry`
- `impl TryFrom for ExternalAgentEntry`
- `impl Any for ExternalAgentEntry`
- `impl VZip for ExternalAgentEntry`
- `impl CastableFrom for ExternalAgentEntry`
- `impl CastableFrom for ExternalAgentEntry`
- `impl Read for ExternalAgentEntry`
- `impl IntoEither for ExternalAgentEntry`
- `impl ErasedDestructor for ExternalAgentEntry`
- `impl Same for ExternalAgentEntry`
- `impl Pointable for ExternalAgentEntry`
- `impl Instrument for ExternalAgentEntry`
- `impl WithSubscriber for ExternalAgentEntry`
- `impl FromAngle for ExternalAgentEntry`
- `impl IntoAngle for ExternalAgentEntry`
- `impl IntoCam16Unclamped for ExternalAgentEntry`
- `impl Cam16IntoUnclamped for ExternalAgentEntry`
- `impl ArraysFrom for ExternalAgentEntry`
- `impl ArraysInto for ExternalAgentEntry`
- `impl ComponentsFrom for ExternalAgentEntry`
- `impl TryComponentsInto for ExternalAgentEntry`
- `impl UintsFrom for ExternalAgentEntry`
- `impl UintsInto for ExternalAgentEntry`
- `impl AdaptIntoUnclamped for ExternalAgentEntry`
- `impl AdaptInto for ExternalAgentEntry`
- `impl IntoColor for ExternalAgentEntry`
- `impl IntoColorUnclamped for ExternalAgentEntry`
- `impl TryIntoColor for ExternalAgentEntry`
- `impl FromStimulus for ExternalAgentEntry`
- `impl IntoStimulus for ExternalAgentEntry`

## Auto Trait Implementations

`Freeze` `Unpin` `UnsafeUnpin`

