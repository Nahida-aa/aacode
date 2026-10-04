---
id: GitEntryRef
title: GitEntryRef
---

# Struct: GitEntryRef

Defined in: [`packages/project/src/git_store/git_traversal.rs:197`](../../../../packages/project/src/git_store/git_traversal.rs#L197)

## Definition

```rust
pub struct GitEntryRef<'a>
{
    pub entry: &'a Entry,
    pub git_summary: GitSummary,
}
```

## Implementations

### to_owned()

```rust
pub fn to_owned(self) -> GitEntry
```

Defined in: [`packages/project/src/git_store/git_traversal.rs:203`](../../../../packages/project/src/git_store/git_traversal.rs#L203)

#### Returns

[`GitEntry`](GitEntry.md)

## Trait Implementations

- `impl Borrow for GitEntryRef`
- `impl BorrowMut for GitEntryRef`
- `impl CloneToUninit for GitEntryRef`
- `impl Into for GitEntryRef`
- `impl From for GitEntryRef`
- `impl TryInto for GitEntryRef`
- `impl TryFrom for GitEntryRef`
- `impl Receiver for GitEntryRef`
- `impl Any for GitEntryRef`
- `impl ToOwned for GitEntryRef`
- `impl DynClone for GitEntryRef`
- `impl VZip for GitEntryRef`
- `impl CastableFrom for GitEntryRef`
- `impl CastableFrom for GitEntryRef`
- `impl Read for GitEntryRef`
- `impl IntoEither for GitEntryRef`
- `impl ErasedDestructor for GitEntryRef`
- `impl Same for GitEntryRef`
- `impl Pointable for GitEntryRef`
- `impl Instrument for GitEntryRef`
- `impl WithSubscriber for GitEntryRef`
- `impl FromAngle for GitEntryRef`
- `impl IntoAngle for GitEntryRef`
- `impl IntoCam16Unclamped for GitEntryRef`
- `impl Cam16IntoUnclamped for GitEntryRef`
- `impl ArraysFrom for GitEntryRef`
- `impl ArraysInto for GitEntryRef`
- `impl ComponentsFrom for GitEntryRef`
- `impl TryComponentsInto for GitEntryRef`
- `impl UintsFrom for GitEntryRef`
- `impl UintsInto for GitEntryRef`
- `impl AdaptIntoUnclamped for GitEntryRef`
- `impl AdaptInto for GitEntryRef`
- `impl IntoColor for GitEntryRef`
- `impl IntoColorUnclamped for GitEntryRef`
- `impl TryIntoColor for GitEntryRef`
- `impl FromStimulus for GitEntryRef`
- `impl IntoStimulus for GitEntryRef`
- `impl ResetDiscriminant for GitEntryRef`
- `impl Debug for GitEntryRef`
- `impl Clone for GitEntryRef`
- `impl Copy for GitEntryRef`
- `impl Deref for GitEntryRef`
- `impl AsRef for GitEntryRef`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

