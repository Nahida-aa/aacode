---
id: ContextProviderWithTasks
title: ContextProviderWithTasks
---

# Struct: ContextProviderWithTasks

Defined in: [`packages/project/src/task_inventory.rs:1133`](../../../../packages/project/src/task_inventory.rs#L1133)

A ContextProvider that doesn't provide any task variables on it's own, though it has some associated tasks.

## Definition

```rust
pub struct ContextProviderWithTasks
```

_（存在非公开字段）_

## Implementations

### new()

```rust
pub fn new(definitions: TaskTemplates) -> Self
```

Defined in: [`packages/project/src/task_inventory.rs:1138`](../../../../packages/project/src/task_inventory.rs#L1138)

#### Parameters

##### definitions

`TaskTemplates`

#### Returns

`Self`

## Trait Implementations

- `impl Borrow for ContextProviderWithTasks`
- `impl BorrowMut for ContextProviderWithTasks`
- `impl Into for ContextProviderWithTasks`
- `impl From for ContextProviderWithTasks`
- `impl TryInto for ContextProviderWithTasks`
- `impl TryFrom for ContextProviderWithTasks`
- `impl Any for ContextProviderWithTasks`
- `impl VZip for ContextProviderWithTasks`
- `impl CastableFrom for ContextProviderWithTasks`
- `impl CastableFrom for ContextProviderWithTasks`
- `impl Read for ContextProviderWithTasks`
- `impl IntoEither for ContextProviderWithTasks`
- `impl ErasedDestructor for ContextProviderWithTasks`
- `impl Same for ContextProviderWithTasks`
- `impl Pointable for ContextProviderWithTasks`
- `impl Instrument for ContextProviderWithTasks`
- `impl WithSubscriber for ContextProviderWithTasks`
- `impl FromAngle for ContextProviderWithTasks`
- `impl IntoAngle for ContextProviderWithTasks`
- `impl IntoCam16Unclamped for ContextProviderWithTasks`
- `impl Cam16IntoUnclamped for ContextProviderWithTasks`
- `impl ArraysFrom for ContextProviderWithTasks`
- `impl ArraysInto for ContextProviderWithTasks`
- `impl ComponentsFrom for ContextProviderWithTasks`
- `impl TryComponentsInto for ContextProviderWithTasks`
- `impl UintsFrom for ContextProviderWithTasks`
- `impl UintsInto for ContextProviderWithTasks`
- `impl AdaptIntoUnclamped for ContextProviderWithTasks`
- `impl AdaptInto for ContextProviderWithTasks`
- `impl IntoColor for ContextProviderWithTasks`
- `impl IntoColorUnclamped for ContextProviderWithTasks`
- `impl TryIntoColor for ContextProviderWithTasks`
- `impl FromStimulus for ContextProviderWithTasks`
- `impl IntoStimulus for ContextProviderWithTasks`
- `impl ContextProvider for ContextProviderWithTasks`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

