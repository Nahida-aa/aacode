---
id: SearchHistory
title: SearchHistory
---

# Struct: SearchHistory

Defined in: [`packages/project/src/search_history.rs:39`](../../../../packages/project/src/search_history.rs#L39)

## Definition

```rust
pub struct SearchHistory
```

_（存在非公开字段）_

## Implementations

### new()

```rust
pub fn new(max_history_len: Option<usize>, insertion_behavior: QueryInsertionBehavior) -> Self
```

Defined in: [`packages/project/src/search_history.rs:46`](../../../../packages/project/src/search_history.rs#L46)

#### Parameters

##### max_history_len

`Option<usize>`

##### insertion_behavior

[`QueryInsertionBehavior`](../enums/QueryInsertionBehavior.md)

#### Returns

`Self`


***

### add()

```rust
pub fn add(&self, cursor: &SearchHistoryCursor, search_string: String)
```

Defined in: [`packages/project/src/search_history.rs:54`](../../../../packages/project/src/search_history.rs#L54)

#### Parameters

##### cursor

`&SearchHistoryCursor`

##### search_string

`String`


***

### next()

```rust
pub fn next(&self, cursor: &SearchHistoryCursor) -> Option<&str>
```

Defined in: [`packages/project/src/search_history.rs:76`](../../../../packages/project/src/search_history.rs#L76)

#### Parameters

##### cursor

`&SearchHistoryCursor`

#### Returns

`Option<&str>`


***

### current()

```rust
pub fn current(&self, cursor: &SearchHistoryCursor) -> Option<&str>
```

Defined in: [`packages/project/src/search_history.rs:85`](../../../../packages/project/src/search_history.rs#L85)

#### Parameters

##### cursor

`&SearchHistoryCursor`

#### Returns

`Option<&str>`


***

### previous()

```rust
pub fn previous(&self, cursor: &SearchHistoryCursor, current_query: &str) -> Option<&str>
```

Defined in: [`packages/project/src/search_history.rs:97`](../../../../packages/project/src/search_history.rs#L97)

Get the previous history entry using the given `SearchHistoryCursor`.
Uses the last element in the history when there is no cursor.

`current_query` is the current text in the search editor. If it differs
from the history entry at the cursor position (or if the cursor has no
selection), it is saved as a draft so it can be restored later.

#### Parameters

##### cursor

`&SearchHistoryCursor`

##### current_query

`&str`

#### Returns

`Option<&str>`


***

### len()

```rust
pub fn len(&self) -> usize
```

Defined in: [`packages/project/src/search_history.rs:120`](../../../../packages/project/src/search_history.rs#L120)

#### Returns

`usize`


***

### iter()

```rust
pub fn iter(&self) -> impl ?
```

Defined in: [`packages/project/src/search_history.rs:125`](../../../../packages/project/src/search_history.rs#L125)

Iterate over history entries from newest to oldest.

#### Returns

`impl ?`

## Trait Implementations

- `impl Borrow for SearchHistory`
- `impl BorrowMut for SearchHistory`
- `impl CloneToUninit for SearchHistory`
- `impl Into for SearchHistory`
- `impl From for SearchHistory`
- `impl TryInto for SearchHistory`
- `impl TryFrom for SearchHistory`
- `impl Any for SearchHistory`
- `impl ToOwned for SearchHistory`
- `impl DynClone for SearchHistory`
- `impl VZip for SearchHistory`
- `impl CastableFrom for SearchHistory`
- `impl CastableFrom for SearchHistory`
- `impl Read for SearchHistory`
- `impl IntoEither for SearchHistory`
- `impl ErasedDestructor for SearchHistory`
- `impl Same for SearchHistory`
- `impl Pointable for SearchHistory`
- `impl Instrument for SearchHistory`
- `impl WithSubscriber for SearchHistory`
- `impl FromAngle for SearchHistory`
- `impl IntoAngle for SearchHistory`
- `impl IntoCam16Unclamped for SearchHistory`
- `impl Cam16IntoUnclamped for SearchHistory`
- `impl ArraysFrom for SearchHistory`
- `impl ArraysInto for SearchHistory`
- `impl ComponentsFrom for SearchHistory`
- `impl TryComponentsInto for SearchHistory`
- `impl UintsFrom for SearchHistory`
- `impl UintsInto for SearchHistory`
- `impl AdaptIntoUnclamped for SearchHistory`
- `impl AdaptInto for SearchHistory`
- `impl IntoColor for SearchHistory`
- `impl IntoColorUnclamped for SearchHistory`
- `impl TryIntoColor for SearchHistory`
- `impl FromStimulus for SearchHistory`
- `impl IntoStimulus for SearchHistory`
- `impl Debug for SearchHistory`
- `impl Clone for SearchHistory`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

