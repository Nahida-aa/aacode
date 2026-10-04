---
id: BufferStore
title: BufferStore
---

# Struct: BufferStore

Defined in: [`packages/project/src/buffer_store.rs:34`](../../../../packages/project/src/buffer_store.rs#L34)

A set of open buffers.

## Definition

```rust
pub struct BufferStore
```

_（存在非公开字段）_

## Implementations

### init()

```rust
pub fn init(client: &AnyProtoClient)
```

Defined in: [`packages/project/src/buffer_store.rs:828`](../../../../packages/project/src/buffer_store.rs#L828)

#### Parameters

##### client

`&AnyProtoClient`


***

### local()

```rust
pub fn local(worktree_store: Entity<WorktreeStore>, cx: &Context<'_, Self>) -> Self
```

Defined in: [`packages/project/src/buffer_store.rs:837`](../../../../packages/project/src/buffer_store.rs#L837)

Creates a buffer store, optionally retaining its buffers.

#### Parameters

##### worktree_store

`Entity<WorktreeStore>`

##### cx

`&Context<'_, Self>`

#### Returns

`Self`


***

### remote()

```rust
pub fn remote(worktree_store: Entity<WorktreeStore>, upstream_client: AnyProtoClient, remote_id: u64, _cx: &Context<'_, Self>) -> Self
```

Defined in: [`packages/project/src/buffer_store.rs:860`](../../../../packages/project/src/buffer_store.rs#L860)

#### Parameters

##### worktree_store

`Entity<WorktreeStore>`

##### upstream_client

`AnyProtoClient`

##### remote_id

`u64`

##### _cx

`&Context<'_, Self>`

#### Returns

`Self`


***

### open_buffer()

```rust
pub fn open_buffer(&self, project_path: ProjectPath, cx: &Context<'_, Self>) -> Task<Result<Entity<Buffer>>>
```

Defined in: [`packages/project/src/buffer_store.rs:908`](../../../../packages/project/src/buffer_store.rs#L908)

#### Parameters

##### project_path

[`ProjectPath`](ProjectPath.md)

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<Entity<Buffer>>>`


***

### create_buffer()

```rust
pub fn create_buffer(&self, language: Option<Arc<Language>>, project_searchable: bool, cx: &Context<'_, Self>) -> Task<Result<Entity<Buffer>>>
```

Defined in: [`packages/project/src/buffer_store.rs:962`](../../../../packages/project/src/buffer_store.rs#L962)

#### Parameters

##### language

`Option<Arc<Language>>`

##### project_searchable

`bool`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<Entity<Buffer>>>`


***

### save_buffer()

```rust
pub fn save_buffer(&self, buffer: Entity<Buffer>, cx: &Context<'_, Self>) -> Task<Result<()>>
```

Defined in: [`packages/project/src/buffer_store.rs:974`](../../../../packages/project/src/buffer_store.rs#L974)

#### Parameters

##### buffer

`Entity<Buffer>`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<()>>`


***

### save_buffer_as()

```rust
pub fn save_buffer_as(&self, buffer: Entity<Buffer>, path: ProjectPath, cx: &Context<'_, Self>) -> Task<Result<()>>
```

Defined in: [`packages/project/src/buffer_store.rs:985`](../../../../packages/project/src/buffer_store.rs#L985)

#### Parameters

##### buffer

`Entity<Buffer>`

##### path

[`ProjectPath`](ProjectPath.md)

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<()>>`


***

### buffers()

```rust
pub fn buffers(&self) -> impl ? + ?
```

Defined in: [`packages/project/src/buffer_store.rs:1067`](../../../../packages/project/src/buffer_store.rs#L1067)

#### Returns

`impl ? + ?`


***

### loading_buffers()

```rust
pub fn loading_buffers(&self) -> impl ?
```

Defined in: [`packages/project/src/buffer_store.rs:1077`](../../../../packages/project/src/buffer_store.rs#L1077)

#### Returns

`impl ?`


***

### buffer_id_for_project_path()

```rust
pub fn buffer_id_for_project_path(&self, project_path: &ProjectPath) -> Option<&BufferId>
```

Defined in: [`packages/project/src/buffer_store.rs:1094`](../../../../packages/project/src/buffer_store.rs#L1094)

#### Parameters

##### project_path

`&ProjectPath`

#### Returns

`Option<&BufferId>`


***

### get_by_path()

```rust
pub fn get_by_path(&self, path: &ProjectPath) -> Option<Entity<Buffer>>
```

Defined in: [`packages/project/src/buffer_store.rs:1098`](../../../../packages/project/src/buffer_store.rs#L1098)

#### Parameters

##### path

`&ProjectPath`

#### Returns

`Option<Entity<Buffer>>`


***

### get()

```rust
pub fn get(&self, buffer_id: BufferId) -> Option<Entity<Buffer>>
```

Defined in: [`packages/project/src/buffer_store.rs:1104`](../../../../packages/project/src/buffer_store.rs#L1104)

#### Parameters

##### buffer_id

`BufferId`

#### Returns

`Option<Entity<Buffer>>`


***

### get_existing()

```rust
pub fn get_existing(&self, buffer_id: BufferId) -> Result<Entity<Buffer>>
```

Defined in: [`packages/project/src/buffer_store.rs:1108`](../../../../packages/project/src/buffer_store.rs#L1108)

#### Parameters

##### buffer_id

`BufferId`

#### Returns

`Result<Entity<Buffer>>`


***

### get_possibly_incomplete()

```rust
pub fn get_possibly_incomplete(&self, buffer_id: BufferId) -> Option<Entity<Buffer>>
```

Defined in: [`packages/project/src/buffer_store.rs:1113`](../../../../packages/project/src/buffer_store.rs#L1113)

#### Parameters

##### buffer_id

`BufferId`

#### Returns

`Option<Entity<Buffer>>`


***

### buffer_version_info()

```rust
pub fn buffer_version_info(&self, cx: &App) -> (Vec<BufferVersion>, Vec<BufferId>)
```

Defined in: [`packages/project/src/buffer_store.rs:1120`](../../../../packages/project/src/buffer_store.rs#L1120)

#### Parameters

##### cx

`&App`

#### Returns

`(Vec<BufferVersion>, Vec<BufferId>)`


***

### disconnected_from_host()

```rust
pub fn disconnected_from_host(&self, cx: &App)
```

Defined in: [`packages/project/src/buffer_store.rs:1138`](../../../../packages/project/src/buffer_store.rs#L1138)

#### Parameters

##### cx

`&App`


***

### shared()

```rust
pub fn shared(&self, remote_id: u64, downstream_client: AnyProtoClient, _cx: &App)
```

Defined in: [`packages/project/src/buffer_store.rs:1158`](../../../../packages/project/src/buffer_store.rs#L1158)

#### Parameters

##### remote_id

`u64`

##### downstream_client

`AnyProtoClient`

##### _cx

`&App`


***

### unshared()

```rust
pub fn unshared(&self, _cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/buffer_store.rs:1162`](../../../../packages/project/src/buffer_store.rs#L1162)

#### Parameters

##### _cx

`&Context<'_, Self>`


***

### discard_incomplete()

```rust
pub fn discard_incomplete(&self)
```

Defined in: [`packages/project/src/buffer_store.rs:1167`](../../../../packages/project/src/buffer_store.rs#L1167)


***

### handle_update_buffer()

```rust
pub async fn handle_update_buffer(this: Entity<Self>, envelope: TypedEnvelope<UpdateBuffer>, cx: AsyncApp) -> Result<Ack>
```

Defined in: [`packages/project/src/buffer_store.rs:1231`](../../../../packages/project/src/buffer_store.rs#L1231)

#### Parameters

##### this

`Entity<Self>`

##### envelope

`TypedEnvelope<UpdateBuffer>`

##### cx

`AsyncApp`

#### Returns

`Result<Ack>`


***

### register_shared_lsp_handle()

```rust
pub fn register_shared_lsp_handle(&self, peer_id: PeerId, buffer_id: BufferId, handle: OpenLspBufferHandle)
```

Defined in: [`packages/project/src/buffer_store.rs:1261`](../../../../packages/project/src/buffer_store.rs#L1261)

#### Parameters

##### peer_id

`PeerId`

##### buffer_id

`BufferId`

##### handle

[`OpenLspBufferHandle`](OpenLspBufferHandle.md)


***

### handle_synchronize_buffers()

```rust
pub fn handle_synchronize_buffers(&self, envelope: TypedEnvelope<SynchronizeBuffers>, cx: &Context<'_, Self>, client: Arc<Client>) -> Result<SynchronizeBuffersResponse>
```

Defined in: [`packages/project/src/buffer_store.rs:1276`](../../../../packages/project/src/buffer_store.rs#L1276)

#### Parameters

##### envelope

`TypedEnvelope<SynchronizeBuffers>`

##### cx

`&Context<'_, Self>`

##### client

`Arc<Client>`

#### Returns

`Result<SynchronizeBuffersResponse>`


***

### handle_create_buffer_for_peer()

```rust
pub fn handle_create_buffer_for_peer(&self, envelope: TypedEnvelope<CreateBufferForPeer>, replica_id: ReplicaId, capability: Capability, cx: &Context<'_, Self>) -> Result<()>
```

Defined in: [`packages/project/src/buffer_store.rs:1364`](../../../../packages/project/src/buffer_store.rs#L1364)

#### Parameters

##### envelope

`TypedEnvelope<CreateBufferForPeer>`

##### replica_id

`ReplicaId`

##### capability

`Capability`

##### cx

`&Context<'_, Self>`

#### Returns

`Result<()>`


***

### handle_update_buffer_file()

```rust
pub async fn handle_update_buffer_file(this: Entity<Self>, envelope: TypedEnvelope<UpdateBufferFile>, cx: AsyncApp) -> Result<()>
```

Defined in: [`packages/project/src/buffer_store.rs:1384`](../../../../packages/project/src/buffer_store.rs#L1384)

#### Parameters

##### this

`Entity<Self>`

##### envelope

`TypedEnvelope<UpdateBufferFile>`

##### cx

`AsyncApp`

#### Returns

`Result<()>`


***

### handle_save_buffer()

```rust
pub async fn handle_save_buffer(this: Entity<Self>, envelope: TypedEnvelope<SaveBuffer>, cx: AsyncApp) -> Result<BufferSaved>
```

Defined in: [`packages/project/src/buffer_store.rs:1444`](../../../../packages/project/src/buffer_store.rs#L1444)

#### Parameters

##### this

`Entity<Self>`

##### envelope

`TypedEnvelope<SaveBuffer>`

##### cx

`AsyncApp`

#### Returns

`Result<BufferSaved>`


***

### handle_close_buffer()

```rust
pub async fn handle_close_buffer(this: Entity<Self>, envelope: TypedEnvelope<CloseBuffer>, cx: AsyncApp) -> Result<()>
```

Defined in: [`packages/project/src/buffer_store.rs:1486`](../../../../packages/project/src/buffer_store.rs#L1486)

#### Parameters

##### this

`Entity<Self>`

##### envelope

`TypedEnvelope<CloseBuffer>`

##### cx

`AsyncApp`

#### Returns

`Result<()>`


***

### handle_buffer_saved()

```rust
pub async fn handle_buffer_saved(this: Entity<Self>, envelope: TypedEnvelope<BufferSaved>, cx: AsyncApp) -> Result<()>
```

Defined in: [`packages/project/src/buffer_store.rs:1512`](../../../../packages/project/src/buffer_store.rs#L1512)

#### Parameters

##### this

`Entity<Self>`

##### envelope

`TypedEnvelope<BufferSaved>`

##### cx

`AsyncApp`

#### Returns

`Result<()>`


***

### handle_buffer_reloaded()

```rust
pub async fn handle_buffer_reloaded(this: Entity<Self>, envelope: TypedEnvelope<BufferReloaded>, cx: AsyncApp) -> Result<()>
```

Defined in: [`packages/project/src/buffer_store.rs:1541`](../../../../packages/project/src/buffer_store.rs#L1541)

#### Parameters

##### this

`Entity<Self>`

##### envelope

`TypedEnvelope<BufferReloaded>`

##### cx

`AsyncApp`

#### Returns

`Result<()>`


***

### reload_buffers()

```rust
pub fn reload_buffers(&self, buffers: HashSet<Entity<Buffer>>, push_to_history: bool, cx: &Context<'_, Self>) -> Task<Result<ProjectTransaction>>
```

Defined in: [`packages/project/src/buffer_store.rs:1576`](../../../../packages/project/src/buffer_store.rs#L1576)

#### Parameters

##### buffers

`HashSet<Entity<Buffer>>`

##### push_to_history

`bool`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<ProjectTransaction>>`


***

### create_buffer_for_peer()

```rust
pub fn create_buffer_for_peer(&self, buffer: &Entity<Buffer>, peer_id: PeerId, cx: &Context<'_, Self>) -> Task<Result<()>>
```

Defined in: [`packages/project/src/buffer_store.rs:1615`](../../../../packages/project/src/buffer_store.rs#L1615)

#### Parameters

##### buffer

`&Entity<Buffer>`

##### peer_id

`PeerId`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<()>>`


***

### forget_shared_buffers()

```rust
pub fn forget_shared_buffers(&self)
```

Defined in: [`packages/project/src/buffer_store.rs:1680`](../../../../packages/project/src/buffer_store.rs#L1680)


***

### forget_shared_buffers_for()

```rust
pub fn forget_shared_buffers_for(&self, peer_id: &PeerId)
```

Defined in: [`packages/project/src/buffer_store.rs:1684`](../../../../packages/project/src/buffer_store.rs#L1684)

#### Parameters

##### peer_id

`&PeerId`


***

### is_shared()

```rust
pub fn is_shared(&self, buffer_id: BufferId, cx: &App) -> bool
```

Defined in: [`packages/project/src/buffer_store.rs:1688`](../../../../packages/project/src/buffer_store.rs#L1688)

#### Parameters

##### buffer_id

`BufferId`

##### cx

`&App`

#### Returns

`bool`


***

### update_peer_id()

```rust
pub fn update_peer_id(&self, old_peer_id: &PeerId, new_peer_id: PeerId)
```

Defined in: [`packages/project/src/buffer_store.rs:1700`](../../../../packages/project/src/buffer_store.rs#L1700)

#### Parameters

##### old_peer_id

`&PeerId`

##### new_peer_id

`PeerId`


***

### has_shared_buffers()

```rust
pub fn has_shared_buffers(&self) -> bool
```

Defined in: [`packages/project/src/buffer_store.rs:1706`](../../../../packages/project/src/buffer_store.rs#L1706)

#### Returns

`bool`


***

### create_local_buffer()

```rust
pub fn create_local_buffer(&self, text: &str, language: Option<Arc<Language>>, project_searchable: bool, cx: &Context<'_, Self>) -> Entity<Buffer>
```

Defined in: [`packages/project/src/buffer_store.rs:1710`](../../../../packages/project/src/buffer_store.rs#L1710)

#### Parameters

##### text

`&str`

##### language

`Option<Arc<Language>>`

##### project_searchable

`bool`

##### cx

`&Context<'_, Self>`

#### Returns

`Entity<Buffer>`


***

### deserialize_project_transaction()

```rust
pub fn deserialize_project_transaction(&self, message: ProjectTransaction, push_to_history: bool, cx: &Context<'_, Self>) -> Task<Result<ProjectTransaction>>
```

Defined in: [`packages/project/src/buffer_store.rs:1749`](../../../../packages/project/src/buffer_store.rs#L1749)

#### Parameters

##### message

`ProjectTransaction`

##### push_to_history

`bool`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<ProjectTransaction>>`


***

### wait_for_remote_buffer()

```rust
pub fn wait_for_remote_buffer(&self, id: BufferId, cx: &Context<'_, BufferStore>) -> Task<Result<Entity<Buffer>>>
```

Defined in: [`packages/project/src/buffer_store.rs:1763`](../../../../packages/project/src/buffer_store.rs#L1763)

#### Parameters

##### id

`BufferId`

##### cx

`&Context<'_, BufferStore>`

#### Returns

`Task<Result<Entity<Buffer>>>`


***

### serialize_project_transaction_for_peer()

```rust
pub fn serialize_project_transaction_for_peer(&self, project_transaction: ProjectTransaction, peer_id: PeerId, cx: &Context<'_, Self>) -> ProjectTransaction
```

Defined in: [`packages/project/src/buffer_store.rs:1776`](../../../../packages/project/src/buffer_store.rs#L1776)

#### Parameters

##### project_transaction

[`ProjectTransaction`](ProjectTransaction.md)

##### peer_id

`PeerId`

##### cx

`&Context<'_, Self>`

#### Returns

`ProjectTransaction`


***

### register_ongoing_project_search()

```rust
pub fn register_ongoing_project_search(&self, id: (PeerId, u64), search: Task<Result<()>>)
```

Defined in: [`packages/project/src/buffer_store.rs:1809`](../../../../packages/project/src/buffer_store.rs#L1809)

#### Parameters

##### id

`(PeerId, u64)`

##### search

`Task<Result<()>>`


***

### handle_find_search_candidates_cancel()

```rust
pub async fn handle_find_search_candidates_cancel(this: Entity<Self>, envelope: TypedEnvelope<FindSearchCandidatesCancelled>, cx: AsyncApp) -> Result<()>
```

Defined in: [`packages/project/src/buffer_store.rs:1818`](../../../../packages/project/src/buffer_store.rs#L1818)

#### Parameters

##### this

`Entity<Self>`

##### envelope

`TypedEnvelope<FindSearchCandidatesCancelled>`

##### cx

`AsyncApp`

#### Returns

`Result<()>`

## Trait Implementations

- `impl Borrow for BufferStore`
- `impl BorrowMut for BufferStore`
- `impl Into for BufferStore`
- `impl From for BufferStore`
- `impl TryInto for BufferStore`
- `impl TryFrom for BufferStore`
- `impl Any for BufferStore`
- `impl VZip for BufferStore`
- `impl CastableFrom for BufferStore`
- `impl CastableFrom for BufferStore`
- `impl Read for BufferStore`
- `impl IntoEither for BufferStore`
- `impl ErasedDestructor for BufferStore`
- `impl Same for BufferStore`
- `impl Pointable for BufferStore`
- `impl Instrument for BufferStore`
- `impl WithSubscriber for BufferStore`
- `impl FromAngle for BufferStore`
- `impl IntoAngle for BufferStore`
- `impl IntoCam16Unclamped for BufferStore`
- `impl Cam16IntoUnclamped for BufferStore`
- `impl ArraysFrom for BufferStore`
- `impl ArraysInto for BufferStore`
- `impl ComponentsFrom for BufferStore`
- `impl TryComponentsInto for BufferStore`
- `impl UintsFrom for BufferStore`
- `impl UintsInto for BufferStore`
- `impl AdaptIntoUnclamped for BufferStore`
- `impl AdaptInto for BufferStore`
- `impl IntoColor for BufferStore`
- `impl IntoColorUnclamped for BufferStore`
- `impl TryIntoColor for BufferStore`
- `impl FromStimulus for BufferStore`
- `impl IntoStimulus for BufferStore`
- `impl EventEmitter for BufferStore`

## Auto Trait Implementations

`Freeze` `Unpin` `UnsafeUnpin`

