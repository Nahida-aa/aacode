---
id: GitAccess
title: GitAccess
---

# Enum: GitAccess

Defined in: [`packages/project/src/git_store/mod.rs:449`](../../../../packages/project/src/git_store/mod.rs#L449)

## Definition

```rust
pub enum GitAccess
{
    // Either: - the user owns `.git` - the user doesn't own `.git`, but has both of: - OS-level read permissions - the directory is marked as safe (git config safe.directory)
    Yes,
    // The user is not the owner of `.git`, and one of the following is true: - the directory is not marked as safe (git config safe.directory) - the user does not have OS-level read permissions to `.git`
    No,
}
```

## Variants

### Yes

Defined in: [`packages/project/src/git_store/mod.rs:455`](../../../../packages/project/src/git_store/mod.rs#L455)

Either:
- the user owns `.git`
- the user doesn't own `.git`, but has both of:
  - OS-level read permissions
  - the directory is marked as safe (git config safe.directory)


***

### No

Defined in: [`packages/project/src/git_store/mod.rs:460`](../../../../packages/project/src/git_store/mod.rs#L460)

The user is not the owner of `.git`, and one of the following is true:
- the directory is not marked as safe (git config safe.directory)
- the user does not have OS-level read permissions to `.git`

## Trait Implementations

- `impl Borrow for GitAccess`
- `impl BorrowMut for GitAccess`
- `impl CloneToUninit for GitAccess`
- `impl Into for GitAccess`
- `impl From for GitAccess`
- `impl TryInto for GitAccess`
- `impl TryFrom for GitAccess`
- `impl Any for GitAccess`
- `impl ToOwned for GitAccess`
- `impl DynClone for GitAccess`
- `impl VZip for GitAccess`
- `impl CastableFrom for GitAccess`
- `impl CastableFrom for GitAccess`
- `impl Read for GitAccess`
- `impl IntoEither for GitAccess`
- `impl ErasedDestructor for GitAccess`
- `impl Same for GitAccess`
- `impl Pointable for GitAccess`
- `impl Instrument for GitAccess`
- `impl WithSubscriber for GitAccess`
- `impl FromAngle for GitAccess`
- `impl IntoAngle for GitAccess`
- `impl IntoCam16Unclamped for GitAccess`
- `impl Cam16IntoUnclamped for GitAccess`
- `impl ArraysFrom for GitAccess`
- `impl ArraysInto for GitAccess`
- `impl ComponentsFrom for GitAccess`
- `impl TryComponentsInto for GitAccess`
- `impl UintsFrom for GitAccess`
- `impl UintsInto for GitAccess`
- `impl AdaptIntoUnclamped for GitAccess`
- `impl AdaptInto for GitAccess`
- `impl IntoColor for GitAccess`
- `impl IntoColorUnclamped for GitAccess`
- `impl TryIntoColor for GitAccess`
- `impl FromStimulus for GitAccess`
- `impl IntoStimulus for GitAccess`
- `impl ResetDiscriminant for GitAccess`
- `impl Debug for GitAccess`
- `impl Clone for GitAccess`
- `impl Copy for GitAccess`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

