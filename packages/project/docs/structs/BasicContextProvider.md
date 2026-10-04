---
id: BasicContextProvider
title: BasicContextProvider
---

# Struct: BasicContextProvider

Defined in: [`packages/project/src/task_inventory.rs:1004`](../../../../packages/project/src/task_inventory.rs#L1004)

A context provided that tries to provide values for all non-custom `VariableName` variants for a currently opened file.
Applied as a base for every custom `ContextProvider` unless explicitly oped out.

## Definition

```rust
pub struct BasicContextProvider
```

_（存在非公开字段）_

## Implementations

### new()

```rust
pub fn new(worktree_store: Entity<WorktreeStore>, git_store: Entity<GitStore>) -> Self
```

Defined in: [`packages/project/src/task_inventory.rs:1010`](../../../../packages/project/src/task_inventory.rs#L1010)

#### Parameters

##### worktree_store

`Entity<WorktreeStore>`

##### git_store

`Entity<GitStore>`

#### Returns

`Self`

## Trait Implementations

- `impl Borrow for BasicContextProvider`
- `impl BorrowMut for BasicContextProvider`
- `impl Into for BasicContextProvider`
- `impl From for BasicContextProvider`
- `impl TryInto for BasicContextProvider`
- `impl TryFrom for BasicContextProvider`
- `impl Any for BasicContextProvider`
- `impl VZip for BasicContextProvider`
- `impl CastableFrom for BasicContextProvider`
- `impl CastableFrom for BasicContextProvider`
- `impl Read for BasicContextProvider`
- `impl IntoEither for BasicContextProvider`
- `impl ErasedDestructor for BasicContextProvider`
- `impl Same for BasicContextProvider`
- `impl Pointable for BasicContextProvider`
- `impl Instrument for BasicContextProvider`
- `impl WithSubscriber for BasicContextProvider`
- `impl FromAngle for BasicContextProvider`
- `impl IntoAngle for BasicContextProvider`
- `impl IntoCam16Unclamped for BasicContextProvider`
- `impl Cam16IntoUnclamped for BasicContextProvider`
- `impl ArraysFrom for BasicContextProvider`
- `impl ArraysInto for BasicContextProvider`
- `impl ComponentsFrom for BasicContextProvider`
- `impl TryComponentsInto for BasicContextProvider`
- `impl UintsFrom for BasicContextProvider`
- `impl UintsInto for BasicContextProvider`
- `impl AdaptIntoUnclamped for BasicContextProvider`
- `impl AdaptInto for BasicContextProvider`
- `impl IntoColor for BasicContextProvider`
- `impl IntoColorUnclamped for BasicContextProvider`
- `impl TryIntoColor for BasicContextProvider`
- `impl FromStimulus for BasicContextProvider`
- `impl IntoStimulus for BasicContextProvider`
- `impl ContextProvider for BasicContextProvider`

## Auto Trait Implementations

`Freeze` `Send` `Sync` `Unpin` `UnsafeUnpin`

