---
id: SearchResultsHandle
title: SearchResultsHandle
---

# Struct: SearchResultsHandle

Defined in: [`packages/project/src/project_search.rs:66`](../../../../packages/project/src/project_search.rs#L66)

Represents results of project search and allows one to either obtain match positions OR
just the handles to buffers that may match the search. Grabbing the handles is cheaper than obtaining full match positions, because in that case we'll look for
at most one match in each file.

## Definition

```rust
pub struct SearchResultsHandle
```

_（存在非公开字段）_

## Implementations

### results()

```rust
pub fn results(self, cx: &App) -> SearchResults<SearchResult>
```

Defined in: [`packages/project/src/project_search.rs:77`](../../../../packages/project/src/project_search.rs#L77)

#### Parameters

##### cx

`&App`

#### Returns

[`SearchResults<SearchResult>`](SearchResults.md)


***

### matching_buffers()

```rust
pub fn matching_buffers(self, cx: &App) -> SearchResults<(Entity<Buffer>, MatchPositionHint)>
```

Defined in: [`packages/project/src/project_search.rs:83`](../../../../packages/project/src/project_search.rs#L83)

#### Parameters

##### cx

`&App`

#### Returns

[`SearchResults<(Entity<Buffer>, MatchPositionHint)>`](SearchResults.md)

## Trait Implementations

- `impl Borrow for SearchResultsHandle`
- `impl BorrowMut for SearchResultsHandle`
- `impl Into for SearchResultsHandle`
- `impl From for SearchResultsHandle`
- `impl TryInto for SearchResultsHandle`
- `impl TryFrom for SearchResultsHandle`
- `impl Any for SearchResultsHandle`
- `impl VZip for SearchResultsHandle`
- `impl CastableFrom for SearchResultsHandle`
- `impl CastableFrom for SearchResultsHandle`
- `impl Read for SearchResultsHandle`
- `impl IntoEither for SearchResultsHandle`
- `impl ErasedDestructor for SearchResultsHandle`
- `impl Same for SearchResultsHandle`
- `impl Pointable for SearchResultsHandle`
- `impl Instrument for SearchResultsHandle`
- `impl WithSubscriber for SearchResultsHandle`
- `impl FromAngle for SearchResultsHandle`
- `impl IntoAngle for SearchResultsHandle`
- `impl IntoCam16Unclamped for SearchResultsHandle`
- `impl Cam16IntoUnclamped for SearchResultsHandle`
- `impl ArraysFrom for SearchResultsHandle`
- `impl ArraysInto for SearchResultsHandle`
- `impl ComponentsFrom for SearchResultsHandle`
- `impl TryComponentsInto for SearchResultsHandle`
- `impl UintsFrom for SearchResultsHandle`
- `impl UintsInto for SearchResultsHandle`
- `impl AdaptIntoUnclamped for SearchResultsHandle`
- `impl AdaptInto for SearchResultsHandle`
- `impl IntoColor for SearchResultsHandle`
- `impl IntoColorUnclamped for SearchResultsHandle`
- `impl TryIntoColor for SearchResultsHandle`
- `impl FromStimulus for SearchResultsHandle`
- `impl IntoStimulus for SearchResultsHandle`

## Auto Trait Implementations

`Freeze` `Send` `Sync`

