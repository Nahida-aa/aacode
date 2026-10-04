---
id: ProjectEnvironment
title: ProjectEnvironment
---

# Struct: ProjectEnvironment

Defined in: [`packages/project/src/environment.rs:20`](../../../../packages/project/src/environment.rs#L20)

## Definition

```rust
pub struct ProjectEnvironment
```

_（存在非公开字段）_

## Implementations

### new()

```rust
pub fn new(cli_environment: Option<HashMap<String, String>>, worktree_store: WeakEntity<WorktreeStore>, remote_client: Option<WeakEntity<RemoteClient>>, is_remote_project: bool, cx: &Context<'_, Self>) -> Self
```

Defined in: [`packages/project/src/environment.rs:39`](../../../../packages/project/src/environment.rs#L39)

#### Parameters

##### cli_environment

`Option<HashMap<String, String>>`

##### worktree_store

`WeakEntity<WorktreeStore>`

##### remote_client

`Option<WeakEntity<RemoteClient>>`

##### is_remote_project

`bool`

##### cx

`&Context<'_, Self>`

#### Returns

`Self`


***

### buffer_environment()

```rust
pub fn buffer_environment(&self, buffer: &Entity<Buffer>, worktree_store: &Entity<WorktreeStore>, cx: &Context<'_, Self>) -> Shared<Task<Option<HashMap<String, String>>>>
```

Defined in: [`packages/project/src/environment.rs:82`](../../../../packages/project/src/environment.rs#L82)

#### Parameters

##### buffer

`&Entity<Buffer>`

##### worktree_store

`&Entity<WorktreeStore>`

##### cx

`&Context<'_, Self>`

#### Returns

`Shared<Task<Option<HashMap<String, String>>>>`


***

### worktree_environment()

```rust
pub fn worktree_environment(&self, worktree: Entity<Worktree>, cx: &App) -> Shared<Task<Option<HashMap<String, String>>>>
```

Defined in: [`packages/project/src/environment.rs:104`](../../../../packages/project/src/environment.rs#L104)

#### Parameters

##### worktree

`Entity<Worktree>`

##### cx

`&App`

#### Returns

`Shared<Task<Option<HashMap<String, String>>>>`


***

### directory_environment()

```rust
pub fn directory_environment(&self, abs_path: Arc<Path>, cx: &App) -> Shared<Task<Option<HashMap<String, String>>>>
```

Defined in: [`packages/project/src/environment.rs:141`](../../../../packages/project/src/environment.rs#L141)

#### Parameters

##### abs_path

`Arc<Path>`

##### cx

`&App`

#### Returns

`Shared<Task<Option<HashMap<String, String>>>>`


***

### default_environment()

```rust
pub fn default_environment(&self, cx: &App) -> Shared<Task<Option<HashMap<String, String>>>>
```

Defined in: [`packages/project/src/environment.rs:173`](../../../../packages/project/src/environment.rs#L173)

Returns the project environment using the default worktree path.
This ensures that project-specific environment variables (e.g. from `.envrc`)
are loaded from the project directory rather than the home directory.

#### Parameters

##### cx

`&App`

#### Returns

`Shared<Task<Option<HashMap<String, String>>>>`


***

### local_directory_environment()

```rust
pub fn local_directory_environment(&self, shell: &Shell, abs_path: Arc<Path>, cx: &App) -> Shared<Task<Option<HashMap<String, String>>>>
```

Defined in: [`packages/project/src/environment.rs:195`](../../../../packages/project/src/environment.rs#L195)

Returns the project environment, if possible.
If the project was opened from the CLI, then the inherited CLI environment is returned.
If it wasn't opened from the CLI, and an absolute path is given, then a shell is spawned in
that directory, to get environment variables as if the user has `cd`'d there.

#### Parameters

##### shell

`&Shell`

##### abs_path

`Arc<Path>`

##### cx

`&App`

#### Returns

`Shared<Task<Option<HashMap<String, String>>>>`


***

### remote_directory_environment()

```rust
pub fn remote_directory_environment(&self, shell: &Shell, abs_path: Arc<Path>, remote_client: Entity<RemoteClient>, cx: &App) -> Shared<Task<Option<HashMap<String, String>>>>
```

Defined in: [`packages/project/src/environment.rs:252`](../../../../packages/project/src/environment.rs#L252)

#### Parameters

##### shell

`&Shell`

##### abs_path

`Arc<Path>`

##### remote_client

`Entity<RemoteClient>`

##### cx

`&App`

#### Returns

`Shared<Task<Option<HashMap<String, String>>>>`


***

### peek_environment_error()

```rust
pub fn peek_environment_error(&self) -> Option<&String>
```

Defined in: [`packages/project/src/environment.rs:284`](../../../../packages/project/src/environment.rs#L284)

#### Returns

`Option<&String>`


***

### pop_environment_error()

```rust
pub fn pop_environment_error(&self) -> Option<String>
```

Defined in: [`packages/project/src/environment.rs:288`](../../../../packages/project/src/environment.rs#L288)

#### Returns

`Option<String>`

## Trait Implementations

- `impl Borrow for ProjectEnvironment`
- `impl BorrowMut for ProjectEnvironment`
- `impl Into for ProjectEnvironment`
- `impl From for ProjectEnvironment`
- `impl TryInto for ProjectEnvironment`
- `impl TryFrom for ProjectEnvironment`
- `impl Any for ProjectEnvironment`
- `impl VZip for ProjectEnvironment`
- `impl CastableFrom for ProjectEnvironment`
- `impl CastableFrom for ProjectEnvironment`
- `impl Read for ProjectEnvironment`
- `impl IntoEither for ProjectEnvironment`
- `impl ErasedDestructor for ProjectEnvironment`
- `impl Same for ProjectEnvironment`
- `impl Pointable for ProjectEnvironment`
- `impl Instrument for ProjectEnvironment`
- `impl WithSubscriber for ProjectEnvironment`
- `impl FromAngle for ProjectEnvironment`
- `impl IntoAngle for ProjectEnvironment`
- `impl IntoCam16Unclamped for ProjectEnvironment`
- `impl Cam16IntoUnclamped for ProjectEnvironment`
- `impl ArraysFrom for ProjectEnvironment`
- `impl ArraysInto for ProjectEnvironment`
- `impl ComponentsFrom for ProjectEnvironment`
- `impl TryComponentsInto for ProjectEnvironment`
- `impl UintsFrom for ProjectEnvironment`
- `impl UintsInto for ProjectEnvironment`
- `impl AdaptIntoUnclamped for ProjectEnvironment`
- `impl AdaptInto for ProjectEnvironment`
- `impl IntoColor for ProjectEnvironment`
- `impl IntoColorUnclamped for ProjectEnvironment`
- `impl TryIntoColor for ProjectEnvironment`
- `impl FromStimulus for ProjectEnvironment`
- `impl IntoStimulus for ProjectEnvironment`
- `impl EventEmitter for ProjectEnvironment`

## Auto Trait Implementations

`Freeze` `Send` `Sync` `Unpin` `UnsafeUnpin`

