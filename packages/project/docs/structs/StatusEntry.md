---
id: StatusEntry
title: StatusEntry
---

# Struct: StatusEntry

Defined in: [`packages/project/src/git_store/mod.rs:496`](../../../../packages/project/src/git_store/mod.rs#L496)

## Definition

```rust
pub struct StatusEntry
{
    pub repo_path: RepoPath,
    pub status: FileStatus,
    pub diff_stat: Option<DiffStat>,
    pub staged_diff_stat: Option<DiffStat>,
    pub unstaged_diff_stat: Option<DiffStat>,
}
```

## Trait Implementations

- `impl Borrow for StatusEntry`
- `impl BorrowMut for StatusEntry`
- `impl CloneToUninit for StatusEntry`
- `impl Into for StatusEntry`
- `impl From for StatusEntry`
- `impl TryInto for StatusEntry`
- `impl TryFrom for StatusEntry`
- `impl Any for StatusEntry`
- `impl ToOwned for StatusEntry`
- `impl Equivalent for StatusEntry`
- `impl DynClone for StatusEntry`
- `impl VZip for StatusEntry`
- `impl CastableFrom for StatusEntry`
- `impl CastableFrom for StatusEntry`
- `impl Read for StatusEntry`
- `impl IntoEither for StatusEntry`
- `impl ErasedDestructor for StatusEntry`
- `impl Same for StatusEntry`
- `impl Pointable for StatusEntry`
- `impl Instrument for StatusEntry`
- `impl WithSubscriber for StatusEntry`
- `impl FromAngle for StatusEntry`
- `impl IntoAngle for StatusEntry`
- `impl IntoCam16Unclamped for StatusEntry`
- `impl Cam16IntoUnclamped for StatusEntry`
- `impl ArraysFrom for StatusEntry`
- `impl ArraysInto for StatusEntry`
- `impl ComponentsFrom for StatusEntry`
- `impl TryComponentsInto for StatusEntry`
- `impl UintsFrom for StatusEntry`
- `impl UintsInto for StatusEntry`
- `impl AdaptIntoUnclamped for StatusEntry`
- `impl AdaptInto for StatusEntry`
- `impl IntoColor for StatusEntry`
- `impl IntoColorUnclamped for StatusEntry`
- `impl TryIntoColor for StatusEntry`
- `impl FromStimulus for StatusEntry`
- `impl IntoStimulus for StatusEntry`
- `impl Equivalent for StatusEntry`
- `impl Clone for StatusEntry`
- `impl Debug for StatusEntry`
- `impl StructuralPartialEq for StatusEntry`
- `impl PartialEq for StatusEntry`
- `impl Eq for StatusEntry`
- `impl TryFrom for StatusEntry`
- `impl Item for StatusEntry`
- `impl KeyedItem for StatusEntry`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

