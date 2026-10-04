---
id: RegistryBinaryAgent
title: RegistryBinaryAgent
---

# Struct: RegistryBinaryAgent

Defined in: [`packages/project/src/agent_registry_store.rs:40`](../../../../packages/project/src/agent_registry_store.rs#L40)

## Definition

```rust
pub struct RegistryBinaryAgent {
    pub metadata: RegistryAgentMetadata,
    pub targets: HashMap<String, RegistryTargetConfig>,
    pub supports_current_platform: bool,
}
```

## Trait Implementations

- `impl Borrow for RegistryBinaryAgent`
- `impl BorrowMut for RegistryBinaryAgent`
- `impl CloneToUninit for RegistryBinaryAgent`
- `impl Into for RegistryBinaryAgent`
- `impl From for RegistryBinaryAgent`
- `impl TryInto for RegistryBinaryAgent`
- `impl TryFrom for RegistryBinaryAgent`
- `impl Any for RegistryBinaryAgent`
- `impl ToOwned for RegistryBinaryAgent`
- `impl DynClone for RegistryBinaryAgent`
- `impl VZip for RegistryBinaryAgent`
- `impl CastableFrom for RegistryBinaryAgent`
- `impl CastableFrom for RegistryBinaryAgent`
- `impl Read for RegistryBinaryAgent`
- `impl IntoEither for RegistryBinaryAgent`
- `impl ErasedDestructor for RegistryBinaryAgent`
- `impl Same for RegistryBinaryAgent`
- `impl Pointable for RegistryBinaryAgent`
- `impl Instrument for RegistryBinaryAgent`
- `impl WithSubscriber for RegistryBinaryAgent`
- `impl FromAngle for RegistryBinaryAgent`
- `impl IntoAngle for RegistryBinaryAgent`
- `impl IntoCam16Unclamped for RegistryBinaryAgent`
- `impl Cam16IntoUnclamped for RegistryBinaryAgent`
- `impl ArraysFrom for RegistryBinaryAgent`
- `impl ArraysInto for RegistryBinaryAgent`
- `impl ComponentsFrom for RegistryBinaryAgent`
- `impl TryComponentsInto for RegistryBinaryAgent`
- `impl UintsFrom for RegistryBinaryAgent`
- `impl UintsInto for RegistryBinaryAgent`
- `impl AdaptIntoUnclamped for RegistryBinaryAgent`
- `impl AdaptInto for RegistryBinaryAgent`
- `impl IntoColor for RegistryBinaryAgent`
- `impl IntoColorUnclamped for RegistryBinaryAgent`
- `impl TryIntoColor for RegistryBinaryAgent`
- `impl FromStimulus for RegistryBinaryAgent`
- `impl IntoStimulus for RegistryBinaryAgent`
- `impl Clone for RegistryBinaryAgent`
- `impl Debug for RegistryBinaryAgent`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

