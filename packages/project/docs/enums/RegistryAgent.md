---
id: RegistryAgent
title: RegistryAgent
---

# Enum: RegistryAgent

Defined in: [`packages/project/src/agent_registry_store.rs:55`](../../../../packages/project/src/agent_registry_store.rs#L55)

## Definition

```rust
pub enum RegistryAgent
{
    Binary(RegistryBinaryAgent),
    Npx(RegistryNpxAgent),
}
```

## Implementations

### metadata()

```rust
pub fn metadata(&self) -> &RegistryAgentMetadata
```

Defined in: [`packages/project/src/agent_registry_store.rs:61`](../../../../packages/project/src/agent_registry_store.rs#L61)

#### Returns

`&RegistryAgentMetadata`


***

### id()

```rust
pub fn id(&self) -> &AgentId
```

Defined in: [`packages/project/src/agent_registry_store.rs:68`](../../../../packages/project/src/agent_registry_store.rs#L68)

#### Returns

`&AgentId`


***

### name()

```rust
pub fn name(&self) -> &SharedString
```

Defined in: [`packages/project/src/agent_registry_store.rs:72`](../../../../packages/project/src/agent_registry_store.rs#L72)

#### Returns

`&SharedString`


***

### description()

```rust
pub fn description(&self) -> &SharedString
```

Defined in: [`packages/project/src/agent_registry_store.rs:76`](../../../../packages/project/src/agent_registry_store.rs#L76)

#### Returns

`&SharedString`


***

### version()

```rust
pub fn version(&self) -> &SharedString
```

Defined in: [`packages/project/src/agent_registry_store.rs:80`](../../../../packages/project/src/agent_registry_store.rs#L80)

#### Returns

`&SharedString`


***

### repository()

```rust
pub fn repository(&self) -> Option<&SharedString>
```

Defined in: [`packages/project/src/agent_registry_store.rs:84`](../../../../packages/project/src/agent_registry_store.rs#L84)

#### Returns

`Option<&SharedString>`


***

### website()

```rust
pub fn website(&self) -> Option<&SharedString>
```

Defined in: [`packages/project/src/agent_registry_store.rs:88`](../../../../packages/project/src/agent_registry_store.rs#L88)

#### Returns

`Option<&SharedString>`


***

### license_url()

```rust
pub fn license_url(&self) -> Option<&SharedString>
```

Defined in: [`packages/project/src/agent_registry_store.rs:92`](../../../../packages/project/src/agent_registry_store.rs#L92)

#### Returns

`Option<&SharedString>`


***

### icon_path()

```rust
pub fn icon_path(&self) -> Option<&SharedString>
```

Defined in: [`packages/project/src/agent_registry_store.rs:96`](../../../../packages/project/src/agent_registry_store.rs#L96)

#### Returns

`Option<&SharedString>`


***

### supports_current_platform()

```rust
pub fn supports_current_platform(&self) -> bool
```

Defined in: [`packages/project/src/agent_registry_store.rs:100`](../../../../packages/project/src/agent_registry_store.rs#L100)

#### Returns

`bool`

## Trait Implementations

- `impl Borrow for RegistryAgent`
- `impl BorrowMut for RegistryAgent`
- `impl CloneToUninit for RegistryAgent`
- `impl Into for RegistryAgent`
- `impl From for RegistryAgent`
- `impl TryInto for RegistryAgent`
- `impl TryFrom for RegistryAgent`
- `impl Any for RegistryAgent`
- `impl ToOwned for RegistryAgent`
- `impl DynClone for RegistryAgent`
- `impl VZip for RegistryAgent`
- `impl CastableFrom for RegistryAgent`
- `impl CastableFrom for RegistryAgent`
- `impl Read for RegistryAgent`
- `impl IntoEither for RegistryAgent`
- `impl ErasedDestructor for RegistryAgent`
- `impl Same for RegistryAgent`
- `impl Pointable for RegistryAgent`
- `impl Instrument for RegistryAgent`
- `impl WithSubscriber for RegistryAgent`
- `impl FromAngle for RegistryAgent`
- `impl IntoAngle for RegistryAgent`
- `impl IntoCam16Unclamped for RegistryAgent`
- `impl Cam16IntoUnclamped for RegistryAgent`
- `impl ArraysFrom for RegistryAgent`
- `impl ArraysInto for RegistryAgent`
- `impl ComponentsFrom for RegistryAgent`
- `impl TryComponentsInto for RegistryAgent`
- `impl UintsFrom for RegistryAgent`
- `impl UintsInto for RegistryAgent`
- `impl AdaptIntoUnclamped for RegistryAgent`
- `impl AdaptInto for RegistryAgent`
- `impl IntoColor for RegistryAgent`
- `impl IntoColorUnclamped for RegistryAgent`
- `impl TryIntoColor for RegistryAgent`
- `impl FromStimulus for RegistryAgent`
- `impl IntoStimulus for RegistryAgent`
- `impl Clone for RegistryAgent`
- `impl Debug for RegistryAgent`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

