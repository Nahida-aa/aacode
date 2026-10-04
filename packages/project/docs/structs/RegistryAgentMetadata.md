---
id: RegistryAgentMetadata
title: RegistryAgentMetadata
---

# Struct: RegistryAgentMetadata

Defined in: [`packages/project/src/agent_registry_store.rs:28`](../../../../packages/project/src/agent_registry_store.rs#L28)

## Definition

```rust
pub struct RegistryAgentMetadata {
    pub id: AgentId,
    pub name: SharedString,
    pub description: SharedString,
    pub version: SharedString,
    pub repository: Option<SharedString>,
    pub website: Option<SharedString>,
    pub license_url: Option<SharedString>,
    pub icon_path: Option<SharedString>,
}
```

## Trait Implementations

- `impl Borrow for RegistryAgentMetadata`
- `impl BorrowMut for RegistryAgentMetadata`
- `impl CloneToUninit for RegistryAgentMetadata`
- `impl Into for RegistryAgentMetadata`
- `impl From for RegistryAgentMetadata`
- `impl TryInto for RegistryAgentMetadata`
- `impl TryFrom for RegistryAgentMetadata`
- `impl Any for RegistryAgentMetadata`
- `impl ToOwned for RegistryAgentMetadata`
- `impl DynClone for RegistryAgentMetadata`
- `impl VZip for RegistryAgentMetadata`
- `impl CastableFrom for RegistryAgentMetadata`
- `impl CastableFrom for RegistryAgentMetadata`
- `impl Read for RegistryAgentMetadata`
- `impl IntoEither for RegistryAgentMetadata`
- `impl ErasedDestructor for RegistryAgentMetadata`
- `impl Same for RegistryAgentMetadata`
- `impl Pointable for RegistryAgentMetadata`
- `impl Instrument for RegistryAgentMetadata`
- `impl WithSubscriber for RegistryAgentMetadata`
- `impl FromAngle for RegistryAgentMetadata`
- `impl IntoAngle for RegistryAgentMetadata`
- `impl IntoCam16Unclamped for RegistryAgentMetadata`
- `impl Cam16IntoUnclamped for RegistryAgentMetadata`
- `impl ArraysFrom for RegistryAgentMetadata`
- `impl ArraysInto for RegistryAgentMetadata`
- `impl ComponentsFrom for RegistryAgentMetadata`
- `impl TryComponentsInto for RegistryAgentMetadata`
- `impl UintsFrom for RegistryAgentMetadata`
- `impl UintsInto for RegistryAgentMetadata`
- `impl AdaptIntoUnclamped for RegistryAgentMetadata`
- `impl AdaptInto for RegistryAgentMetadata`
- `impl IntoColor for RegistryAgentMetadata`
- `impl IntoColorUnclamped for RegistryAgentMetadata`
- `impl TryIntoColor for RegistryAgentMetadata`
- `impl FromStimulus for RegistryAgentMetadata`
- `impl IntoStimulus for RegistryAgentMetadata`
- `impl Clone for RegistryAgentMetadata`
- `impl Debug for RegistryAgentMetadata`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

