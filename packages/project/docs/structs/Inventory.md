---
id: Inventory
title: Inventory
---

# Struct: Inventory

Defined in: [`packages/project/src/task_inventory.rs:43`](../../../../packages/project/src/task_inventory.rs#L43)

Inventory tracks available tasks for a given project.

## Definition

```rust
pub struct Inventory
```

_（存在非公开字段）_

## Implementations

### new()

```rust
pub fn new(cx: &App) -> Entity<Self>
```

Defined in: [`packages/project/src/task_inventory.rs:299`](../../../../packages/project/src/task_inventory.rs#L299)

#### Parameters

##### cx

`&App`

#### Returns

`Entity<Self>`


***

### scenario_scheduled()

```rust
pub fn scenario_scheduled(&self, scenario: DebugScenario, task_context: SharedTaskContext, worktree_id: Option<WorktreeId>, active_buffer: Option<WeakEntity<Buffer>>)
```

Defined in: [`packages/project/src/task_inventory.rs:308`](../../../../packages/project/src/task_inventory.rs#L308)

#### Parameters

##### scenario

`DebugScenario`

##### task_context

`SharedTaskContext`

##### worktree_id

`Option<WorktreeId>`

##### active_buffer

`Option<WeakEntity<Buffer>>`


***

### last_scheduled_scenario()

```rust
pub fn last_scheduled_scenario(&self) -> Option<&(DebugScenario, DebugScenarioContext)>
```

Defined in: [`packages/project/src/task_inventory.rs:330`](../../../../packages/project/src/task_inventory.rs#L330)

#### Returns

`Option<&(DebugScenario, DebugScenarioContext)>`


***

### list_debug_scenarios()

```rust
pub fn list_debug_scenarios(&self, task_contexts: &TaskContexts, lsp_tasks: Vec<(TaskSourceKind, ResolvedTask)>, current_resolved_tasks: Vec<(TaskSourceKind, ResolvedTask)>, add_current_language_tasks: bool, cx: &App) -> Task<(Vec<(DebugScenario, DebugScenarioContext)>, Vec<(TaskSourceKind, DebugScenario)>)>
```

Defined in: [`packages/project/src/task_inventory.rs:334`](../../../../packages/project/src/task_inventory.rs#L334)

#### Parameters

##### task_contexts

`&TaskContexts`

##### lsp_tasks

`Vec<(TaskSourceKind, ResolvedTask)>`

##### current_resolved_tasks

`Vec<(TaskSourceKind, ResolvedTask)>`

##### add_current_language_tasks

`bool`

##### cx

`&App`

#### Returns

`Task<(Vec<(DebugScenario, DebugScenarioContext)>, Vec<(TaskSourceKind, DebugScenario)>)>`


***

### task_template_by_label()

```rust
pub fn task_template_by_label(&self, buffer: Option<Entity<Buffer>>, worktree_id: Option<WorktreeId>, label: &str, cx: &App) -> Task<Option<TaskTemplate>>
```

Defined in: [`packages/project/src/task_inventory.rs:400`](../../../../packages/project/src/task_inventory.rs#L400)

#### Parameters

##### buffer

`Option<Entity<Buffer>>`

##### worktree_id

`Option<WorktreeId>`

##### label

`&str`

##### cx

`&App`

#### Returns

`Task<Option<TaskTemplate>>`


***

### list_tasks()

```rust
pub fn list_tasks(&self, buffer: Option<Entity<Buffer>>, language: Option<Arc<Language>>, worktree: Option<WorktreeId>, cx: &App) -> Task<Vec<(TaskSourceKind, TaskTemplate)>>
```

Defined in: [`packages/project/src/task_inventory.rs:432`](../../../../packages/project/src/task_inventory.rs#L432)

Pulls its task sources relevant to the worktree and the language given,
returns all task templates with their source kinds, worktree tasks first, language tasks second
and global tasks last. No specific order inside source kinds groups.

#### Parameters

##### buffer

`Option<Entity<Buffer>>`

##### language

`Option<Arc<Language>>`

##### worktree

`Option<WorktreeId>`

##### cx

`&App`

#### Returns

`Task<Vec<(TaskSourceKind, TaskTemplate)>>`


***

### used_and_current_resolved_tasks()

```rust
pub fn used_and_current_resolved_tasks(&self, task_contexts: Arc<TaskContexts>, cx: &Context<'_, Self>) -> Task<(Vec<(TaskSourceKind, ResolvedTask)>, Vec<(TaskSourceKind, ResolvedTask)>)>
```

Defined in: [`packages/project/src/task_inventory.rs:481`](../../../../packages/project/src/task_inventory.rs#L481)

Pulls its task sources relevant to the worktree and the language given and resolves them with the [`TaskContexts`](TaskContexts.md) given.
Joins the new resolutions with the resolved tasks that were used (spawned) before,
orders them so that the most recently used come first, all equally used ones are ordered so that the most specific tasks come first.
Deduplicates the tasks by their labels and context and splits the ordered list into two: used tasks and the rest, newly resolved tasks.

#### Parameters

##### task_contexts

`Arc<TaskContexts>`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<(Vec<(TaskSourceKind, ResolvedTask)>, Vec<(TaskSourceKind, ResolvedTask)>)>`


***

### last_scheduled_task()

```rust
pub fn last_scheduled_task(&self, task_id: Option<&TaskId>) -> Option<(TaskSourceKind, ResolvedTask)>
```

Defined in: [`packages/project/src/task_inventory.rs:662`](../../../../packages/project/src/task_inventory.rs#L662)

Returns the last scheduled task by task_id if provided.
Otherwise, returns the last scheduled task.

#### Parameters

##### task_id

`Option<&TaskId>`

#### Returns

`Option<(TaskSourceKind, ResolvedTask)>`


***

### task_scheduled()

```rust
pub fn task_scheduled(&self, task_source_kind: TaskSourceKind, resolved_task: ResolvedTask)
```

Defined in: [`packages/project/src/task_inventory.rs:677`](../../../../packages/project/src/task_inventory.rs#L677)

Registers task "usage" as being scheduled – to be used for LRU sorting when listing all tasks.

#### Parameters

##### task_source_kind

[`TaskSourceKind`](../enums/TaskSourceKind.md)

##### resolved_task

`ResolvedTask`


***

### delete_previously_used()

```rust
pub fn delete_previously_used(&self, id: &TaskId)
```

Defined in: [`packages/project/src/task_inventory.rs:691`](../../../../packages/project/src/task_inventory.rs#L691)

Deletes a resolved task from history, using its id.
A similar may still resurface in `used_and_current_resolved_tasks` when its `TaskTemplate` is resolved again.

#### Parameters

##### id

`&TaskId`


***

### global_templates_with_tag()

```rust
pub fn global_templates_with_tag(&self, tag: &str) -> Vec<(TaskSourceKind, TaskTemplate)>
```

Defined in: [`packages/project/src/task_inventory.rs:696`](../../../../packages/project/src/task_inventory.rs#L696)

Returns global task templates with the provided tag.

#### Parameters

##### tag

`&str`

#### Returns

`Vec<(TaskSourceKind, TaskTemplate)>`


***

### resolve_global_tasks_with_tag()

```rust
pub fn resolve_global_tasks_with_tag(&self, tag: &str, task_context: &TaskContext) -> Vec<(TaskSourceKind, ResolvedTask)>
```

Defined in: [`packages/project/src/task_inventory.rs:703`](../../../../packages/project/src/task_inventory.rs#L703)

Resolves global task templates with the provided tag against the provided task context.

#### Parameters

##### tag

`&str`

##### task_context

`&TaskContext`

#### Returns

`Vec<(TaskSourceKind, ResolvedTask)>`


***

### templates_with_hooks()

```rust
pub fn templates_with_hooks(&self, hooks: &HashSet<TaskHook>, worktree: WorktreeId) -> Vec<(TaskSourceKind, TaskTemplate)>
```

Defined in: [`packages/project/src/task_inventory.rs:721`](../../../../packages/project/src/task_inventory.rs#L721)

Returns all task templates (worktree and global) that have at least one
hook in the provided set.

#### Parameters

##### hooks

`&HashSet<TaskHook>`

##### worktree

`WorktreeId`

#### Returns

`Vec<(TaskSourceKind, TaskTemplate)>`


***

### update_file_based_tasks()

```rust
pub fn update_file_based_tasks(&self, location: TaskSettingsLocation<'_>, raw_tasks_json: Option<&str>) -> Result<(), InvalidSettingsError>
```

Defined in: [`packages/project/src/task_inventory.rs:762`](../../../../packages/project/src/task_inventory.rs#L762)

Updates in-memory task metadata from the JSON string given.
Will fail if the JSON is not a valid array of objects, but will continue if any object will not parse into a `TaskTemplate`.

Global tasks are updated for no worktree provided, otherwise the worktree metadata for a given path will be updated.

#### Parameters

##### location

[`TaskSettingsLocation<'_>`](../enums/TaskSettingsLocation.md)

##### raw_tasks_json

`Option<&str>`

#### Returns

`Result<(), InvalidSettingsError>`


***

### update_file_based_scenarios()

```rust
pub fn update_file_based_scenarios(&self, location: TaskSettingsLocation<'_>, raw_tasks_json: Option<&str>) -> Result<(), InvalidSettingsError>
```

Defined in: [`packages/project/src/task_inventory.rs:875`](../../../../packages/project/src/task_inventory.rs#L875)

Updates in-memory task metadata from the JSON string given.
Will fail if the JSON is not a valid array of objects, but will continue if any object will not parse into a `TaskTemplate`.

Global tasks are updated for no worktree provided, otherwise the worktree metadata for a given path will be updated.

#### Parameters

##### location

[`TaskSettingsLocation<'_>`](../enums/TaskSettingsLocation.md)

##### raw_tasks_json

`Option<&str>`

#### Returns

`Result<(), InvalidSettingsError>`

## Trait Implementations

- `impl Borrow for Inventory`
- `impl BorrowMut for Inventory`
- `impl Into for Inventory`
- `impl From for Inventory`
- `impl TryInto for Inventory`
- `impl TryFrom for Inventory`
- `impl Any for Inventory`
- `impl VZip for Inventory`
- `impl CastableFrom for Inventory`
- `impl CastableFrom for Inventory`
- `impl Read for Inventory`
- `impl IntoEither for Inventory`
- `impl ErasedDestructor for Inventory`
- `impl Same for Inventory`
- `impl Pointable for Inventory`
- `impl Instrument for Inventory`
- `impl WithSubscriber for Inventory`
- `impl FromAngle for Inventory`
- `impl IntoAngle for Inventory`
- `impl IntoCam16Unclamped for Inventory`
- `impl Cam16IntoUnclamped for Inventory`
- `impl ArraysFrom for Inventory`
- `impl ArraysInto for Inventory`
- `impl ComponentsFrom for Inventory`
- `impl TryComponentsInto for Inventory`
- `impl UintsFrom for Inventory`
- `impl UintsInto for Inventory`
- `impl AdaptIntoUnclamped for Inventory`
- `impl AdaptInto for Inventory`
- `impl IntoColor for Inventory`
- `impl IntoColorUnclamped for Inventory`
- `impl TryIntoColor for Inventory`
- `impl FromStimulus for Inventory`
- `impl IntoStimulus for Inventory`
- `impl Debug for Inventory`

## Auto Trait Implementations

`Freeze` `Send` `Sync` `Unpin` `UnsafeUnpin`

