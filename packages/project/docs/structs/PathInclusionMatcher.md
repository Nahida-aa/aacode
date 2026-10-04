---
id: PathInclusionMatcher
title: PathInclusionMatcher
---

# Struct: PathInclusionMatcher

Defined in: [`packages/project/src/project_search.rs:975`](../../../../packages/project/src/project_search.rs#L975)

This struct encapsulates the logic to decide whether a given gitignored directory should be
scanned based on include/exclude patterns of a search query (as include/exclude parameters may match paths inside it).
It is kind-of doing an inverse of glob. Given a glob pattern like `src/**/` and a parent path like `src`, we need to decide whether the parent
may contain glob hits.

## Definition

```rust
pub struct PathInclusionMatcher
```

_（存在非公开字段）_

## Implementations

### new()

```rust
pub fn new(query: Arc<SearchQuery>) -> Self
```

Defined in: [`packages/project/src/project_search.rs:981`](../../../../packages/project/src/project_search.rs#L981)

#### Parameters

##### query

`Arc<SearchQuery>`

#### Returns

`Self`


***

### should_scan_gitignored_dir()

```rust
pub fn should_scan_gitignored_dir(&self, entry: &Entry, snapshot: &Snapshot, worktree_settings: &WorktreeSettings) -> bool
```

Defined in: [`packages/project/src/project_search.rs:997`](../../../../packages/project/src/project_search.rs#L997)

#### Parameters

##### entry

`&Entry`

##### snapshot

`&Snapshot`

##### worktree_settings

`&WorktreeSettings`

#### Returns

`bool`

## Trait Implementations

- `impl Borrow for PathInclusionMatcher`
- `impl BorrowMut for PathInclusionMatcher`
- `impl Into for PathInclusionMatcher`
- `impl From for PathInclusionMatcher`
- `impl TryInto for PathInclusionMatcher`
- `impl TryFrom for PathInclusionMatcher`
- `impl Any for PathInclusionMatcher`
- `impl VZip for PathInclusionMatcher`
- `impl CastableFrom for PathInclusionMatcher`
- `impl CastableFrom for PathInclusionMatcher`
- `impl Read for PathInclusionMatcher`
- `impl IntoEither for PathInclusionMatcher`
- `impl ErasedDestructor for PathInclusionMatcher`
- `impl Same for PathInclusionMatcher`
- `impl Pointable for PathInclusionMatcher`
- `impl Instrument for PathInclusionMatcher`
- `impl WithSubscriber for PathInclusionMatcher`
- `impl FromAngle for PathInclusionMatcher`
- `impl IntoAngle for PathInclusionMatcher`
- `impl IntoCam16Unclamped for PathInclusionMatcher`
- `impl Cam16IntoUnclamped for PathInclusionMatcher`
- `impl ArraysFrom for PathInclusionMatcher`
- `impl ArraysInto for PathInclusionMatcher`
- `impl ComponentsFrom for PathInclusionMatcher`
- `impl TryComponentsInto for PathInclusionMatcher`
- `impl UintsFrom for PathInclusionMatcher`
- `impl UintsInto for PathInclusionMatcher`
- `impl AdaptIntoUnclamped for PathInclusionMatcher`
- `impl AdaptInto for PathInclusionMatcher`
- `impl IntoColor for PathInclusionMatcher`
- `impl IntoColorUnclamped for PathInclusionMatcher`
- `impl TryIntoColor for PathInclusionMatcher`
- `impl FromStimulus for PathInclusionMatcher`
- `impl IntoStimulus for PathInclusionMatcher`

## Auto Trait Implementations

`Freeze` `Send` `Sync` `Unpin` `UnsafeUnpin`

