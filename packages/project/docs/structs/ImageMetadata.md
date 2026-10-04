---
id: ImageMetadata
title: ImageMetadata
---

# Struct: ImageMetadata

Defined in: [`packages/project/src/image_store.rs:61`](../../../../packages/project/src/image_store.rs#L61)

## Definition

```rust
pub struct ImageMetadata {
    pub width: u32,
    pub height: u32,
    pub file_size: u64,
    pub colors: Option<ImageColorInfo>,
    pub format: ImageFormat,
}
```

## Trait Implementations

- `impl Borrow for ImageMetadata`
- `impl BorrowMut for ImageMetadata`
- `impl CloneToUninit for ImageMetadata`
- `impl Into for ImageMetadata`
- `impl From for ImageMetadata`
- `impl TryInto for ImageMetadata`
- `impl TryFrom for ImageMetadata`
- `impl Any for ImageMetadata`
- `impl ToOwned for ImageMetadata`
- `impl DynClone for ImageMetadata`
- `impl VZip for ImageMetadata`
- `impl CastableFrom for ImageMetadata`
- `impl CastableFrom for ImageMetadata`
- `impl Read for ImageMetadata`
- `impl IntoEither for ImageMetadata`
- `impl ErasedDestructor for ImageMetadata`
- `impl Same for ImageMetadata`
- `impl Pointable for ImageMetadata`
- `impl Instrument for ImageMetadata`
- `impl WithSubscriber for ImageMetadata`
- `impl FromAngle for ImageMetadata`
- `impl IntoAngle for ImageMetadata`
- `impl IntoCam16Unclamped for ImageMetadata`
- `impl Cam16IntoUnclamped for ImageMetadata`
- `impl ArraysFrom for ImageMetadata`
- `impl ArraysInto for ImageMetadata`
- `impl ComponentsFrom for ImageMetadata`
- `impl TryComponentsInto for ImageMetadata`
- `impl UintsFrom for ImageMetadata`
- `impl UintsInto for ImageMetadata`
- `impl AdaptIntoUnclamped for ImageMetadata`
- `impl AdaptInto for ImageMetadata`
- `impl IntoColor for ImageMetadata`
- `impl IntoColorUnclamped for ImageMetadata`
- `impl TryIntoColor for ImageMetadata`
- `impl FromStimulus for ImageMetadata`
- `impl IntoStimulus for ImageMetadata`
- `impl ResetDiscriminant for ImageMetadata`
- `impl Debug for ImageMetadata`
- `impl Clone for ImageMetadata`
- `impl Copy for ImageMetadata`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

