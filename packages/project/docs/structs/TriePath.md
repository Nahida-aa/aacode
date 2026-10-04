---
id: TriePath
title: TriePath
---

# Struct: TriePath

Defined in: [`packages/project/src/manifest_tree/path_trie.rs:123`](../../../../packages/project/src/manifest_tree/path_trie.rs#L123)

[TriePath](TriePath.md) is a [Path] preprocessed for amortizing the cost of doing multiple lookups in distinct [RootPathTrie](RootPathTrie.md)s.

## Definition

```rust
pub struct TriePath
```

## Implementations

### new()

```rust
pub fn new(value: &RelPath) -> Self
```

Defined in: [`packages/project/src/manifest_tree/path_trie.rs:126`](../../../../packages/project/src/manifest_tree/path_trie.rs#L126)

#### Parameters

##### value

`&RelPath`

#### Returns

`Self`

## Trait Implementations

- `impl Borrow for TriePath`
- `impl BorrowMut for TriePath`
- `impl CloneToUninit for TriePath`
- `impl Into for TriePath`
- `impl From for TriePath`
- `impl TryInto for TriePath`
- `impl TryFrom for TriePath`
- `impl Any for TriePath`
- `impl ToOwned for TriePath`
- `impl DynClone for TriePath`
- `impl VZip for TriePath`
- `impl CastableFrom for TriePath`
- `impl CastableFrom for TriePath`
- `impl Read for TriePath`
- `impl IntoEither for TriePath`
- `impl ErasedDestructor for TriePath`
- `impl Same for TriePath`
- `impl Pointable for TriePath`
- `impl Instrument for TriePath`
- `impl WithSubscriber for TriePath`
- `impl FromAngle for TriePath`
- `impl IntoAngle for TriePath`
- `impl IntoCam16Unclamped for TriePath`
- `impl Cam16IntoUnclamped for TriePath`
- `impl ArraysFrom for TriePath`
- `impl ArraysInto for TriePath`
- `impl ComponentsFrom for TriePath`
- `impl TryComponentsInto for TriePath`
- `impl UintsFrom for TriePath`
- `impl UintsInto for TriePath`
- `impl AdaptIntoUnclamped for TriePath`
- `impl AdaptInto for TriePath`
- `impl IntoColor for TriePath`
- `impl IntoColorUnclamped for TriePath`
- `impl TryIntoColor for TriePath`
- `impl FromStimulus for TriePath`
- `impl IntoStimulus for TriePath`
- `impl Clone for TriePath`
- `impl From for TriePath`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

