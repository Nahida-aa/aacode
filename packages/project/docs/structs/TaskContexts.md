---
id: TaskContexts
title: TaskContexts
---

# Struct: TaskContexts

Defined in: [`packages/project/src/task_inventory.rs:173`](../../../../packages/project/src/task_inventory.rs#L173)

A collection of task contexts, derived from the current state of the workspace.
Only contains worktrees that are visible and with their root being a directory.

## Definition

```rust
pub struct TaskContexts {
    // A context, related to the currently opened item. Item can be opened from an invisible worktree, or any other, not necessarily active worktree.
    pub active_item_context: Option<(Option<WorktreeId>, Option<Location>, TaskContext)>,
    // A worktree that corresponds to the active item, or the only worktree in the workspace.
    pub active_worktree_context: Option<(WorktreeId, TaskContext)>,
    // If there are multiple worktrees in the workspace, all non-active ones are included here.
    pub other_worktree_contexts: Vec<(WorktreeId, TaskContext)>,
    pub lsp_task_sources: HashMap<LanguageServerName, Vec<BufferId>>,
    pub latest_selection: Option<Anchor>,
}
```

## Fields

### active_item_context

Defined in: [`packages/project/src/task_inventory.rs:176`](../../../../packages/project/src/task_inventory.rs#L176)

A context, related to the currently opened item.
Item can be opened from an invisible worktree, or any other, not necessarily active worktree.


***

### active_worktree_context

Defined in: [`packages/project/src/task_inventory.rs:178`](../../../../packages/project/src/task_inventory.rs#L178)

A worktree that corresponds to the active item, or the only worktree in the workspace.


***

### other_worktree_contexts

Defined in: [`packages/project/src/task_inventory.rs:180`](../../../../packages/project/src/task_inventory.rs#L180)

If there are multiple worktrees in the workspace, all non-active ones are included here.

## Implementations

### active_context()

```rust
pub fn active_context(&self) -> Option<&TaskContext>
```

Defined in: [`packages/project/src/task_inventory.rs:186`](../../../../packages/project/src/task_inventory.rs#L186)

#### Returns

`Option<&TaskContext>`


***

### location()

```rust
pub fn location(&self) -> Option<&Location>
```

Defined in: [`packages/project/src/task_inventory.rs:197`](../../../../packages/project/src/task_inventory.rs#L197)

#### Returns

`Option<&Location>`


***

### file()

```rust
pub fn file(&self, cx: &App) -> Option<Arc<dyn File>>
```

Defined in: [`packages/project/src/task_inventory.rs:203`](../../../../packages/project/src/task_inventory.rs#L203)

#### Parameters

##### cx

`&App`

#### Returns

`Option<Arc<dyn File>>`


***

### worktree()

```rust
pub fn worktree(&self) -> Option<WorktreeId>
```

Defined in: [`packages/project/src/task_inventory.rs:210`](../../../../packages/project/src/task_inventory.rs#L210)

#### Returns

`Option<WorktreeId>`


***

### task_context_for_worktree_id()

```rust
pub fn task_context_for_worktree_id(&self, worktree_id: WorktreeId) -> Option<&TaskContext>
```

Defined in: [`packages/project/src/task_inventory.rs:222`](../../../../packages/project/src/task_inventory.rs#L222)

#### Parameters

##### worktree_id

`WorktreeId`

#### Returns

`Option<&TaskContext>`

## Trait Implementations

- `impl Borrow for TaskContexts`
- `impl BorrowMut for TaskContexts`
- `impl Into for TaskContexts`
- `impl From for TaskContexts`
- `impl TryInto for TaskContexts`
- `impl TryFrom for TaskContexts`
- `impl Any for TaskContexts`
- `impl VZip for TaskContexts`
- `impl CastableFrom for TaskContexts`
- `impl CastableFrom for TaskContexts`
- `impl Read for TaskContexts`
- `impl IntoEither for TaskContexts`
- `impl ErasedDestructor for TaskContexts`
- `impl Same for TaskContexts`
- `impl ReadPrimitive for TaskContexts`
- `impl Pointable for TaskContexts`
- `impl Instrument for TaskContexts`
- `impl WithSubscriber for TaskContexts`
- `impl FromAngle for TaskContexts`
- `impl IntoAngle for TaskContexts`
- `impl IntoCam16Unclamped for TaskContexts`
- `impl Cam16IntoUnclamped for TaskContexts`
- `impl ArraysFrom for TaskContexts`
- `impl ArraysInto for TaskContexts`
- `impl ComponentsFrom for TaskContexts`
- `impl TryComponentsInto for TaskContexts`
- `impl UintsFrom for TaskContexts`
- `impl UintsInto for TaskContexts`
- `impl AdaptIntoUnclamped for TaskContexts`
- `impl AdaptInto for TaskContexts`
- `impl IntoColor for TaskContexts`
- `impl IntoColorUnclamped for TaskContexts`
- `impl TryIntoColor for TaskContexts`
- `impl FromStimulus for TaskContexts`
- `impl IntoStimulus for TaskContexts`
- `impl Debug for TaskContexts`
- `impl Default for TaskContexts`

## Auto Trait Implementations

`Freeze` `Send` `Sync` `Unpin` `UnsafeUnpin`

