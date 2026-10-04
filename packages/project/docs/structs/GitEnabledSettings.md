---
id: GitEnabledSettings
title: GitEnabledSettings
---

# Struct: GitEnabledSettings

Defined in: [`packages/project/src/project_settings.rs:509`](../../../../packages/project/src/project_settings.rs#L509)

## Definition

```rust
pub struct GitEnabledSettings
{
    // Whether git integration is enabled for showing git status. Default: true
    pub status: bool,
    // Whether git integration is enabled for showing diffs. Default: true
    pub diff: bool,
}
```

## Fields

### status

Defined in: [`packages/project/src/project_settings.rs:513`](../../../../packages/project/src/project_settings.rs#L513)

Whether git integration is enabled for showing git status.

Default: true


***

### diff

Defined in: [`packages/project/src/project_settings.rs:517`](../../../../packages/project/src/project_settings.rs#L517)

Whether git integration is enabled for showing diffs.

Default: true

## Trait Implementations

- `impl Borrow for GitEnabledSettings`
- `impl BorrowMut for GitEnabledSettings`
- `impl CloneToUninit for GitEnabledSettings`
- `impl Into for GitEnabledSettings`
- `impl From for GitEnabledSettings`
- `impl TryInto for GitEnabledSettings`
- `impl TryFrom for GitEnabledSettings`
- `impl Any for GitEnabledSettings`
- `impl ToOwned for GitEnabledSettings`
- `impl DynClone for GitEnabledSettings`
- `impl VZip for GitEnabledSettings`
- `impl CastableFrom for GitEnabledSettings`
- `impl CastableFrom for GitEnabledSettings`
- `impl Read for GitEnabledSettings`
- `impl IntoEither for GitEnabledSettings`
- `impl ErasedDestructor for GitEnabledSettings`
- `impl Same for GitEnabledSettings`
- `impl Pointable for GitEnabledSettings`
- `impl Instrument for GitEnabledSettings`
- `impl WithSubscriber for GitEnabledSettings`
- `impl FromAngle for GitEnabledSettings`
- `impl IntoAngle for GitEnabledSettings`
- `impl IntoCam16Unclamped for GitEnabledSettings`
- `impl Cam16IntoUnclamped for GitEnabledSettings`
- `impl ArraysFrom for GitEnabledSettings`
- `impl ArraysInto for GitEnabledSettings`
- `impl ComponentsFrom for GitEnabledSettings`
- `impl TryComponentsInto for GitEnabledSettings`
- `impl UintsFrom for GitEnabledSettings`
- `impl UintsInto for GitEnabledSettings`
- `impl AdaptIntoUnclamped for GitEnabledSettings`
- `impl AdaptInto for GitEnabledSettings`
- `impl IntoColor for GitEnabledSettings`
- `impl IntoColorUnclamped for GitEnabledSettings`
- `impl TryIntoColor for GitEnabledSettings`
- `impl FromStimulus for GitEnabledSettings`
- `impl IntoStimulus for GitEnabledSettings`
- `impl ResetDiscriminant for GitEnabledSettings`
- `impl Clone for GitEnabledSettings`
- `impl Copy for GitEnabledSettings`
- `impl Debug for GitEnabledSettings`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

