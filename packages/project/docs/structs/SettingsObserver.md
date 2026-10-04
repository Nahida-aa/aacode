---
id: SettingsObserver
title: SettingsObserver
---

# Struct: SettingsObserver

Defined in: [`packages/project/src/project_settings.rs:823`](../../../../packages/project/src/project_settings.rs#L823)

## Definition

```rust
pub struct SettingsObserver
```

_（存在非公开字段）_

## Implementations

### init()

```rust
pub fn init(client: &AnyProtoClient)
```

Defined in: [`packages/project/src/project_settings.rs:844`](../../../../packages/project/src/project_settings.rs#L844)

#### Parameters

##### client

`&AnyProtoClient`


***

### new_local()

```rust
pub fn new_local(fs: Arc<dyn Fs>, worktree_store: Entity<WorktreeStore>, task_store: Entity<TaskStore>, watch_global_configs: bool, cx: &Context<'_, Self>) -> Self
```

Defined in: [`packages/project/src/project_settings.rs:849`](../../../../packages/project/src/project_settings.rs#L849)

#### Parameters

##### fs

`Arc<dyn Fs>`

##### worktree_store

`Entity<WorktreeStore>`

##### task_store

`Entity<TaskStore>`

##### watch_global_configs

`bool`

##### cx

`&Context<'_, Self>`

#### Returns

`Self`


***

### new_remote()

```rust
pub fn new_remote(fs: Arc<dyn Fs>, worktree_store: Entity<WorktreeStore>, task_store: Entity<TaskStore>, upstream_client: Option<AnyProtoClient>, via_collab: bool, cx: &Context<'_, Self>) -> Self
```

Defined in: [`packages/project/src/project_settings.rs:971`](../../../../packages/project/src/project_settings.rs#L971)

#### Parameters

##### fs

`Arc<dyn Fs>`

##### worktree_store

`Entity<WorktreeStore>`

##### task_store

`Entity<TaskStore>`

##### upstream_client

`Option<AnyProtoClient>`

##### via_collab

`bool`

##### cx

`&Context<'_, Self>`

#### Returns

`Self`


***

### shared()

```rust
pub fn shared(&self, project_id: u64, downstream_client: AnyProtoClient, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/project_settings.rs:1026`](../../../../packages/project/src/project_settings.rs#L1026)

#### Parameters

##### project_id

`u64`

##### downstream_client

`AnyProtoClient`

##### cx

`&Context<'_, Self>`


***

### unshared()

```rust
pub fn unshared(&self, _: &Context<'_, Self>)
```

Defined in: [`packages/project/src/project_settings.rs:1074`](../../../../packages/project/src/project_settings.rs#L1074)

#### Parameters

##### _

`&Context<'_, Self>`

## Trait Implementations

- `impl Borrow for SettingsObserver`
- `impl BorrowMut for SettingsObserver`
- `impl Into for SettingsObserver`
- `impl From for SettingsObserver`
- `impl TryInto for SettingsObserver`
- `impl TryFrom for SettingsObserver`
- `impl Any for SettingsObserver`
- `impl VZip for SettingsObserver`
- `impl CastableFrom for SettingsObserver`
- `impl CastableFrom for SettingsObserver`
- `impl Read for SettingsObserver`
- `impl IntoEither for SettingsObserver`
- `impl ErasedDestructor for SettingsObserver`
- `impl Same for SettingsObserver`
- `impl Pointable for SettingsObserver`
- `impl Instrument for SettingsObserver`
- `impl WithSubscriber for SettingsObserver`
- `impl FromAngle for SettingsObserver`
- `impl IntoAngle for SettingsObserver`
- `impl IntoCam16Unclamped for SettingsObserver`
- `impl Cam16IntoUnclamped for SettingsObserver`
- `impl ArraysFrom for SettingsObserver`
- `impl ArraysInto for SettingsObserver`
- `impl ComponentsFrom for SettingsObserver`
- `impl TryComponentsInto for SettingsObserver`
- `impl UintsFrom for SettingsObserver`
- `impl UintsInto for SettingsObserver`
- `impl AdaptIntoUnclamped for SettingsObserver`
- `impl AdaptInto for SettingsObserver`
- `impl IntoColor for SettingsObserver`
- `impl IntoColorUnclamped for SettingsObserver`
- `impl TryIntoColor for SettingsObserver`
- `impl FromStimulus for SettingsObserver`
- `impl IntoStimulus for SettingsObserver`
- `impl EventEmitter for SettingsObserver`

## Auto Trait Implementations

`Freeze` `Unpin` `UnsafeUnpin`

