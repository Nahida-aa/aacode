---
id: RegistryNpxAgent
title: RegistryNpxAgent
---

# Struct: RegistryNpxAgent

Defined in: [`packages/project/src/agent_registry_store.rs:47`](../../../../packages/project/src/agent_registry_store.rs#L47)

## Definition

```rust
pub struct RegistryNpxAgent {
    pub metadata: RegistryAgentMetadata,
    pub package: SharedString,
    pub args: Vec<String>,
    pub env: HashMap<String, String>,
}
```

## Trait Implementations

- `impl Borrow for RegistryNpxAgent`
- `impl BorrowMut for RegistryNpxAgent`
- `impl CloneToUninit for RegistryNpxAgent`
- `impl Into for RegistryNpxAgent`
- `impl From for RegistryNpxAgent`
- `impl TryInto for RegistryNpxAgent`
- `impl TryFrom for RegistryNpxAgent`
- `impl Any for RegistryNpxAgent`
- `impl ToOwned for RegistryNpxAgent`
- `impl DynClone for RegistryNpxAgent`
- `impl VZip for RegistryNpxAgent`
- `impl CastableFrom for RegistryNpxAgent`
- `impl CastableFrom for RegistryNpxAgent`
- `impl Read for RegistryNpxAgent`
- `impl IntoEither for RegistryNpxAgent`
- `impl ErasedDestructor for RegistryNpxAgent`
- `impl Same for RegistryNpxAgent`
- `impl Pointable for RegistryNpxAgent`
- `impl Instrument for RegistryNpxAgent`
- `impl WithSubscriber for RegistryNpxAgent`
- `impl FromAngle for RegistryNpxAgent`
- `impl IntoAngle for RegistryNpxAgent`
- `impl IntoCam16Unclamped for RegistryNpxAgent`
- `impl Cam16IntoUnclamped for RegistryNpxAgent`
- `impl ArraysFrom for RegistryNpxAgent`
- `impl ArraysInto for RegistryNpxAgent`
- `impl ComponentsFrom for RegistryNpxAgent`
- `impl TryComponentsInto for RegistryNpxAgent`
- `impl UintsFrom for RegistryNpxAgent`
- `impl UintsInto for RegistryNpxAgent`
- `impl AdaptIntoUnclamped for RegistryNpxAgent`
- `impl AdaptInto for RegistryNpxAgent`
- `impl IntoColor for RegistryNpxAgent`
- `impl IntoColorUnclamped for RegistryNpxAgent`
- `impl TryIntoColor for RegistryNpxAgent`
- `impl FromStimulus for RegistryNpxAgent`
- `impl IntoStimulus for RegistryNpxAgent`
- `impl Clone for RegistryNpxAgent`
- `impl Debug for RegistryNpxAgent`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

