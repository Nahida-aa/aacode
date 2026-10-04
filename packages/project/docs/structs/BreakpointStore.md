---
id: BreakpointStore
title: BreakpointStore
---

# Struct: BreakpointStore

Defined in: [`packages/project/src/debugger/breakpoint_store.rs:153`](../../../../packages/project/src/debugger/breakpoint_store.rs#L153)

## Definition

```rust
pub struct BreakpointStore
```

_（存在非公开字段）_

## Implementations

### init()

```rust
pub fn init(client: &AnyProtoClient)
```

Defined in: [`packages/project/src/debugger/breakpoint_store.rs:165`](../../../../packages/project/src/debugger/breakpoint_store.rs#L165)

#### Parameters

##### client

`&AnyProtoClient`


***

### local()

```rust
pub fn local(worktree_store: Entity<WorktreeStore>, buffer_store: Entity<BufferStore>) -> Self
```

Defined in: [`packages/project/src/debugger/breakpoint_store.rs:169`](../../../../packages/project/src/debugger/breakpoint_store.rs#L169)

#### Parameters

##### worktree_store

`Entity<WorktreeStore>`

##### buffer_store

`Entity<BufferStore>`

#### Returns

`Self`


***

### shared()

```rust
pub fn shared(&self, project_id: u64, downstream_client: AnyProtoClient)
```

Defined in: [`packages/project/src/debugger/breakpoint_store.rs:201`](../../../../packages/project/src/debugger/breakpoint_store.rs#L201)

#### Parameters

##### project_id

`u64`

##### downstream_client

`AnyProtoClient`


***

### abs_path_from_buffer()

```rust
pub fn abs_path_from_buffer(buffer: &Entity<Buffer>, cx: &App) -> Option<Arc<Path>>
```

Defined in: [`packages/project/src/debugger/breakpoint_store.rs:396`](../../../../packages/project/src/debugger/breakpoint_store.rs#L396)

#### Parameters

##### buffer

`&Entity<Buffer>`

##### cx

`&App`

#### Returns

`Option<Arc<Path>>`


***

### toggle_breakpoint()

```rust
pub fn toggle_breakpoint(&self, buffer: Entity<Buffer>, breakpoint: BreakpointWithPosition, edit_action: BreakpointEditAction, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/debugger/breakpoint_store.rs:402`](../../../../packages/project/src/debugger/breakpoint_store.rs#L402)

#### Parameters

##### buffer

`Entity<Buffer>`

##### breakpoint

[`BreakpointWithPosition`](BreakpointWithPosition.md)

##### edit_action

[`BreakpointEditAction`](../enums/BreakpointEditAction.md)

##### cx

`&Context<'_, Self>`


***

### on_file_rename()

```rust
pub fn on_file_rename(&self, old_path: Arc<Path>, new_path: Arc<Path>, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/debugger/breakpoint_store.rs:616`](../../../../packages/project/src/debugger/breakpoint_store.rs#L616)

#### Parameters

##### old_path

`Arc<Path>`

##### new_path

`Arc<Path>`

##### cx

`&Context<'_, Self>`


***

### clear_breakpoints()

```rust
pub fn clear_breakpoints(&self, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/debugger/breakpoint_store.rs:629`](../../../../packages/project/src/debugger/breakpoint_store.rs#L629)

#### Parameters

##### cx

`&Context<'_, Self>`


***

### breakpoints()

```rust
pub fn breakpoints<'a>(&self, buffer: &'a Entity<Buffer>, range: Option<Range<Anchor>>, buffer_snapshot: &'a BufferSnapshot, cx: &App) -> impl ? + ?
```

Defined in: [`packages/project/src/debugger/breakpoint_store.rs:635`](../../../../packages/project/src/debugger/breakpoint_store.rs#L635)

#### Parameters

##### buffer

`&'a Entity<Buffer>`

##### range

`Option<Range<Anchor>>`

##### buffer_snapshot

`&'a BufferSnapshot`

##### cx

`&App`

#### Returns

`impl ? + ?`


***

### active_position()

```rust
pub fn active_position(&self) -> Option<&ActiveStackFrame>
```

Defined in: [`packages/project/src/debugger/breakpoint_store.rs:674`](../../../../packages/project/src/debugger/breakpoint_store.rs#L674)

#### Returns

`Option<&ActiveStackFrame>`


***

### active_debug_line_pane_id()

```rust
pub fn active_debug_line_pane_id(&self) -> Option<EntityId>
```

Defined in: [`packages/project/src/debugger/breakpoint_store.rs:678`](../../../../packages/project/src/debugger/breakpoint_store.rs#L678)

#### Returns

`Option<EntityId>`


***

### set_active_debug_pane_id()

```rust
pub fn set_active_debug_pane_id(&self, pane_id: EntityId)
```

Defined in: [`packages/project/src/debugger/breakpoint_store.rs:682`](../../../../packages/project/src/debugger/breakpoint_store.rs#L682)

#### Parameters

##### pane_id

`EntityId`


***

### remove_active_position()

```rust
pub fn remove_active_position(&self, session_id: Option<SessionId>, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/debugger/breakpoint_store.rs:686`](../../../../packages/project/src/debugger/breakpoint_store.rs#L686)

#### Parameters

##### session_id

`Option<SessionId>`

##### cx

`&Context<'_, Self>`


***

### set_active_position()

```rust
pub fn set_active_position(&self, position: ActiveStackFrame, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/debugger/breakpoint_store.rs:708`](../../../../packages/project/src/debugger/breakpoint_store.rs#L708)

#### Parameters

##### position

[`ActiveStackFrame`](ActiveStackFrame.md)

##### cx

`&Context<'_, Self>`


***

### breakpoint_at_row()

```rust
pub fn breakpoint_at_row(&self, path: &Path, row: u32, cx: &App) -> Option<(Entity<Buffer>, BreakpointWithPosition)>
```

Defined in: [`packages/project/src/debugger/breakpoint_store.rs:728`](../../../../packages/project/src/debugger/breakpoint_store.rs#L728)

#### Parameters

##### path

`&Path`

##### row

`u32`

##### cx

`&App`

#### Returns

`Option<(Entity<Buffer>, BreakpointWithPosition)>`


***

### breakpoints_from_path()

```rust
pub fn breakpoints_from_path(&self, path: &Arc<Path>) -> Vec<BreakpointWithPosition>
```

Defined in: [`packages/project/src/debugger/breakpoint_store.rs:745`](../../../../packages/project/src/debugger/breakpoint_store.rs#L745)

#### Parameters

##### path

`&Arc<Path>`

#### Returns

`Vec<BreakpointWithPosition>`


***

### source_breakpoints_from_path()

```rust
pub fn source_breakpoints_from_path(&self, path: &Arc<Path>, cx: &App) -> Vec<SourceBreakpoint>
```

Defined in: [`packages/project/src/debugger/breakpoint_store.rs:752`](../../../../packages/project/src/debugger/breakpoint_store.rs#L752)

#### Parameters

##### path

`&Arc<Path>`

##### cx

`&App`

#### Returns

`Vec<SourceBreakpoint>`


***

### all_breakpoints()

```rust
pub fn all_breakpoints(&self) -> BTreeMap<Arc<Path>, Vec<BreakpointWithPosition>>
```

Defined in: [`packages/project/src/debugger/breakpoint_store.rs:780`](../../../../packages/project/src/debugger/breakpoint_store.rs#L780)

#### Returns

`BTreeMap<Arc<Path>, Vec<BreakpointWithPosition>>`


***

### all_source_breakpoints()

```rust
pub fn all_source_breakpoints(&self, cx: &App) -> BTreeMap<Arc<Path>, Vec<SourceBreakpoint>>
```

Defined in: [`packages/project/src/debugger/breakpoint_store.rs:791`](../../../../packages/project/src/debugger/breakpoint_store.rs#L791)

#### Parameters

##### cx

`&App`

#### Returns

`BTreeMap<Arc<Path>, Vec<SourceBreakpoint>>`


***

### with_serialized_breakpoints()

```rust
pub fn with_serialized_breakpoints(&self, breakpoints: BTreeMap<Arc<Path>, Vec<SourceBreakpoint>>, cx: &Context<'_, BreakpointStore>) -> Task<Result<()>>
```

Defined in: [`packages/project/src/debugger/breakpoint_store.rs:820`](../../../../packages/project/src/debugger/breakpoint_store.rs#L820)

#### Parameters

##### breakpoints

`BTreeMap<Arc<Path>, Vec<SourceBreakpoint>>`

##### cx

`&Context<'_, BreakpointStore>`

#### Returns

`Task<Result<()>>`

## Trait Implementations

- `impl Borrow for BreakpointStore`
- `impl BorrowMut for BreakpointStore`
- `impl Into for BreakpointStore`
- `impl From for BreakpointStore`
- `impl TryInto for BreakpointStore`
- `impl TryFrom for BreakpointStore`
- `impl Any for BreakpointStore`
- `impl VZip for BreakpointStore`
- `impl CastableFrom for BreakpointStore`
- `impl CastableFrom for BreakpointStore`
- `impl Read for BreakpointStore`
- `impl IntoEither for BreakpointStore`
- `impl ErasedDestructor for BreakpointStore`
- `impl Same for BreakpointStore`
- `impl Pointable for BreakpointStore`
- `impl Instrument for BreakpointStore`
- `impl WithSubscriber for BreakpointStore`
- `impl FromAngle for BreakpointStore`
- `impl IntoAngle for BreakpointStore`
- `impl IntoCam16Unclamped for BreakpointStore`
- `impl Cam16IntoUnclamped for BreakpointStore`
- `impl ArraysFrom for BreakpointStore`
- `impl ArraysInto for BreakpointStore`
- `impl ComponentsFrom for BreakpointStore`
- `impl TryComponentsInto for BreakpointStore`
- `impl UintsFrom for BreakpointStore`
- `impl UintsInto for BreakpointStore`
- `impl AdaptIntoUnclamped for BreakpointStore`
- `impl AdaptInto for BreakpointStore`
- `impl IntoColor for BreakpointStore`
- `impl IntoColorUnclamped for BreakpointStore`
- `impl TryIntoColor for BreakpointStore`
- `impl FromStimulus for BreakpointStore`
- `impl IntoStimulus for BreakpointStore`
- `impl EventEmitter for BreakpointStore`

## Auto Trait Implementations

`Freeze` `Unpin` `UnsafeUnpin`

