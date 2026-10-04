---
id: GitTraversal
title: GitTraversal
---

# Struct: GitTraversal

Defined in: [`packages/project/src/git_store/git_traversal.rs:12`](../../../../packages/project/src/git_store/git_traversal.rs#L12)

Walks the worktree entries and their associated git statuses.

## Definition

```rust
pub struct GitTraversal<'a>
```

_（存在非公开字段）_

## Implementations

### new()

```rust
pub fn new(repo_snapshots: &'a HashMap<RepositoryId, RepositorySnapshot>, traversal: Traversal<'a>) -> GitTraversal<'a>
```

Defined in: [`packages/project/src/git_store/git_traversal.rs:23`](../../../../packages/project/src/git_store/git_traversal.rs#L23)

#### Parameters

##### repo_snapshots

`&'a HashMap<RepositoryId, RepositorySnapshot>`

##### traversal

`Traversal<'a>`

#### Returns

[`GitTraversal<'a>`](GitTraversal.md)


***

### advance()

```rust
pub fn advance(&self) -> bool
```

Defined in: [`packages/project/src/git_store/git_traversal.rs:113`](../../../../packages/project/src/git_store/git_traversal.rs#L113)

#### Returns

`bool`


***

### advance_to_sibling()

```rust
pub fn advance_to_sibling(&self) -> bool
```

Defined in: [`packages/project/src/git_store/git_traversal.rs:119`](../../../../packages/project/src/git_store/git_traversal.rs#L119)

#### Returns

`bool`


***

### back_to_parent()

```rust
pub fn back_to_parent(&self) -> bool
```

Defined in: [`packages/project/src/git_store/git_traversal.rs:125`](../../../../packages/project/src/git_store/git_traversal.rs#L125)

#### Returns

`bool`


***

### start_offset()

```rust
pub fn start_offset(&self) -> usize
```

Defined in: [`packages/project/src/git_store/git_traversal.rs:131`](../../../../packages/project/src/git_store/git_traversal.rs#L131)

#### Returns

`usize`


***

### end_offset()

```rust
pub fn end_offset(&self) -> usize
```

Defined in: [`packages/project/src/git_store/git_traversal.rs:135`](../../../../packages/project/src/git_store/git_traversal.rs#L135)

#### Returns

`usize`


***

### entry()

```rust
pub fn entry(&self) -> Option<GitEntryRef<'a>>
```

Defined in: [`packages/project/src/git_store/git_traversal.rs:139`](../../../../packages/project/src/git_store/git_traversal.rs#L139)

#### Returns

`Option<GitEntryRef<'a>>`

## Trait Implementations

- `impl Borrow for GitTraversal`
- `impl BorrowMut for GitTraversal`
- `impl Into for GitTraversal`
- `impl From for GitTraversal`
- `impl TryInto for GitTraversal`
- `impl TryFrom for GitTraversal`
- `impl Any for GitTraversal`
- `impl IntoIterator for GitTraversal`
- `impl IteratorRandom for GitTraversal`
- `impl VZip for GitTraversal`
- `impl CastableFrom for GitTraversal`
- `impl CastableFrom for GitTraversal`
- `impl Read for GitTraversal`
- `impl Itertools for GitTraversal`
- `impl IntoEither for GitTraversal`
- `impl ErasedDestructor for GitTraversal`
- `impl Same for GitTraversal`
- `impl TakeUntilExt for GitTraversal`
- `impl Pointable for GitTraversal`
- `impl Instrument for GitTraversal`
- `impl WithSubscriber for GitTraversal`
- `impl IntoStreamingIterator for GitTraversal`
- `impl TryCollect for GitTraversal`
- `impl IteratorRandom for GitTraversal`
- `impl FromAngle for GitTraversal`
- `impl IntoAngle for GitTraversal`
- `impl IntoCam16Unclamped for GitTraversal`
- `impl Cam16IntoUnclamped for GitTraversal`
- `impl ArraysFrom for GitTraversal`
- `impl ArraysInto for GitTraversal`
- `impl ComponentsFrom for GitTraversal`
- `impl TryComponentsInto for GitTraversal`
- `impl UintsFrom for GitTraversal`
- `impl UintsInto for GitTraversal`
- `impl AdaptIntoUnclamped for GitTraversal`
- `impl AdaptInto for GitTraversal`
- `impl IntoColor for GitTraversal`
- `impl IntoColorUnclamped for GitTraversal`
- `impl TryIntoColor for GitTraversal`
- `impl FromStimulus for GitTraversal`
- `impl IntoStimulus for GitTraversal`
- `impl Iterator for GitTraversal`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

