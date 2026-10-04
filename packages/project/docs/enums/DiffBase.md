---
id: DiffBase
title: DiffBase
---

# Enum: DiffBase

Defined in: [`packages/project/src/git_store/diff_buffer_list.rs:27`](../../../../packages/project/src/git_store/diff_buffer_list.rs#L27)

## Definition

```rust
pub enum DiffBase
{
    Head,
    Index,
    Staged,
    Merge{ .. },
}
```

## Implementations

### is_merge_base()

```rust
pub fn is_merge_base(&self) -> bool
```

Defined in: [`packages/project/src/git_store/diff_buffer_list.rs:35`](../../../../packages/project/src/git_store/diff_buffer_list.rs#L35)

#### Returns

`bool`

## Trait Implementations

- `impl Borrow for DiffBase`
- `impl BorrowMut for DiffBase`
- `impl CloneToUninit for DiffBase`
- `impl Into for DiffBase`
- `impl From for DiffBase`
- `impl TryInto for DiffBase`
- `impl TryFrom for DiffBase`
- `impl Any for DiffBase`
- `impl ToOwned for DiffBase`
- `impl DeserializeOwned for DiffBase`
- `impl Equivalent for DiffBase`
- `impl Serialize for DiffBase`
- `impl DynClone for DiffBase`
- `impl VZip for DiffBase`
- `impl CastableFrom for DiffBase`
- `impl CastableFrom for DiffBase`
- `impl Read for DiffBase`
- `impl IntoEither for DiffBase`
- `impl ErasedDestructor for DiffBase`
- `impl Same for DiffBase`
- `impl Pointable for DiffBase`
- `impl Instrument for DiffBase`
- `impl WithSubscriber for DiffBase`
- `impl FromAngle for DiffBase`
- `impl IntoAngle for DiffBase`
- `impl IntoCam16Unclamped for DiffBase`
- `impl Cam16IntoUnclamped for DiffBase`
- `impl ArraysFrom for DiffBase`
- `impl ArraysInto for DiffBase`
- `impl ComponentsFrom for DiffBase`
- `impl TryComponentsInto for DiffBase`
- `impl UintsFrom for DiffBase`
- `impl UintsInto for DiffBase`
- `impl AdaptIntoUnclamped for DiffBase`
- `impl AdaptInto for DiffBase`
- `impl IntoColor for DiffBase`
- `impl IntoColorUnclamped for DiffBase`
- `impl TryIntoColor for DiffBase`
- `impl FromStimulus for DiffBase`
- `impl IntoStimulus for DiffBase`
- `impl Equivalent for DiffBase`
- `impl Debug for DiffBase`
- `impl Clone for DiffBase`
- `impl StructuralPartialEq for DiffBase`
- `impl PartialEq for DiffBase`
- `impl Eq for DiffBase`
- `impl Serialize for DiffBase`
- `impl Deserialize for DiffBase`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

