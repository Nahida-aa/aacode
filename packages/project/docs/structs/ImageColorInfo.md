---
id: ImageColorInfo
title: ImageColorInfo
---

# Struct: ImageColorInfo

Defined in: [`packages/project/src/image_store.rs:70`](../../../../packages/project/src/image_store.rs#L70)

## Definition

```rust
pub struct ImageColorInfo {
    pub channels: u8,
    pub bits_per_channel: u8,
}
```

## Implementations

### from_color_type()

```rust
pub fn from_color_type<impl Into<ExtendedColorType>: Into>(color_type: impl ?) -> Option<Self>
```

Defined in: [`packages/project/src/image_store.rs:76`](../../../../packages/project/src/image_store.rs#L76)

#### Parameters

##### color_type

`impl ?`

#### Returns

`Option<Self>`


***

### bits_per_pixel()

```rust
pub const fn bits_per_pixel(&self) -> u8
```

Defined in: [`packages/project/src/image_store.rs:99`](../../../../packages/project/src/image_store.rs#L99)

#### Returns

`u8`

## Trait Implementations

- `impl Borrow for ImageColorInfo`
- `impl BorrowMut for ImageColorInfo`
- `impl CloneToUninit for ImageColorInfo`
- `impl Into for ImageColorInfo`
- `impl From for ImageColorInfo`
- `impl TryInto for ImageColorInfo`
- `impl TryFrom for ImageColorInfo`
- `impl Any for ImageColorInfo`
- `impl ToOwned for ImageColorInfo`
- `impl DynClone for ImageColorInfo`
- `impl VZip for ImageColorInfo`
- `impl CastableFrom for ImageColorInfo`
- `impl CastableFrom for ImageColorInfo`
- `impl Read for ImageColorInfo`
- `impl IntoEither for ImageColorInfo`
- `impl ErasedDestructor for ImageColorInfo`
- `impl Same for ImageColorInfo`
- `impl Pointable for ImageColorInfo`
- `impl Instrument for ImageColorInfo`
- `impl WithSubscriber for ImageColorInfo`
- `impl FromAngle for ImageColorInfo`
- `impl IntoAngle for ImageColorInfo`
- `impl IntoCam16Unclamped for ImageColorInfo`
- `impl Cam16IntoUnclamped for ImageColorInfo`
- `impl ArraysFrom for ImageColorInfo`
- `impl ArraysInto for ImageColorInfo`
- `impl ComponentsFrom for ImageColorInfo`
- `impl TryComponentsInto for ImageColorInfo`
- `impl UintsFrom for ImageColorInfo`
- `impl UintsInto for ImageColorInfo`
- `impl AdaptIntoUnclamped for ImageColorInfo`
- `impl AdaptInto for ImageColorInfo`
- `impl IntoColor for ImageColorInfo`
- `impl IntoColorUnclamped for ImageColorInfo`
- `impl TryIntoColor for ImageColorInfo`
- `impl FromStimulus for ImageColorInfo`
- `impl IntoStimulus for ImageColorInfo`
- `impl ResetDiscriminant for ImageColorInfo`
- `impl Debug for ImageColorInfo`
- `impl Clone for ImageColorInfo`
- `impl Copy for ImageColorInfo`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

