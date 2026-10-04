---
id: SessionSettings
title: SessionSettings
---

# Struct: SessionSettings

Defined in: [`packages/project/src/project_settings.rs:85`](../../../../packages/project/src/project_settings.rs#L85)

## Definition

```rust
pub struct SessionSettings {
    // Whether or not to restore unsaved buffers on restart. If this is true, user won't be prompted whether to save/discard dirty files when closing the application. Default: true
    pub restore_unsaved_buffers: bool,
    // Whether or not to skip worktree trust checks. When trusted, project settings are synchronized automatically, language and MCP servers are downloaded and started automatically. Default: false
    pub trust_all_worktrees: bool,
}
```

## Fields

### restore_unsaved_buffers

Defined in: [`packages/project/src/project_settings.rs:92`](../../../../packages/project/src/project_settings.rs#L92)

Whether or not to restore unsaved buffers on restart.

If this is true, user won't be prompted whether to save/discard
dirty files when closing the application.

Default: true


***

### trust_all_worktrees

Defined in: [`packages/project/src/project_settings.rs:98`](../../../../packages/project/src/project_settings.rs#L98)

Whether or not to skip worktree trust checks.
When trusted, project settings are synchronized automatically,
language and MCP servers are downloaded and started automatically.

Default: false

## Trait Implementations

- `impl Borrow for SessionSettings`
- `impl BorrowMut for SessionSettings`
- `impl CloneToUninit for SessionSettings`
- `impl Into for SessionSettings`
- `impl From for SessionSettings`
- `impl TryInto for SessionSettings`
- `impl TryFrom for SessionSettings`
- `impl Any for SessionSettings`
- `impl ToOwned for SessionSettings`
- `impl DynClone for SessionSettings`
- `impl VZip for SessionSettings`
- `impl CastableFrom for SessionSettings`
- `impl CastableFrom for SessionSettings`
- `impl Read for SessionSettings`
- `impl IntoEither for SessionSettings`
- `impl ErasedDestructor for SessionSettings`
- `impl Same for SessionSettings`
- `impl Pointable for SessionSettings`
- `impl Instrument for SessionSettings`
- `impl WithSubscriber for SessionSettings`
- `impl FromAngle for SessionSettings`
- `impl IntoAngle for SessionSettings`
- `impl IntoCam16Unclamped for SessionSettings`
- `impl Cam16IntoUnclamped for SessionSettings`
- `impl ArraysFrom for SessionSettings`
- `impl ArraysInto for SessionSettings`
- `impl ComponentsFrom for SessionSettings`
- `impl TryComponentsInto for SessionSettings`
- `impl UintsFrom for SessionSettings`
- `impl UintsInto for SessionSettings`
- `impl AdaptIntoUnclamped for SessionSettings`
- `impl AdaptInto for SessionSettings`
- `impl IntoColor for SessionSettings`
- `impl IntoColorUnclamped for SessionSettings`
- `impl TryIntoColor for SessionSettings`
- `impl FromStimulus for SessionSettings`
- `impl IntoStimulus for SessionSettings`
- `impl ResetDiscriminant for SessionSettings`
- `impl Copy for SessionSettings`
- `impl Clone for SessionSettings`
- `impl Debug for SessionSettings`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

