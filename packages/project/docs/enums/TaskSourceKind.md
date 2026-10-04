---
id: TaskSourceKind
title: TaskSourceKind
---

# Enum: TaskSourceKind

Defined in: [`packages/project/src/task_inventory.rs:147`](../../../../packages/project/src/task_inventory.rs#L147)

Kind of a source the tasks are fetched from, used to display more source information in the UI.

## Definition

```rust
pub enum TaskSourceKind
{
    // bash-like commands spawned by users, not associated with any path
    UserInput,
    // Tasks from the worktree's .zed/task.json
    Worktree{ .. },
    // ~/.config/zed/task.json - like global files with task definitions, applicable to any path
    AbsPath{ .. },
    // Languages-specific tasks coming from extensions.
    Language{ .. },
    // Language-specific tasks coming from LSP servers.
    Lsp{ .. },
}
```

## Variants

### UserInput

Defined in: [`packages/project/src/task_inventory.rs:149`](../../../../packages/project/src/task_inventory.rs#L149)

bash-like commands spawned by users, not associated with any path


***

### Worktree

Defined in: [`packages/project/src/task_inventory.rs:151`](../../../../packages/project/src/task_inventory.rs#L151)

Tasks from the worktree's .zed/task.json


***

### AbsPath

Defined in: [`packages/project/src/task_inventory.rs:157`](../../../../packages/project/src/task_inventory.rs#L157)

~/.config/zed/task.json - like global files with task definitions, applicable to any path


***

### Language

Defined in: [`packages/project/src/task_inventory.rs:162`](../../../../packages/project/src/task_inventory.rs#L162)

Languages-specific tasks coming from extensions.


***

### Lsp

Defined in: [`packages/project/src/task_inventory.rs:164`](../../../../packages/project/src/task_inventory.rs#L164)

Language-specific tasks coming from LSP servers.

## Implementations

### to_id_base()

```rust
pub fn to_id_base(&self) -> String
```

Defined in: [`packages/project/src/task_inventory.rs:276`](../../../../packages/project/src/task_inventory.rs#L276)

#### Returns

`String`

## Trait Implementations

- `impl Borrow for TaskSourceKind`
- `impl BorrowMut for TaskSourceKind`
- `impl CloneToUninit for TaskSourceKind`
- `impl Into for TaskSourceKind`
- `impl From for TaskSourceKind`
- `impl TryInto for TaskSourceKind`
- `impl TryFrom for TaskSourceKind`
- `impl Any for TaskSourceKind`
- `impl ToOwned for TaskSourceKind`
- `impl Equivalent for TaskSourceKind`
- `impl Comparable for TaskSourceKind`
- `impl DynClone for TaskSourceKind`
- `impl VZip for TaskSourceKind`
- `impl CastableFrom for TaskSourceKind`
- `impl CastableFrom for TaskSourceKind`
- `impl Read for TaskSourceKind`
- `impl IntoEither for TaskSourceKind`
- `impl ErasedDestructor for TaskSourceKind`
- `impl Same for TaskSourceKind`
- `impl Pointable for TaskSourceKind`
- `impl MapSeekTarget for TaskSourceKind`
- `impl Instrument for TaskSourceKind`
- `impl WithSubscriber for TaskSourceKind`
- `impl FromAngle for TaskSourceKind`
- `impl IntoAngle for TaskSourceKind`
- `impl IntoCam16Unclamped for TaskSourceKind`
- `impl Cam16IntoUnclamped for TaskSourceKind`
- `impl ArraysFrom for TaskSourceKind`
- `impl ArraysInto for TaskSourceKind`
- `impl ComponentsFrom for TaskSourceKind`
- `impl TryComponentsInto for TaskSourceKind`
- `impl UintsFrom for TaskSourceKind`
- `impl UintsInto for TaskSourceKind`
- `impl AdaptIntoUnclamped for TaskSourceKind`
- `impl AdaptInto for TaskSourceKind`
- `impl IntoColor for TaskSourceKind`
- `impl IntoColorUnclamped for TaskSourceKind`
- `impl TryIntoColor for TaskSourceKind`
- `impl FromStimulus for TaskSourceKind`
- `impl IntoStimulus for TaskSourceKind`
- `impl Equivalent for TaskSourceKind`
- `impl Debug for TaskSourceKind`
- `impl Clone for TaskSourceKind`
- `impl StructuralPartialEq for TaskSourceKind`
- `impl PartialEq for TaskSourceKind`
- `impl Eq for TaskSourceKind`
- `impl Hash for TaskSourceKind`
- `impl PartialOrd for TaskSourceKind`
- `impl Ord for TaskSourceKind`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

