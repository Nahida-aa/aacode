---
id: LabelPresence
title: LabelPresence
---

# Enum: LabelPresence

Defined in: [`packages/project/src/manifest_tree/path_trie.rs:36`](../../../../packages/project/src/manifest_tree/path_trie.rs#L36)

Label presence is a marker that allows to optimize searches within [RootPathTrie](../structs/RootPathTrie.md); node label can be:
- Present; we know there's definitely a project root at this node.
- Known Absent - we know there's definitely no project root at this node and none of it's ancestors are Present (descendants can be present though!).
  The distinction is there to optimize searching; when we encounter a node with unknown status, we don't need to look at it's full path
  to the root of the worktree; it's sufficient to explore only the path between last node with a KnownAbsent state and the directory of a path, since we run searches
  from the leaf up to the root of the worktree.

In practical terms, it means that by storing label presence we don't need to do a project discovery on a given folder more than once
(unless the node is invalidated, which can happen when FS entries are renamed/removed).

Storing absent nodes allows us to recognize which paths have already been scanned for a project root unsuccessfully. This way we don't need to run
such scan more than once.

## Definition

```rust
pub enum LabelPresence {
    KnownAbsent,
    Present,
}
```

## Trait Implementations

- `impl Borrow for LabelPresence`
- `impl BorrowMut for LabelPresence`
- `impl CloneToUninit for LabelPresence`
- `impl Into for LabelPresence`
- `impl From for LabelPresence`
- `impl TryInto for LabelPresence`
- `impl TryFrom for LabelPresence`
- `impl Any for LabelPresence`
- `impl ToOwned for LabelPresence`
- `impl Equivalent for LabelPresence`
- `impl Comparable for LabelPresence`
- `impl DynClone for LabelPresence`
- `impl VZip for LabelPresence`
- `impl CastableFrom for LabelPresence`
- `impl CastableFrom for LabelPresence`
- `impl Read for LabelPresence`
- `impl IntoEither for LabelPresence`
- `impl ErasedDestructor for LabelPresence`
- `impl Same for LabelPresence`
- `impl Pointable for LabelPresence`
- `impl MapSeekTarget for LabelPresence`
- `impl Instrument for LabelPresence`
- `impl WithSubscriber for LabelPresence`
- `impl FromAngle for LabelPresence`
- `impl IntoAngle for LabelPresence`
- `impl IntoCam16Unclamped for LabelPresence`
- `impl Cam16IntoUnclamped for LabelPresence`
- `impl ArraysFrom for LabelPresence`
- `impl ArraysInto for LabelPresence`
- `impl ComponentsFrom for LabelPresence`
- `impl TryComponentsInto for LabelPresence`
- `impl UintsFrom for LabelPresence`
- `impl UintsInto for LabelPresence`
- `impl AdaptIntoUnclamped for LabelPresence`
- `impl AdaptInto for LabelPresence`
- `impl IntoColor for LabelPresence`
- `impl IntoColorUnclamped for LabelPresence`
- `impl TryIntoColor for LabelPresence`
- `impl FromStimulus for LabelPresence`
- `impl IntoStimulus for LabelPresence`
- `impl Equivalent for LabelPresence`
- `impl ResetDiscriminant for LabelPresence`
- `impl Clone for LabelPresence`
- `impl Copy for LabelPresence`
- `impl Debug for LabelPresence`
- `impl PartialOrd for LabelPresence`
- `impl StructuralPartialEq for LabelPresence`
- `impl PartialEq for LabelPresence`
- `impl Ord for LabelPresence`
- `impl Eq for LabelPresence`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

