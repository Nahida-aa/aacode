---
id: DiffBufferList
title: DiffBufferList
---

# Struct: DiffBufferList

Defined in: [`packages/project/src/git_store/diff_buffer_list.rs:40`](../../../../packages/project/src/git_store/diff_buffer_list.rs#L40)

## Definition

```rust
pub struct DiffBufferList
```

_（存在非公开字段）_

## Implementations

### new()

```rust
pub fn new(source: DiffBase, git_store: Entity<GitStore>, repo: Option<Entity<Repository>>, cx: &Context<'_, Self>) -> Self
```

Defined in: [`packages/project/src/git_store/diff_buffer_list.rs:61`](../../../../packages/project/src/git_store/diff_buffer_list.rs#L61)

#### Parameters

##### source

[`DiffBase`](../enums/DiffBase.md)

##### git_store

`Entity<GitStore>`

##### repo

`Option<Entity<Repository>>`

##### cx

`&Context<'_, Self>`

#### Returns

`Self`


***

### diff_base()

```rust
pub fn diff_base(&self) -> &DiffBase
```

Defined in: [`packages/project/src/git_store/diff_buffer_list.rs:114`](../../../../packages/project/src/git_store/diff_buffer_list.rs#L114)

#### Returns

`&DiffBase`


***

### set_repo()

```rust
pub fn set_repo(&self, repo: Option<Entity<Repository>>, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/git_store/diff_buffer_list.rs:118`](../../../../packages/project/src/git_store/diff_buffer_list.rs#L118)

#### Parameters

##### repo

`Option<Entity<Repository>>`

##### cx

`&Context<'_, Self>`


***

### set_diff_base()

```rust
pub fn set_diff_base(&self, diff_base: DiffBase, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/git_store/diff_buffer_list.rs:137`](../../../../packages/project/src/git_store/diff_buffer_list.rs#L137)

#### Parameters

##### diff_base

[`DiffBase`](../enums/DiffBase.md)

##### cx

`&Context<'_, Self>`


***

### handle_status_updates()

```rust
pub async fn handle_status_updates(this: WeakEntity<Self>, recv: Receiver<()>, cx: &AsyncApp)
```

Defined in: [`packages/project/src/git_store/diff_buffer_list.rs:153`](../../../../packages/project/src/git_store/diff_buffer_list.rs#L153)

#### Parameters

##### this

`WeakEntity<Self>`

##### recv

`Receiver<()>`

##### cx

`&AsyncApp`


***

### status_for_buffer_id()

```rust
pub fn status_for_buffer_id(&self, buffer_id: BufferId, cx: &App) -> Option<FileStatus>
```

Defined in: [`packages/project/src/git_store/diff_buffer_list.rs:175`](../../../../packages/project/src/git_store/diff_buffer_list.rs#L175)

#### Parameters

##### buffer_id

`BufferId`

##### cx

`&App`

#### Returns

`Option<FileStatus>`


***

### status_for_path()

```rust
pub fn status_for_path(&self, path: &RepoPath, cx: &App) -> Option<FileStatus>
```

Defined in: [`packages/project/src/git_store/diff_buffer_list.rs:185`](../../../../packages/project/src/git_store/diff_buffer_list.rs#L185)

#### Parameters

##### path

`&RepoPath`

##### cx

`&App`

#### Returns

`Option<FileStatus>`


***

### statuses_by_path()

```rust
pub fn statuses_by_path(&self) -> Option<SumTree<StatusEntry>>
```

Defined in: [`packages/project/src/git_store/diff_buffer_list.rs:207`](../../../../packages/project/src/git_store/diff_buffer_list.rs#L207)

#### Returns

`Option<SumTree<StatusEntry>>`


***

### base_oid_for_path()

```rust
pub fn base_oid_for_path(&self, path: &RepoPath) -> Option<Option<Oid>>
```

Defined in: [`packages/project/src/git_store/diff_buffer_list.rs:211`](../../../../packages/project/src/git_store/diff_buffer_list.rs#L211)

#### Parameters

##### path

`&RepoPath`

#### Returns

`Option<Option<Oid>>`


***

### is_tree_base_loading()

```rust
pub fn is_tree_base_loading(&self) -> bool
```

Defined in: [`packages/project/src/git_store/diff_buffer_list.rs:240`](../../../../packages/project/src/git_store/diff_buffer_list.rs#L240)

#### Returns

`bool`


***

### reload_tree_diff()

```rust
pub async fn reload_tree_diff(this: WeakEntity<Self>, cx: &AsyncApp) -> Result<()>
```

Defined in: [`packages/project/src/git_store/diff_buffer_list.rs:246`](../../../../packages/project/src/git_store/diff_buffer_list.rs#L246)

#### Parameters

##### this

`WeakEntity<Self>`

##### cx

`&AsyncApp`

#### Returns

`Result<()>`


***

### repo()

```rust
pub fn repo(&self) -> Option<&Entity<Repository>>
```

Defined in: [`packages/project/src/git_store/diff_buffer_list.rs:289`](../../../../packages/project/src/git_store/diff_buffer_list.rs#L289)

#### Returns

`Option<&Entity<Repository>>`


***

### load_buffers()

```rust
pub fn load_buffers(&self, cx: &Context<'_, Self>) -> Vec<DiffBuffer>
```

Defined in: [`packages/project/src/git_store/diff_buffer_list.rs:294`](../../../../packages/project/src/git_store/diff_buffer_list.rs#L294)

#### Parameters

##### cx

`&Context<'_, Self>`

#### Returns

`Vec<DiffBuffer>`

## Trait Implementations

- `impl Borrow for DiffBufferList`
- `impl BorrowMut for DiffBufferList`
- `impl Into for DiffBufferList`
- `impl From for DiffBufferList`
- `impl TryInto for DiffBufferList`
- `impl TryFrom for DiffBufferList`
- `impl Any for DiffBufferList`
- `impl VZip for DiffBufferList`
- `impl CastableFrom for DiffBufferList`
- `impl CastableFrom for DiffBufferList`
- `impl Read for DiffBufferList`
- `impl IntoEither for DiffBufferList`
- `impl ErasedDestructor for DiffBufferList`
- `impl Same for DiffBufferList`
- `impl Pointable for DiffBufferList`
- `impl Instrument for DiffBufferList`
- `impl WithSubscriber for DiffBufferList`
- `impl FromAngle for DiffBufferList`
- `impl IntoAngle for DiffBufferList`
- `impl IntoCam16Unclamped for DiffBufferList`
- `impl Cam16IntoUnclamped for DiffBufferList`
- `impl ArraysFrom for DiffBufferList`
- `impl ArraysInto for DiffBufferList`
- `impl ComponentsFrom for DiffBufferList`
- `impl TryComponentsInto for DiffBufferList`
- `impl UintsFrom for DiffBufferList`
- `impl UintsInto for DiffBufferList`
- `impl AdaptIntoUnclamped for DiffBufferList`
- `impl AdaptInto for DiffBufferList`
- `impl IntoColor for DiffBufferList`
- `impl IntoColorUnclamped for DiffBufferList`
- `impl TryIntoColor for DiffBufferList`
- `impl FromStimulus for DiffBufferList`
- `impl IntoStimulus for DiffBufferList`
- `impl EventEmitter for DiffBufferList`

## Auto Trait Implementations

`Freeze` `Unpin` `UnsafeUnpin`

