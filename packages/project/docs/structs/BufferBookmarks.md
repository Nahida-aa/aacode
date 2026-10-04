---
id: BufferBookmarks
title: BufferBookmarks
---

# Struct: BufferBookmarks

Defined in: [`packages/project/src/bookmark_store.rs:37`](../../../../packages/project/src/bookmark_store.rs#L37)

## Definition

```rust
pub struct BufferBookmarks
```

_（存在非公开字段）_

## Implementations

### new()

```rust
pub fn new(buffer: Entity<Buffer>, cx: &Context<'_, BookmarkStore>) -> Self
```

Defined in: [`packages/project/src/bookmark_store.rs:44`](../../../../packages/project/src/bookmark_store.rs#L44)

#### Parameters

##### buffer

`Entity<Buffer>`

##### cx

`&Context<'_, BookmarkStore>`

#### Returns

`Self`


***

### buffer()

```rust
pub fn buffer(&self) -> &Entity<Buffer>
```

Defined in: [`packages/project/src/bookmark_store.rs:62`](../../../../packages/project/src/bookmark_store.rs#L62)

#### Returns

`&Entity<Buffer>`


***

### bookmarks()

```rust
pub fn bookmarks(&self) -> &[Bookmark]
```

Defined in: [`packages/project/src/bookmark_store.rs:66`](../../../../packages/project/src/bookmark_store.rs#L66)

#### Returns

`&[Bookmark]`

## Trait Implementations

- `impl Borrow for BufferBookmarks`
- `impl BorrowMut for BufferBookmarks`
- `impl Into for BufferBookmarks`
- `impl From for BufferBookmarks`
- `impl TryInto for BufferBookmarks`
- `impl TryFrom for BufferBookmarks`
- `impl Any for BufferBookmarks`
- `impl VZip for BufferBookmarks`
- `impl CastableFrom for BufferBookmarks`
- `impl CastableFrom for BufferBookmarks`
- `impl Read for BufferBookmarks`
- `impl IntoEither for BufferBookmarks`
- `impl ErasedDestructor for BufferBookmarks`
- `impl Same for BufferBookmarks`
- `impl Pointable for BufferBookmarks`
- `impl Instrument for BufferBookmarks`
- `impl WithSubscriber for BufferBookmarks`
- `impl FromAngle for BufferBookmarks`
- `impl IntoAngle for BufferBookmarks`
- `impl IntoCam16Unclamped for BufferBookmarks`
- `impl Cam16IntoUnclamped for BufferBookmarks`
- `impl ArraysFrom for BufferBookmarks`
- `impl ArraysInto for BufferBookmarks`
- `impl ComponentsFrom for BufferBookmarks`
- `impl TryComponentsInto for BufferBookmarks`
- `impl UintsFrom for BufferBookmarks`
- `impl UintsInto for BufferBookmarks`
- `impl AdaptIntoUnclamped for BufferBookmarks`
- `impl AdaptInto for BufferBookmarks`
- `impl IntoColor for BufferBookmarks`
- `impl IntoColorUnclamped for BufferBookmarks`
- `impl TryIntoColor for BufferBookmarks`
- `impl FromStimulus for BufferBookmarks`
- `impl IntoStimulus for BufferBookmarks`
- `impl Debug for BufferBookmarks`

## Auto Trait Implementations

`Freeze` `Unpin` `UnsafeUnpin`

