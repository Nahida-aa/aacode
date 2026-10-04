---
id: Search
title: Search
---

# Struct: Search

Defined in: [`packages/project/src/project_search.rs:38`](../../../../packages/project/src/project_search.rs#L38)

## Definition

```rust
pub struct Search
```

_（存在非公开字段）_

## Implementations

### local()

```rust
pub fn local(fs: Arc<dyn Fs>, buffer_store: Entity<BufferStore>, worktree_store: Entity<WorktreeStore>, limit: usize, cx: &App) -> Self
```

Defined in: [`packages/project/src/project_search.rs:115`](../../../../packages/project/src/project_search.rs#L115)

#### Parameters

##### fs

`Arc<dyn Fs>`

##### buffer_store

`Entity<BufferStore>`

##### worktree_store

`Entity<WorktreeStore>`

##### limit

`usize`

##### cx

`&App`

#### Returns

`Self`


***

### into_handle()

```rust
pub fn into_handle(self, query: SearchQuery, cx: &App) -> SearchResultsHandle
```

Defined in: [`packages/project/src/project_search.rs:169`](../../../../packages/project/src/project_search.rs#L169)

Prepares a project search run. The resulting [`SearchResultsHandle`](SearchResultsHandle.md) has to be used to specify whether you're interested in matching buffers
or full search results.

#### Parameters

##### query

[`SearchQuery`](../enums/SearchQuery.md)

##### cx

`&App`

#### Returns

[`SearchResultsHandle`](SearchResultsHandle.md)

## Trait Implementations

- `impl Borrow for Search`
- `impl BorrowMut for Search`
- `impl Into for Search`
- `impl From for Search`
- `impl TryInto for Search`
- `impl TryFrom for Search`
- `impl Any for Search`
- `impl VZip for Search`
- `impl CastableFrom for Search`
- `impl CastableFrom for Search`
- `impl Read for Search`
- `impl IntoEither for Search`
- `impl ErasedDestructor for Search`
- `impl Same for Search`
- `impl Pointable for Search`
- `impl Instrument for Search`
- `impl WithSubscriber for Search`
- `impl FromAngle for Search`
- `impl IntoAngle for Search`
- `impl IntoCam16Unclamped for Search`
- `impl Cam16IntoUnclamped for Search`
- `impl ArraysFrom for Search`
- `impl ArraysInto for Search`
- `impl ComponentsFrom for Search`
- `impl TryComponentsInto for Search`
- `impl UintsFrom for Search`
- `impl UintsInto for Search`
- `impl AdaptIntoUnclamped for Search`
- `impl AdaptInto for Search`
- `impl IntoColor for Search`
- `impl IntoColorUnclamped for Search`
- `impl TryIntoColor for Search`
- `impl FromStimulus for Search`
- `impl IntoStimulus for Search`

## Auto Trait Implementations

`Freeze` `Send` `Sync` `Unpin` `UnsafeUnpin`

