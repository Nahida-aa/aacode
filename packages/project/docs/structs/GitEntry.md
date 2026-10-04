---
id: GitEntry
title: GitEntry
---

# Struct: GitEntry

Defined in: [`packages/project/src/git_store/git_traversal.rs:226`](../../../../packages/project/src/git_store/git_traversal.rs#L226)

## Definition

```rust
pub struct GitEntry
{
    pub entry: Entry,
    pub git_summary: GitSummary,
}
```

## Implementations

### to_ref()

```rust
pub fn to_ref(&self) -> GitEntryRef<'_>
```

Defined in: [`packages/project/src/git_store/git_traversal.rs:232`](../../../../packages/project/src/git_store/git_traversal.rs#L232)

#### Returns

[`GitEntryRef<'_>`](GitEntryRef.md)

## Trait Implementations

- `impl Borrow for GitEntry`
- `impl BorrowMut for GitEntry`
- `impl CloneToUninit for GitEntry`
- `impl Into for GitEntry`
- `impl From for GitEntry`
- `impl TryInto for GitEntry`
- `impl TryFrom for GitEntry`
- `impl Receiver for GitEntry`
- `impl Any for GitEntry`
- `impl ToOwned for GitEntry`
- `impl Equivalent for GitEntry`
- `impl DynClone for GitEntry`
- `impl VZip for GitEntry`
- `impl CastableFrom for GitEntry`
- `impl CastableFrom for GitEntry`
- `impl Read for GitEntry`
- `impl IntoEither for GitEntry`
- `impl ErasedDestructor for GitEntry`
- `impl Same for GitEntry`
- `impl Pointable for GitEntry`
- `impl Instrument for GitEntry`
- `impl WithSubscriber for GitEntry`
- `impl FromAngle for GitEntry`
- `impl IntoAngle for GitEntry`
- `impl IntoCam16Unclamped for GitEntry`
- `impl Cam16IntoUnclamped for GitEntry`
- `impl ArraysFrom for GitEntry`
- `impl ArraysInto for GitEntry`
- `impl ComponentsFrom for GitEntry`
- `impl TryComponentsInto for GitEntry`
- `impl UintsFrom for GitEntry`
- `impl UintsInto for GitEntry`
- `impl AdaptIntoUnclamped for GitEntry`
- `impl AdaptInto for GitEntry`
- `impl IntoColor for GitEntry`
- `impl IntoColorUnclamped for GitEntry`
- `impl TryIntoColor for GitEntry`
- `impl FromStimulus for GitEntry`
- `impl IntoStimulus for GitEntry`
- `impl Equivalent for GitEntry`
- `impl Debug for GitEntry`
- `impl Clone for GitEntry`
- `impl StructuralPartialEq for GitEntry`
- `impl PartialEq for GitEntry`
- `impl Eq for GitEntry`
- `impl Deref for GitEntry`
- `impl AsRef for GitEntry`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

