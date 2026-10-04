---
id: RegistryTargetConfig
title: RegistryTargetConfig
---

# Struct: RegistryTargetConfig

Defined in: [`packages/project/src/agent_registry_store.rs:109`](../../../../packages/project/src/agent_registry_store.rs#L109)

## Definition

```rust
pub struct RegistryTargetConfig
{
    pub archive: String,
    pub cmd: String,
    pub args: Vec<String>,
    pub sha256: Option<String>,
    pub env: HashMap<String, String>,
}
```

## Trait Implementations

- `impl Borrow for RegistryTargetConfig`
- `impl BorrowMut for RegistryTargetConfig`
- `impl CloneToUninit for RegistryTargetConfig`
- `impl Into for RegistryTargetConfig`
- `impl From for RegistryTargetConfig`
- `impl TryInto for RegistryTargetConfig`
- `impl TryFrom for RegistryTargetConfig`
- `impl Any for RegistryTargetConfig`
- `impl ToOwned for RegistryTargetConfig`
- `impl DynClone for RegistryTargetConfig`
- `impl VZip for RegistryTargetConfig`
- `impl CastableFrom for RegistryTargetConfig`
- `impl CastableFrom for RegistryTargetConfig`
- `impl Read for RegistryTargetConfig`
- `impl IntoEither for RegistryTargetConfig`
- `impl ErasedDestructor for RegistryTargetConfig`
- `impl Same for RegistryTargetConfig`
- `impl Pointable for RegistryTargetConfig`
- `impl Instrument for RegistryTargetConfig`
- `impl WithSubscriber for RegistryTargetConfig`
- `impl FromAngle for RegistryTargetConfig`
- `impl IntoAngle for RegistryTargetConfig`
- `impl IntoCam16Unclamped for RegistryTargetConfig`
- `impl Cam16IntoUnclamped for RegistryTargetConfig`
- `impl ArraysFrom for RegistryTargetConfig`
- `impl ArraysInto for RegistryTargetConfig`
- `impl ComponentsFrom for RegistryTargetConfig`
- `impl TryComponentsInto for RegistryTargetConfig`
- `impl UintsFrom for RegistryTargetConfig`
- `impl UintsInto for RegistryTargetConfig`
- `impl AdaptIntoUnclamped for RegistryTargetConfig`
- `impl AdaptInto for RegistryTargetConfig`
- `impl IntoColor for RegistryTargetConfig`
- `impl IntoColorUnclamped for RegistryTargetConfig`
- `impl TryIntoColor for RegistryTargetConfig`
- `impl FromStimulus for RegistryTargetConfig`
- `impl IntoStimulus for RegistryTargetConfig`
- `impl Clone for RegistryTargetConfig`
- `impl Debug for RegistryTargetConfig`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

