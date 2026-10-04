---
id: TaskStore
title: TaskStore
---

# Enum: TaskStore

Defined in: [`packages/project/src/task_store.rs:26`](../../../../packages/project/src/task_store.rs#L26)

## Definition

```rust
pub enum TaskStore {
    Functional(StoreState),
    Noop,
}
```

## Implementations

### init()

```rust
pub fn init(client: Option<&AnyProtoClient>)
```

Defined in: [`packages/project/src/task_store.rs:60`](../../../../packages/project/src/task_store.rs#L60)

#### Parameters

##### client

`Option<&AnyProtoClient>`


***

### local()

```rust
pub fn local(buffer_store: WeakEntity<BufferStore>, worktree_store: Entity<WorktreeStore>, toolchain_store: Arc<dyn LanguageToolchainStore>, environment: Entity<ProjectEnvironment>, git_store: Entity<GitStore>, cx: &Context<'_, Self>) -> Self
```

Defined in: [`packages/project/src/task_store.rs:162`](../../../../packages/project/src/task_store.rs#L162)

#### Parameters

##### buffer_store

`WeakEntity<BufferStore>`

##### worktree_store

`Entity<WorktreeStore>`

##### toolchain_store

`Arc<dyn LanguageToolchainStore>`

##### environment

`Entity<ProjectEnvironment>`

##### git_store

`Entity<GitStore>`

##### cx

`&Context<'_, Self>`

#### Returns

`Self`


***

### remote()

```rust
pub fn remote(buffer_store: WeakEntity<BufferStore>, worktree_store: Entity<WorktreeStore>, toolchain_store: Arc<dyn LanguageToolchainStore>, upstream_client: AnyProtoClient, project_id: u64, git_store: Entity<GitStore>, cx: &Context<'_, Self>) -> Self
```

Defined in: [`packages/project/src/task_store.rs:183`](../../../../packages/project/src/task_store.rs#L183)

#### Parameters

##### buffer_store

`WeakEntity<BufferStore>`

##### worktree_store

`Entity<WorktreeStore>`

##### toolchain_store

`Arc<dyn LanguageToolchainStore>`

##### upstream_client

`AnyProtoClient`

##### project_id

`u64`

##### git_store

`Entity<GitStore>`

##### cx

`&Context<'_, Self>`

#### Returns

`Self`


***

### task_context_for_location()

```rust
pub fn task_context_for_location(&self, captured_variables: TaskVariables, location: Location, cx: &App) -> Task<Result<Option<TaskContext>>>
```

Defined in: [`packages/project/src/task_store.rs:205`](../../../../packages/project/src/task_store.rs#L205)

#### Parameters

##### captured_variables

`TaskVariables`

##### location

`Location`

##### cx

`&App`

#### Returns

`Task<Result<Option<TaskContext>>>`


***

### task_inventory()

```rust
pub fn task_inventory(&self) -> Option<&Entity<Inventory>>
```

Defined in: [`packages/project/src/task_store.rs:240`](../../../../packages/project/src/task_store.rs#L240)

#### Returns

`Option<&Entity<Inventory>>`


***

### shared()

```rust
pub fn shared(&self, remote_id: u64, new_downstream_client: AnyProtoClient, _cx: &App)
```

Defined in: [`packages/project/src/task_store.rs:247`](../../../../packages/project/src/task_store.rs#L247)

#### Parameters

##### remote_id

`u64`

##### new_downstream_client

`AnyProtoClient`

##### _cx

`&App`


***

### unshared()

```rust
pub fn unshared(&self, _: &Context<'_, Self>)
```

Defined in: [`packages/project/src/task_store.rs:259`](../../../../packages/project/src/task_store.rs#L259)

#### Parameters

##### _

`&Context<'_, Self>`

## Trait Implementations

- `impl Borrow for TaskStore`
- `impl BorrowMut for TaskStore`
- `impl Into for TaskStore`
- `impl From for TaskStore`
- `impl TryInto for TaskStore`
- `impl TryFrom for TaskStore`
- `impl Any for TaskStore`
- `impl VZip for TaskStore`
- `impl CastableFrom for TaskStore`
- `impl CastableFrom for TaskStore`
- `impl Read for TaskStore`
- `impl IntoEither for TaskStore`
- `impl ErasedDestructor for TaskStore`
- `impl Same for TaskStore`
- `impl Pointable for TaskStore`
- `impl Instrument for TaskStore`
- `impl WithSubscriber for TaskStore`
- `impl FromAngle for TaskStore`
- `impl IntoAngle for TaskStore`
- `impl IntoCam16Unclamped for TaskStore`
- `impl Cam16IntoUnclamped for TaskStore`
- `impl ArraysFrom for TaskStore`
- `impl ArraysInto for TaskStore`
- `impl ComponentsFrom for TaskStore`
- `impl TryComponentsInto for TaskStore`
- `impl UintsFrom for TaskStore`
- `impl UintsInto for TaskStore`
- `impl AdaptIntoUnclamped for TaskStore`
- `impl AdaptInto for TaskStore`
- `impl IntoColor for TaskStore`
- `impl IntoColorUnclamped for TaskStore`
- `impl TryIntoColor for TaskStore`
- `impl FromStimulus for TaskStore`
- `impl IntoStimulus for TaskStore`
- `impl EventEmitter for TaskStore`

## Auto Trait Implementations

`Freeze` `Send` `Sync` `Unpin` `UnsafeUnpin`

