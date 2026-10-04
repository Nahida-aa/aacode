---
id: DebugScenarioContext
title: DebugScenarioContext
---

# Struct: DebugScenarioContext

Defined in: [`packages/project/src/task_inventory.rs:36`](../../../../packages/project/src/task_inventory.rs#L36)

## Definition

```rust
pub struct DebugScenarioContext {
    pub task_context: SharedTaskContext,
    pub worktree_id: Option<WorktreeId>,
    pub active_buffer: Option<WeakEntity<Buffer>>,
}
```

## Trait Implementations

- `impl Borrow for DebugScenarioContext`
- `impl BorrowMut for DebugScenarioContext`
- `impl CloneToUninit for DebugScenarioContext`
- `impl Into for DebugScenarioContext`
- `impl From for DebugScenarioContext`
- `impl TryInto for DebugScenarioContext`
- `impl TryFrom for DebugScenarioContext`
- `impl Any for DebugScenarioContext`
- `impl ToOwned for DebugScenarioContext`
- `impl DynClone for DebugScenarioContext`
- `impl VZip for DebugScenarioContext`
- `impl CastableFrom for DebugScenarioContext`
- `impl CastableFrom for DebugScenarioContext`
- `impl Read for DebugScenarioContext`
- `impl IntoEither for DebugScenarioContext`
- `impl ErasedDestructor for DebugScenarioContext`
- `impl Same for DebugScenarioContext`
- `impl ReadPrimitive for DebugScenarioContext`
- `impl Pointable for DebugScenarioContext`
- `impl Instrument for DebugScenarioContext`
- `impl WithSubscriber for DebugScenarioContext`
- `impl FromAngle for DebugScenarioContext`
- `impl IntoAngle for DebugScenarioContext`
- `impl IntoCam16Unclamped for DebugScenarioContext`
- `impl Cam16IntoUnclamped for DebugScenarioContext`
- `impl ArraysFrom for DebugScenarioContext`
- `impl ArraysInto for DebugScenarioContext`
- `impl ComponentsFrom for DebugScenarioContext`
- `impl TryComponentsInto for DebugScenarioContext`
- `impl UintsFrom for DebugScenarioContext`
- `impl UintsInto for DebugScenarioContext`
- `impl AdaptIntoUnclamped for DebugScenarioContext`
- `impl AdaptInto for DebugScenarioContext`
- `impl IntoColor for DebugScenarioContext`
- `impl IntoColorUnclamped for DebugScenarioContext`
- `impl TryIntoColor for DebugScenarioContext`
- `impl FromStimulus for DebugScenarioContext`
- `impl IntoStimulus for DebugScenarioContext`
- `impl Clone for DebugScenarioContext`
- `impl Debug for DebugScenarioContext`
- `impl Default for DebugScenarioContext`

## Auto Trait Implementations

`Freeze` `Send` `Sync` `Unpin` `UnsafeUnpin`

