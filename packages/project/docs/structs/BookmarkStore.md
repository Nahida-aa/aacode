---
id: BookmarkStore
title: BookmarkStore
---

# Struct: BookmarkStore

Defined in: [`packages/project/src/bookmark_store.rs:93`](../../../../packages/project/src/bookmark_store.rs#L93)

## Definition

```rust
pub struct BookmarkStore
```

_（存在非公开字段）_

## Implementations

### new()

```rust
pub fn new(worktree_store: Entity<WorktreeStore>, buffer_store: Entity<BufferStore>, cx: &Context<'_, Self>) -> Self
```

Defined in: [`packages/project/src/bookmark_store.rs:104`](../../../../packages/project/src/bookmark_store.rs#L104)

#### Parameters

##### worktree_store

`Entity<WorktreeStore>`

##### buffer_store

`Entity<BufferStore>`

##### cx

`&Context<'_, Self>`

#### Returns

`Self`


***

### forget_failed_paths()

```rust
pub fn forget_failed_paths(&self, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/bookmark_store.rs:188`](../../../../packages/project/src/bookmark_store.rs#L188)

#### Parameters

##### cx

`&Context<'_, Self>`


***

### is_empty()

```rust
pub fn is_empty(&self) -> bool
```

Defined in: [`packages/project/src/bookmark_store.rs:196`](../../../../packages/project/src/bookmark_store.rs#L196)

#### Returns

`bool`


***

### load_serialized_bookmarks()

```rust
pub fn load_serialized_bookmarks(&self, bookmark_rows: BTreeMap<Arc<Path>, Vec<SerializedBookmark>>, cx: &Context<'_, Self>) -> Task<Result<()>>
```

Defined in: [`packages/project/src/bookmark_store.rs:200`](../../../../packages/project/src/bookmark_store.rs#L200)

#### Parameters

##### bookmark_rows

`BTreeMap<Arc<Path>, Vec<SerializedBookmark>>`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<()>>`


***

### abs_path_from_buffer()

```rust
pub fn abs_path_from_buffer(buffer: &Entity<Buffer>, cx: &App) -> Option<Arc<Path>>
```

Defined in: [`packages/project/src/bookmark_store.rs:277`](../../../../packages/project/src/bookmark_store.rs#L277)

#### Parameters

##### buffer

`&Entity<Buffer>`

##### cx

`&App`

#### Returns

`Option<Arc<Path>>`


***

### toggle_bookmark()

```rust
pub fn toggle_bookmark(&self, buffer: Entity<Buffer>, anchor: Anchor, label: String, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/bookmark_store.rs:286`](../../../../packages/project/src/bookmark_store.rs#L286)

Toggle a bookmark at the given anchor in the buffer.
If a bookmark already exists on the same row, it will be removed.
Otherwise, a new bookmark will be added with the given label.

#### Parameters

##### buffer

`Entity<Buffer>`

##### anchor

`Anchor`

##### label

`String`

##### cx

`&Context<'_, Self>`


***

### find_bookmark()

```rust
pub fn find_bookmark(&self, buffer: &Entity<Buffer>, anchor: Anchor, cx: &Context<'_, Self>) -> Option<&Bookmark>
```

Defined in: [`packages/project/src/bookmark_store.rs:327`](../../../../packages/project/src/bookmark_store.rs#L327)

#### Parameters

##### buffer

`&Entity<Buffer>`

##### anchor

`Anchor`

##### cx

`&Context<'_, Self>`

#### Returns

`Option<&Bookmark>`


***

### edit_bookmark()

```rust
pub fn edit_bookmark(&self, buffer: &Entity<Buffer>, anchor: Anchor, label: String, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/bookmark_store.rs:349`](../../../../packages/project/src/bookmark_store.rs#L349)

#### Parameters

##### buffer

`&Entity<Buffer>`

##### anchor

`Anchor`

##### label

`String`

##### cx

`&Context<'_, Self>`


***

### bookmarks_for_buffer()

```rust
pub fn bookmarks_for_buffer(&self, buffer: Entity<Buffer>, range: Range<Anchor>, buffer_snapshot: &BufferSnapshot, cx: &Context<'_, Self>) -> Vec<Bookmark>
```

Defined in: [`packages/project/src/bookmark_store.rs:384`](../../../../packages/project/src/bookmark_store.rs#L384)

Returns the bookmarks for a given buffer within an optional range.
Only returns bookmarks that have been resolved to anchors (loaded).
Unloaded bookmarks for the given buffer will be resolved first.

#### Parameters

##### buffer

`Entity<Buffer>`

##### range

`Range<Anchor>`

##### buffer_snapshot

`&BufferSnapshot`

##### cx

`&Context<'_, Self>`

#### Returns

`Vec<Bookmark>`


***

### all_serialized_bookmarks()

```rust
pub fn all_serialized_bookmarks(&self, cx: &App) -> BTreeMap<Arc<Path>, Vec<SerializedBookmark>>
```

Defined in: [`packages/project/src/bookmark_store.rs:469`](../../../../packages/project/src/bookmark_store.rs#L469)

#### Parameters

##### cx

`&App`

#### Returns

`BTreeMap<Arc<Path>, Vec<SerializedBookmark>>`


***

### all_bookmark_locations()

```rust
pub async fn all_bookmark_locations<impl AppContext + Clone: AppContext + Clone>(this: Entity<BookmarkStore>, cx: &impl ? + ?) -> Result<HashMap<Entity<Buffer>, Vec<Range<Point>>>>
```

Defined in: [`packages/project/src/bookmark_store.rs:510`](../../../../packages/project/src/bookmark_store.rs#L510)

#### Parameters

##### this

`Entity<BookmarkStore>`

##### cx

`&impl ? + ?`

#### Returns

`Result<HashMap<Entity<Buffer>, Vec<Range<Point>>>>`


***

### clear_bookmarks()

```rust
pub fn clear_bookmarks(&self, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/bookmark_store.rs:595`](../../../../packages/project/src/bookmark_store.rs#L595)

#### Parameters

##### cx

`&Context<'_, Self>`

## Trait Implementations

- `impl Borrow for BookmarkStore`
- `impl BorrowMut for BookmarkStore`
- `impl Into for BookmarkStore`
- `impl From for BookmarkStore`
- `impl TryInto for BookmarkStore`
- `impl TryFrom for BookmarkStore`
- `impl Any for BookmarkStore`
- `impl VZip for BookmarkStore`
- `impl CastableFrom for BookmarkStore`
- `impl CastableFrom for BookmarkStore`
- `impl Read for BookmarkStore`
- `impl IntoEither for BookmarkStore`
- `impl ErasedDestructor for BookmarkStore`
- `impl Same for BookmarkStore`
- `impl Pointable for BookmarkStore`
- `impl Instrument for BookmarkStore`
- `impl WithSubscriber for BookmarkStore`
- `impl FromAngle for BookmarkStore`
- `impl IntoAngle for BookmarkStore`
- `impl IntoCam16Unclamped for BookmarkStore`
- `impl Cam16IntoUnclamped for BookmarkStore`
- `impl ArraysFrom for BookmarkStore`
- `impl ArraysInto for BookmarkStore`
- `impl ComponentsFrom for BookmarkStore`
- `impl TryComponentsInto for BookmarkStore`
- `impl UintsFrom for BookmarkStore`
- `impl UintsInto for BookmarkStore`
- `impl AdaptIntoUnclamped for BookmarkStore`
- `impl AdaptInto for BookmarkStore`
- `impl IntoColor for BookmarkStore`
- `impl IntoColorUnclamped for BookmarkStore`
- `impl TryIntoColor for BookmarkStore`
- `impl FromStimulus for BookmarkStore`
- `impl IntoStimulus for BookmarkStore`
- `impl EventEmitter for BookmarkStore`

## Auto Trait Implementations

`Freeze` `Unpin` `UnsafeUnpin`

