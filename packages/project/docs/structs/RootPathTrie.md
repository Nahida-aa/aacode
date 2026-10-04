---
id: RootPathTrie
title: RootPathTrie
---

# Struct: RootPathTrie

Defined in: [`packages/project/src/manifest_tree/path_trie.rs:17`](../../../../packages/project/src/manifest_tree/path_trie.rs#L17)

[RootPathTrie](RootPathTrie.md) is a workhorse of [super::ManifestTree](ManifestTree.md). It is responsible for determining the closest known entry for a given path.
It also determines how much of a given path is unexplored, thus letting callers fill in that gap if needed.
Conceptually, it allows one to annotate Worktree entries with arbitrary extra metadata and run closest-ancestor searches.

A path is unexplored when the closest ancestor of a path is not the path itself; that means that we have not yet ran the scan on that path.
For example, if there's a project root at path `python/project` and we query for a path `python/project/subdir/another_subdir/file.py`, there is
a known root at `python/project` and the unexplored part is `subdir/another_subdir` - we need to run a scan on these 2 directories.

## Definition

```rust
pub struct RootPathTrie<Label>
```

_（存在非公开字段）_

## Implementations

### new()

```rust
pub fn new() -> Self
```

Defined in: [`packages/project/src/manifest_tree/path_trie.rs:42`](../../../../packages/project/src/manifest_tree/path_trie.rs#L42)

#### Returns

`Self`


***

### insert()

```rust
pub fn insert(&self, path: &TriePath, value: Label, presence: LabelPresence)
```

Defined in: [`packages/project/src/manifest_tree/path_trie.rs:78`](../../../../packages/project/src/manifest_tree/path_trie.rs#L78)

#### Parameters

##### path

`&TriePath`

##### value

`Label`

##### presence

[`LabelPresence`](../enums/LabelPresence.md)


***

### walk()

```rust
pub fn walk<'a>(&self, path: &TriePath, callback: &dyn FnMut(&'b Arc<RelPath>, &'a BTreeMap<Label, LabelPresence>) -> ControlFlow<()>)
```

Defined in: [`packages/project/src/manifest_tree/path_trie.rs:82`](../../../../packages/project/src/manifest_tree/path_trie.rs#L82)

#### Parameters

##### path

`&TriePath`

##### callback

`&dyn FnMut(&'b Arc<RelPath>, &'a BTreeMap<Label, LabelPresence>) -> ControlFlow<()>`


***

### remove()

```rust
pub fn remove(&self, path: &TriePath)
```

Defined in: [`packages/project/src/manifest_tree/path_trie.rs:107`](../../../../packages/project/src/manifest_tree/path_trie.rs#L107)

#### Parameters

##### path

`&TriePath`

## Trait Implementations

- `impl Borrow for RootPathTrie`
- `impl BorrowMut for RootPathTrie`
- `impl Into for RootPathTrie`
- `impl From for RootPathTrie`
- `impl TryInto for RootPathTrie`
- `impl TryFrom for RootPathTrie`
- `impl Any for RootPathTrie`
- `impl VZip for RootPathTrie`
- `impl CastableFrom for RootPathTrie`
- `impl CastableFrom for RootPathTrie`
- `impl Read for RootPathTrie`
- `impl IntoEither for RootPathTrie`
- `impl ErasedDestructor for RootPathTrie`
- `impl Same for RootPathTrie`
- `impl Pointable for RootPathTrie`
- `impl Instrument for RootPathTrie`
- `impl WithSubscriber for RootPathTrie`
- `impl FromAngle for RootPathTrie`
- `impl IntoAngle for RootPathTrie`
- `impl IntoCam16Unclamped for RootPathTrie`
- `impl Cam16IntoUnclamped for RootPathTrie`
- `impl ArraysFrom for RootPathTrie`
- `impl ArraysInto for RootPathTrie`
- `impl ComponentsFrom for RootPathTrie`
- `impl TryComponentsInto for RootPathTrie`
- `impl UintsFrom for RootPathTrie`
- `impl UintsInto for RootPathTrie`
- `impl AdaptIntoUnclamped for RootPathTrie`
- `impl AdaptInto for RootPathTrie`
- `impl IntoColor for RootPathTrie`
- `impl IntoColorUnclamped for RootPathTrie`
- `impl TryIntoColor for RootPathTrie`
- `impl FromStimulus for RootPathTrie`
- `impl IntoStimulus for RootPathTrie`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

