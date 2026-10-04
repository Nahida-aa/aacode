---
id: AgentServerCommand
title: AgentServerCommand
---

# Struct: AgentServerCommand

Defined in: [`packages/project/src/agent_server_store.rs:36`](../../../../packages/project/src/agent_server_store.rs#L36)

## Definition

```rust
pub struct AgentServerCommand {
    pub path: PathBuf,
    pub args: Vec<String>,
    pub env: Option<HashMap<String, String>>,
}
```

## Trait Implementations

- `impl Borrow for AgentServerCommand`
- `impl BorrowMut for AgentServerCommand`
- `impl CloneToUninit for AgentServerCommand`
- `impl Into for AgentServerCommand`
- `impl From for AgentServerCommand`
- `impl TryInto for AgentServerCommand`
- `impl TryFrom for AgentServerCommand`
- `impl Any for AgentServerCommand`
- `impl ToOwned for AgentServerCommand`
- `impl DeserializeOwned for AgentServerCommand`
- `impl Equivalent for AgentServerCommand`
- `impl Serialize for AgentServerCommand`
- `impl DynClone for AgentServerCommand`
- `impl VZip for AgentServerCommand`
- `impl CastableFrom for AgentServerCommand`
- `impl CastableFrom for AgentServerCommand`
- `impl Read for AgentServerCommand`
- `impl IntoEither for AgentServerCommand`
- `impl ErasedDestructor for AgentServerCommand`
- `impl Same for AgentServerCommand`
- `impl Pointable for AgentServerCommand`
- `impl Instrument for AgentServerCommand`
- `impl WithSubscriber for AgentServerCommand`
- `impl FromAngle for AgentServerCommand`
- `impl IntoAngle for AgentServerCommand`
- `impl IntoCam16Unclamped for AgentServerCommand`
- `impl Cam16IntoUnclamped for AgentServerCommand`
- `impl ArraysFrom for AgentServerCommand`
- `impl ArraysInto for AgentServerCommand`
- `impl ComponentsFrom for AgentServerCommand`
- `impl TryComponentsInto for AgentServerCommand`
- `impl UintsFrom for AgentServerCommand`
- `impl UintsInto for AgentServerCommand`
- `impl AdaptIntoUnclamped for AgentServerCommand`
- `impl AdaptInto for AgentServerCommand`
- `impl IntoColor for AgentServerCommand`
- `impl IntoColorUnclamped for AgentServerCommand`
- `impl TryIntoColor for AgentServerCommand`
- `impl FromStimulus for AgentServerCommand`
- `impl IntoStimulus for AgentServerCommand`
- `impl Equivalent for AgentServerCommand`
- `impl Deserialize for AgentServerCommand`
- `impl Serialize for AgentServerCommand`
- `impl Clone for AgentServerCommand`
- `impl StructuralPartialEq for AgentServerCommand`
- `impl PartialEq for AgentServerCommand`
- `impl Eq for AgentServerCommand`
- `impl JsonSchema for AgentServerCommand`
- `impl Debug for AgentServerCommand`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

