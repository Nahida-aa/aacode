---
id: ImageStore
title: ImageStore
---

# Struct: ImageStore

Defined in: [`packages/project/src/image_store.rs:340`](../../../../packages/project/src/image_store.rs#L340)

## Definition

```rust
pub struct ImageStore
```

_（存在非公开字段）_

## Implementations

### local()

```rust
pub fn local(worktree_store: Entity<WorktreeStore>, cx: &Context<'_, Self>) -> Self
```

Defined in: [`packages/project/src/image_store.rs:352`](../../../../packages/project/src/image_store.rs#L352)

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
pub fn remote(worktree_store: Entity<WorktreeStore>, upstream_client: AnyProtoClient, project_id: u64, cx: &Context<'_, Self>) -> Self
```

Defined in: [`packages/project/src/image_store.rs:378`](../../../../packages/project/src/image_store.rs#L378)

#### Parameters

##### worktree_store

`Entity<WorktreeStore>`

##### upstream_client

`AnyProtoClient`

##### project_id

`u64`

##### cx

`&Context<'_, Self>`

#### Returns

`Self`


***

### images()

```rust
pub fn images(&self) -> impl ? + ?
```

Defined in: [`packages/project/src/image_store.rs:398`](../../../../packages/project/src/image_store.rs#L398)

#### Returns

`impl ? + ?`


***

### get()

```rust
pub fn get(&self, image_id: ImageId) -> Option<Entity<ImageItem>>
```

Defined in: [`packages/project/src/image_store.rs:404`](../../../../packages/project/src/image_store.rs#L404)

#### Parameters

##### image_id

[`ImageId`](ImageId.md)

#### Returns

`Option<Entity<ImageItem>>`


***

### get_by_path()

```rust
pub fn get_by_path(&self, path: &ProjectPath, cx: &App) -> Option<Entity<ImageItem>>
```

Defined in: [`packages/project/src/image_store.rs:410`](../../../../packages/project/src/image_store.rs#L410)

#### Parameters

##### path

`&ProjectPath`

##### cx

`&App`

#### Returns

`Option<Entity<ImageItem>>`


***

### open_image()

```rust
pub fn open_image(&self, project_path: ProjectPath, cx: &Context<'_, Self>) -> Task<Result<Entity<ImageItem>>>
```

Defined in: [`packages/project/src/image_store.rs:415`](../../../../packages/project/src/image_store.rs#L415)

#### Parameters

##### project_path

[`ProjectPath`](ProjectPath.md)

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<Entity<ImageItem>>>`


***

### wait_for_loading_image()

```rust
pub async fn wait_for_loading_image(receiver: Receiver<Option<Result<Entity<ImageItem>, Arc<Error>>>>) -> Result<Entity<ImageItem>, Arc<Error>>
```

Defined in: [`packages/project/src/image_store.rs:469`](../../../../packages/project/src/image_store.rs#L469)

#### Parameters

##### receiver

`Receiver<Option<Result<Entity<ImageItem>, Arc<Error>>>>`

#### Returns

`Result<Entity<ImageItem>, Arc<Error>>`


***

### reload_images()

```rust
pub fn reload_images(&self, images: HashSet<Entity<ImageItem>>, cx: &Context<'_, ImageStore>) -> Task<Result<()>>
```

Defined in: [`packages/project/src/image_store.rs:485`](../../../../packages/project/src/image_store.rs#L485)

#### Parameters

##### images

`HashSet<Entity<ImageItem>>`

##### cx

`&Context<'_, ImageStore>`

#### Returns

`Task<Result<()>>`


***

### handle_create_image_for_peer()

```rust
pub fn handle_create_image_for_peer(&self, envelope: TypedEnvelope<CreateImageForPeer>, cx: &Context<'_, Self>) -> Result<()>
```

Defined in: [`packages/project/src/image_store.rs:520`](../../../../packages/project/src/image_store.rs#L520)

#### Parameters

##### envelope

`TypedEnvelope<CreateImageForPeer>`

##### cx

`&Context<'_, Self>`

#### Returns

`Result<()>`

## Trait Implementations

- `impl Borrow for ImageStore`
- `impl BorrowMut for ImageStore`
- `impl Into for ImageStore`
- `impl From for ImageStore`
- `impl TryInto for ImageStore`
- `impl TryFrom for ImageStore`
- `impl Any for ImageStore`
- `impl VZip for ImageStore`
- `impl CastableFrom for ImageStore`
- `impl CastableFrom for ImageStore`
- `impl Read for ImageStore`
- `impl IntoEither for ImageStore`
- `impl ErasedDestructor for ImageStore`
- `impl Same for ImageStore`
- `impl Pointable for ImageStore`
- `impl Instrument for ImageStore`
- `impl WithSubscriber for ImageStore`
- `impl FromAngle for ImageStore`
- `impl IntoAngle for ImageStore`
- `impl IntoCam16Unclamped for ImageStore`
- `impl Cam16IntoUnclamped for ImageStore`
- `impl ArraysFrom for ImageStore`
- `impl ArraysInto for ImageStore`
- `impl ComponentsFrom for ImageStore`
- `impl TryComponentsInto for ImageStore`
- `impl UintsFrom for ImageStore`
- `impl UintsInto for ImageStore`
- `impl AdaptIntoUnclamped for ImageStore`
- `impl AdaptInto for ImageStore`
- `impl IntoColor for ImageStore`
- `impl IntoColorUnclamped for ImageStore`
- `impl TryIntoColor for ImageStore`
- `impl FromStimulus for ImageStore`
- `impl IntoStimulus for ImageStore`
- `impl EventEmitter for ImageStore`

## Auto Trait Implementations

`Freeze` `Unpin` `UnsafeUnpin`

