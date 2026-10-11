---
id: Hover
title: Hover
---

# Struct: Hover

Defined in: [`packages/project/src/types/hover.rs:17`](../../../../packages/project/src/types/hover.rs#L17)

## Definition

```rust
pub struct Hover {
    pub contents: Vec<HoverBlock>,
    pub range: Option<Range<Anchor>>,
    pub language: Option<Arc<Language>>,
}
```

## Implementations

### is_empty()

```rust
pub fn is_empty(&self) -> bool
```

Defined in: [`packages/project/src/types/hover.rs:24`](../../../../packages/project/src/types/hover.rs#L24)

#### Returns

`bool`

## Trait Implementations

- `impl Borrow for Hover`
- `impl BorrowMut for Hover`
- `impl CloneToUninit for Hover`
- `impl Into for Hover`
- `impl From for Hover`
- `impl TryInto for Hover`
- `impl TryFrom for Hover`
- `impl Any for Hover`
- `impl ToOwned for Hover`
- `impl DynClone for Hover`
- `impl VZip for Hover`
- `impl CastableFrom for Hover`
- `impl CastableFrom for Hover`
- `impl Read for Hover`
- `impl IntoEither for Hover`
- `impl ErasedDestructor for Hover`
- `impl Same for Hover`
- `impl Pointable for Hover`
- `impl Instrument for Hover`
- `impl WithSubscriber for Hover`
- `impl FromAngle for Hover`
- `impl IntoAngle for Hover`
- `impl IntoCam16Unclamped for Hover`
- `impl Cam16IntoUnclamped for Hover`
- `impl ArraysFrom for Hover`
- `impl ArraysInto for Hover`
- `impl ComponentsFrom for Hover`
- `impl TryComponentsInto for Hover`
- `impl UintsFrom for Hover`
- `impl UintsInto for Hover`
- `impl AdaptIntoUnclamped for Hover`
- `impl AdaptInto for Hover`
- `impl IntoColor for Hover`
- `impl IntoColorUnclamped for Hover`
- `impl TryIntoColor for Hover`
- `impl FromStimulus for Hover`
- `impl IntoStimulus for Hover`
- `impl Debug for Hover`
- `impl Clone for Hover`

## Auto Trait Implementations

`Freeze` `Send` `Sync` `Unpin` `UnsafeUnpin`

