---
id: BookmarkEntry
title: BookmarkEntry
---

# Enum: BookmarkEntry

Defined in: [`packages/project/src/bookmark_store.rs:72`](../../../../packages/project/src/bookmark_store.rs#L72)

## Definition

```rust
pub enum BookmarkEntry
{
    Loaded(BufferBookmarks),
    Unloaded(Vec<SerializedBookmark>),
}
```

## Implementations

### is_empty()

```rust
pub fn is_empty(&self) -> bool
```

Defined in: [`packages/project/src/bookmark_store.rs:78`](../../../../packages/project/src/bookmark_store.rs#L78)

#### Returns

`bool`

## Trait Implementations

- `impl Borrow for BookmarkEntry`
- `impl BorrowMut for BookmarkEntry`
- `impl Into for BookmarkEntry`
- `impl From for BookmarkEntry`
- `impl TryInto for BookmarkEntry`
- `impl TryFrom for BookmarkEntry`
- `impl Any for BookmarkEntry`
- `impl VZip for BookmarkEntry`
- `impl CastableFrom for BookmarkEntry`
- `impl CastableFrom for BookmarkEntry`
- `impl Read for BookmarkEntry`
- `impl IntoEither for BookmarkEntry`
- `impl ErasedDestructor for BookmarkEntry`
- `impl Same for BookmarkEntry`
- `impl Pointable for BookmarkEntry`
- `impl Instrument for BookmarkEntry`
- `impl WithSubscriber for BookmarkEntry`
- `impl FromAngle for BookmarkEntry`
- `impl IntoAngle for BookmarkEntry`
- `impl IntoCam16Unclamped for BookmarkEntry`
- `impl Cam16IntoUnclamped for BookmarkEntry`
- `impl ArraysFrom for BookmarkEntry`
- `impl ArraysInto for BookmarkEntry`
- `impl ComponentsFrom for BookmarkEntry`
- `impl TryComponentsInto for BookmarkEntry`
- `impl UintsFrom for BookmarkEntry`
- `impl UintsInto for BookmarkEntry`
- `impl AdaptIntoUnclamped for BookmarkEntry`
- `impl AdaptInto for BookmarkEntry`
- `impl IntoColor for BookmarkEntry`
- `impl IntoColorUnclamped for BookmarkEntry`
- `impl TryIntoColor for BookmarkEntry`
- `impl FromStimulus for BookmarkEntry`
- `impl IntoStimulus for BookmarkEntry`
- `impl Debug for BookmarkEntry`

## Auto Trait Implementations

`Freeze` `Unpin` `UnsafeUnpin`

