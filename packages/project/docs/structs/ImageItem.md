---
id: ImageItem
title: ImageItem
---

# Struct: ImageItem

Defined in: [`packages/project/src/image_store.rs:104`](../../../../packages/project/src/image_store.rs#L104)

## Definition

```rust
pub struct ImageItem
{
    pub id: ImageId,
    pub file: Arc<File>,
    pub image: Arc<Image>,
    pub image_metadata: Option<ImageMetadata>,
}
```

_（存在非公开字段）_

## Implementations

### compute_metadata_from_bytes()

```rust
pub fn compute_metadata_from_bytes(image_bytes: &[u8]) -> Result<ImageMetadata>
```

Defined in: [`packages/project/src/image_store.rs:152`](../../../../packages/project/src/image_store.rs#L152)

#### Parameters

##### image_bytes

`&[u8]`

#### Returns

`Result<ImageMetadata>`


***

### load_image_metadata()

```rust
pub async fn load_image_metadata(image: Entity<ImageItem>, project: Entity<Project>, cx: &AsyncApp) -> Result<ImageMetadata>
```

Defined in: [`packages/project/src/image_store.rs:170`](../../../../packages/project/src/image_store.rs#L170)

#### Parameters

##### image

`Entity<ImageItem>`

##### project

`Entity<Project>`

##### cx

`&AsyncApp`

#### Returns

`Result<ImageMetadata>`


***

### project_path()

```rust
pub fn project_path(&self, cx: &App) -> ProjectPath
```

Defined in: [`packages/project/src/image_store.rs:188`](../../../../packages/project/src/image_store.rs#L188)

#### Parameters

##### cx

`&App`

#### Returns

[`ProjectPath`](ProjectPath.md)


***

### abs_path()

```rust
pub fn abs_path(&self, cx: &App) -> Option<PathBuf>
```

Defined in: [`packages/project/src/image_store.rs:195`](../../../../packages/project/src/image_store.rs#L195)

#### Parameters

##### cx

`&App`

#### Returns

`Option<PathBuf>`

## Trait Implementations

- `impl Borrow for ImageItem`
- `impl BorrowMut for ImageItem`
- `impl Into for ImageItem`
- `impl From for ImageItem`
- `impl TryInto for ImageItem`
- `impl TryFrom for ImageItem`
- `impl Any for ImageItem`
- `impl VZip for ImageItem`
- `impl CastableFrom for ImageItem`
- `impl CastableFrom for ImageItem`
- `impl Read for ImageItem`
- `impl IntoEither for ImageItem`
- `impl ErasedDestructor for ImageItem`
- `impl Same for ImageItem`
- `impl Pointable for ImageItem`
- `impl Instrument for ImageItem`
- `impl WithSubscriber for ImageItem`
- `impl FromAngle for ImageItem`
- `impl IntoAngle for ImageItem`
- `impl IntoCam16Unclamped for ImageItem`
- `impl Cam16IntoUnclamped for ImageItem`
- `impl ArraysFrom for ImageItem`
- `impl ArraysInto for ImageItem`
- `impl ComponentsFrom for ImageItem`
- `impl TryComponentsInto for ImageItem`
- `impl UintsFrom for ImageItem`
- `impl UintsInto for ImageItem`
- `impl AdaptIntoUnclamped for ImageItem`
- `impl AdaptInto for ImageItem`
- `impl IntoColor for ImageItem`
- `impl IntoColorUnclamped for ImageItem`
- `impl TryIntoColor for ImageItem`
- `impl FromStimulus for ImageItem`
- `impl IntoStimulus for ImageItem`
- `impl EventEmitter for ImageItem`
- `impl ProjectItem for ImageItem`

## Auto Trait Implementations

`Freeze` `Send` `Sync` `Unpin` `UnsafeUnpin`

