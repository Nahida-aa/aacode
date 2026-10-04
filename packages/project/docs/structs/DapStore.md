---
id: DapStore
title: DapStore
---

# Struct: DapStore

Defined in: [`packages/project/src/debugger/dap_store.rs:94`](../../../../packages/project/src/debugger/dap_store.rs#L94)

## Definition

```rust
pub struct DapStore
```

_（存在非公开字段）_

## Implementations

### init()

```rust
pub fn init(client: &AnyProtoClient, cx: &App)
```

Defined in: [`packages/project/src/debugger/dap_store.rs:119`](../../../../packages/project/src/debugger/dap_store.rs#L119)

#### Parameters

##### client

`&AnyProtoClient`

##### cx

`&App`


***

### new_local()

```rust
pub fn new_local(http_client: Arc<dyn HttpClient>, node_runtime: NodeRuntime, fs: Arc<dyn Fs>, environment: Entity<ProjectEnvironment>, toolchain_store: Arc<dyn LanguageToolchainStore>, worktree_store: Entity<WorktreeStore>, breakpoint_store: Entity<BreakpointStore>, is_headless: bool, cx: &Context<'_, Self>) -> Self
```

Defined in: [`packages/project/src/debugger/dap_store.rs:134`](../../../../packages/project/src/debugger/dap_store.rs#L134)

#### Parameters

##### http_client

`Arc<dyn HttpClient>`

##### node_runtime

`NodeRuntime`

##### fs

`Arc<dyn Fs>`

##### environment

`Entity<ProjectEnvironment>`

##### toolchain_store

`Arc<dyn LanguageToolchainStore>`

##### worktree_store

`Entity<WorktreeStore>`

##### breakpoint_store

`Entity<BreakpointStore>`

##### is_headless

`bool`

##### cx

`&Context<'_, Self>`

#### Returns

`Self`


***

### new_remote()

```rust
pub fn new_remote(project_id: u64, remote_client: Entity<RemoteClient>, breakpoint_store: Entity<BreakpointStore>, worktree_store: Entity<WorktreeStore>, node_runtime: NodeRuntime, http_client: Arc<dyn HttpClient>, fs: Arc<dyn Fs>, cx: &Context<'_, Self>) -> Self
```

Defined in: [`packages/project/src/debugger/dap_store.rs:157`](../../../../packages/project/src/debugger/dap_store.rs#L157)

#### Parameters

##### project_id

`u64`

##### remote_client

`Entity<RemoteClient>`

##### breakpoint_store

`Entity<BreakpointStore>`

##### worktree_store

`Entity<WorktreeStore>`

##### node_runtime

`NodeRuntime`

##### http_client

`Arc<dyn HttpClient>`

##### fs

`Arc<dyn Fs>`

##### cx

`&Context<'_, Self>`

#### Returns

`Self`


***

### new_collab()

```rust
pub fn new_collab(_project_id: u64, _upstream_client: AnyProtoClient, breakpoint_store: Entity<BreakpointStore>, worktree_store: Entity<WorktreeStore>, fs: Arc<dyn Fs>, cx: &Context<'_, Self>) -> Self
```

Defined in: [`packages/project/src/debugger/dap_store.rs:178`](../../../../packages/project/src/debugger/dap_store.rs#L178)

#### Parameters

##### _project_id

`u64`

##### _upstream_client

`AnyProtoClient`

##### breakpoint_store

`Entity<BreakpointStore>`

##### worktree_store

`Entity<WorktreeStore>`

##### fs

`Arc<dyn Fs>`

##### cx

`&Context<'_, Self>`

#### Returns

`Self`


***

### get_debug_adapter_binary()

```rust
pub fn get_debug_adapter_binary(&self, definition: DebugTaskDefinition, session_id: SessionId, worktree: &Entity<Worktree>, console: UnboundedSender<String>, cx: &Context<'_, Self>) -> Task<Result<DebugAdapterBinary>>
```

Defined in: [`packages/project/src/debugger/dap_store.rs:243`](../../../../packages/project/src/debugger/dap_store.rs#L243)

#### Parameters

##### definition

`DebugTaskDefinition`

##### session_id

`SessionId`

##### worktree

`&Entity<Worktree>`

##### console

`UnboundedSender<String>`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<DebugAdapterBinary>>`


***

### debug_scenario_for_build_task()

```rust
pub fn debug_scenario_for_build_task(&self, build: TaskTemplate, adapter: DebugAdapterName, label: SharedString, cx: &App) -> Task<Option<DebugScenario>>
```

Defined in: [`packages/project/src/debugger/dap_store.rs:372`](../../../../packages/project/src/debugger/dap_store.rs#L372)

#### Parameters

##### build

`TaskTemplate`

##### adapter

`DebugAdapterName`

##### label

`SharedString`

##### cx

`&App`

#### Returns

`Task<Option<DebugScenario>>`


***

### run_debug_locator()

```rust
pub fn run_debug_locator(&self, locator_name: &str, build_command: SpawnInTerminal, cx: &Context<'_, Self>) -> Task<Result<DebugRequest>>
```

Defined in: [`packages/project/src/debugger/dap_store.rs:391`](../../../../packages/project/src/debugger/dap_store.rs#L391)

#### Parameters

##### locator_name

`&str`

##### build_command

`SpawnInTerminal`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<DebugRequest>>`


***

### new_session()

```rust
pub fn new_session(&self, label: Option<SharedString>, adapter: DebugAdapterName, task_context: SharedTaskContext, parent_session: Option<Entity<Session>>, quirks: SessionQuirks, cx: &Context<'_, Self>) -> Entity<Session>
```

Defined in: [`packages/project/src/debugger/dap_store.rs:450`](../../../../packages/project/src/debugger/dap_store.rs#L450)

#### Parameters

##### label

`Option<SharedString>`

##### adapter

`DebugAdapterName`

##### task_context

`SharedTaskContext`

##### parent_session

`Option<Entity<Session>>`

##### quirks

[`SessionQuirks`](SessionQuirks.md)

##### cx

`&Context<'_, Self>`

#### Returns

`Entity<Session>`


***

### boot_session()

```rust
pub fn boot_session(&self, session: Entity<Session>, definition: DebugTaskDefinition, worktree: Entity<Worktree>, cx: &Context<'_, Self>) -> Task<Result<()>>
```

Defined in: [`packages/project/src/debugger/dap_store.rs:509`](../../../../packages/project/src/debugger/dap_store.rs#L509)

#### Parameters

##### session

`Entity<Session>`

##### definition

`DebugTaskDefinition`

##### worktree

`Entity<Worktree>`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<()>>`


***

### session_by_id()

```rust
pub fn session_by_id<impl Borrow<SessionId>: Borrow>(&self, session_id: impl ?) -> Option<Entity<Session>>
```

Defined in: [`packages/project/src/debugger/dap_store.rs:543`](../../../../packages/project/src/debugger/dap_store.rs#L543)

#### Parameters

##### session_id

`impl ?`

#### Returns

`Option<Entity<Session>>`


***

### sessions()

```rust
pub fn sessions(&self) -> impl ?
```

Defined in: [`packages/project/src/debugger/dap_store.rs:551`](../../../../packages/project/src/debugger/dap_store.rs#L551)

#### Returns

`impl ?`


***

### capabilities_by_id()

```rust
pub fn capabilities_by_id<impl Borrow<SessionId>: Borrow>(&self, session_id: impl ?, cx: &App) -> Option<Capabilities>
```

Defined in: [`packages/project/src/debugger/dap_store.rs:555`](../../../../packages/project/src/debugger/dap_store.rs#L555)

#### Parameters

##### session_id

`impl ?`

##### cx

`&App`

#### Returns

`Option<Capabilities>`


***

### breakpoint_store()

```rust
pub fn breakpoint_store(&self) -> &Entity<BreakpointStore>
```

Defined in: [`packages/project/src/debugger/dap_store.rs:566`](../../../../packages/project/src/debugger/dap_store.rs#L566)

#### Returns

`&Entity<BreakpointStore>`


***

### worktree_store()

```rust
pub fn worktree_store(&self) -> &Entity<WorktreeStore>
```

Defined in: [`packages/project/src/debugger/dap_store.rs:570`](../../../../packages/project/src/debugger/dap_store.rs#L570)

#### Returns

`&Entity<WorktreeStore>`


***

### resolve_inline_value_locations()

```rust
pub fn resolve_inline_value_locations(&self, session: Entity<Session>, stack_frame_id: StackFrameId, buffer_handle: Entity<Buffer>, inline_value_locations: Vec<InlineValueLocation>, cx: &Context<'_, Self>) -> Task<Result<Vec<InlayHint>>>
```

Defined in: [`packages/project/src/debugger/dap_store.rs:620`](../../../../packages/project/src/debugger/dap_store.rs#L620)

#### Parameters

##### session

`Entity<Session>`

##### stack_frame_id

`StackFrameId`

##### buffer_handle

`Entity<Buffer>`

##### inline_value_locations

`Vec<InlineValueLocation>`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<Vec<InlayHint>>>`


***

### shutdown_sessions()

```rust
pub fn shutdown_sessions(&self, cx: &Context<'_, Self>) -> Task<()>
```

Defined in: [`packages/project/src/debugger/dap_store.rs:726`](../../../../packages/project/src/debugger/dap_store.rs#L726)

#### Parameters

##### cx

`&Context<'_, Self>`

#### Returns

`Task<()>`


***

### shutdown_session()

```rust
pub fn shutdown_session(&self, session_id: SessionId, cx: &Context<'_, Self>) -> Task<Result<()>>
```

Defined in: [`packages/project/src/debugger/dap_store.rs:737`](../../../../packages/project/src/debugger/dap_store.rs#L737)

#### Parameters

##### session_id

`SessionId`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<()>>`


***

### shared()

```rust
pub fn shared(&self, project_id: u64, downstream_client: AnyProtoClient, _: &Context<'_, Self>)
```

Defined in: [`packages/project/src/debugger/dap_store.rs:792`](../../../../packages/project/src/debugger/dap_store.rs#L792)

#### Parameters

##### project_id

`u64`

##### downstream_client

`AnyProtoClient`

##### _

`&Context<'_, Self>`


***

### unshared()

```rust
pub fn unshared(&self, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/debugger/dap_store.rs:801`](../../../../packages/project/src/debugger/dap_store.rs#L801)

#### Parameters

##### cx

`&Context<'_, Self>`


***

### sync_adapter_options()

```rust
pub fn sync_adapter_options(&self, session: &Entity<Session>, cx: &App) -> Arc<PersistedAdapterOptions>
```

Defined in: [`packages/project/src/debugger/dap_store.rs:898`](../../../../packages/project/src/debugger/dap_store.rs#L898)

#### Parameters

##### session

`&Entity<Session>`

##### cx

`&App`

#### Returns

`Arc<PersistedAdapterOptions>`


***

### set_adapter_options()

```rust
pub fn set_adapter_options(&self, adapter: DebugAdapterName, options: PersistedAdapterOptions)
```

Defined in: [`packages/project/src/debugger/dap_store.rs:921`](../../../../packages/project/src/debugger/dap_store.rs#L921)

#### Parameters

##### adapter

`DebugAdapterName`

##### options

[`PersistedAdapterOptions`](PersistedAdapterOptions.md)


***

### adapter_options()

```rust
pub fn adapter_options(&self, name: &str) -> Option<Arc<PersistedAdapterOptions>>
```

Defined in: [`packages/project/src/debugger/dap_store.rs:929`](../../../../packages/project/src/debugger/dap_store.rs#L929)

#### Parameters

##### name

`&str`

#### Returns

`Option<Arc<PersistedAdapterOptions>>`


***

### all_adapter_options()

```rust
pub fn all_adapter_options(&self) -> &BTreeMap<DebugAdapterName, Arc<PersistedAdapterOptions>>
```

Defined in: [`packages/project/src/debugger/dap_store.rs:933`](../../../../packages/project/src/debugger/dap_store.rs#L933)

#### Returns

`&BTreeMap<DebugAdapterName, Arc<PersistedAdapterOptions>>`

## Trait Implementations

- `impl Borrow for DapStore`
- `impl BorrowMut for DapStore`
- `impl Into for DapStore`
- `impl From for DapStore`
- `impl TryInto for DapStore`
- `impl TryFrom for DapStore`
- `impl Any for DapStore`
- `impl VZip for DapStore`
- `impl CastableFrom for DapStore`
- `impl CastableFrom for DapStore`
- `impl Read for DapStore`
- `impl IntoEither for DapStore`
- `impl ErasedDestructor for DapStore`
- `impl Same for DapStore`
- `impl Pointable for DapStore`
- `impl Instrument for DapStore`
- `impl WithSubscriber for DapStore`
- `impl FromAngle for DapStore`
- `impl IntoAngle for DapStore`
- `impl IntoCam16Unclamped for DapStore`
- `impl Cam16IntoUnclamped for DapStore`
- `impl ArraysFrom for DapStore`
- `impl ArraysInto for DapStore`
- `impl ComponentsFrom for DapStore`
- `impl TryComponentsInto for DapStore`
- `impl UintsFrom for DapStore`
- `impl UintsInto for DapStore`
- `impl AdaptIntoUnclamped for DapStore`
- `impl AdaptInto for DapStore`
- `impl IntoColor for DapStore`
- `impl IntoColorUnclamped for DapStore`
- `impl TryIntoColor for DapStore`
- `impl FromStimulus for DapStore`
- `impl IntoStimulus for DapStore`
- `impl EventEmitter for DapStore`

## Auto Trait Implementations

`Freeze` `Send` `Sync` `Unpin` `UnsafeUnpin`

