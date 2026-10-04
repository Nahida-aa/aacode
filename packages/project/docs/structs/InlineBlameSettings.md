---
id: InlineBlameSettings
title: InlineBlameSettings
---

# Struct: InlineBlameSettings

Defined in: [`packages/project/src/project_settings.rs:553`](../../../../packages/project/src/project_settings.rs#L553)

## Definition

```rust
pub struct InlineBlameSettings {
    // Whether or not to show git blame data inline in the currently focused line. Default: true
    pub enabled: bool,
    // Whether to only show the inline blame information after a delay once the cursor stops moving. Default: 0
    pub delay_ms: DelayMs,
    // Where to render the blame information when enabled. Default: inline
    pub location: InlineBlameLocation,
    // The amount of padding between the end of the source line and the start of the inline blame in units of columns. Default: 7
    pub padding: u32,
    // The minimum column number to show the inline blame information at Default: 0
    pub min_column: u32,
    // Whether to show commit summary as part of the inline blame. Default: false
    pub show_commit_summary: bool,
}
```

## Fields

### enabled

Defined in: [`packages/project/src/project_settings.rs:558`](../../../../packages/project/src/project_settings.rs#L558)

Whether or not to show git blame data inline in
the currently focused line.

Default: true


***

### delay_ms

Defined in: [`packages/project/src/project_settings.rs:563`](../../../../packages/project/src/project_settings.rs#L563)

Whether to only show the inline blame information
after a delay once the cursor stops moving.

Default: 0


***

### location

Defined in: [`packages/project/src/project_settings.rs:567`](../../../../packages/project/src/project_settings.rs#L567)

Where to render the blame information when enabled.

Default: inline


***

### padding

Defined in: [`packages/project/src/project_settings.rs:572`](../../../../packages/project/src/project_settings.rs#L572)

The amount of padding between the end of the source line and the start
of the inline blame in units of columns.

Default: 7


***

### min_column

Defined in: [`packages/project/src/project_settings.rs:576`](../../../../packages/project/src/project_settings.rs#L576)

The minimum column number to show the inline blame information at

Default: 0


***

### show_commit_summary

Defined in: [`packages/project/src/project_settings.rs:580`](../../../../packages/project/src/project_settings.rs#L580)

Whether to show commit summary as part of the inline blame.

Default: false

## Trait Implementations

- `impl Borrow for InlineBlameSettings`
- `impl BorrowMut for InlineBlameSettings`
- `impl CloneToUninit for InlineBlameSettings`
- `impl Into for InlineBlameSettings`
- `impl From for InlineBlameSettings`
- `impl TryInto for InlineBlameSettings`
- `impl TryFrom for InlineBlameSettings`
- `impl Any for InlineBlameSettings`
- `impl ToOwned for InlineBlameSettings`
- `impl DynClone for InlineBlameSettings`
- `impl VZip for InlineBlameSettings`
- `impl CastableFrom for InlineBlameSettings`
- `impl CastableFrom for InlineBlameSettings`
- `impl Read for InlineBlameSettings`
- `impl IntoEither for InlineBlameSettings`
- `impl ErasedDestructor for InlineBlameSettings`
- `impl Same for InlineBlameSettings`
- `impl Pointable for InlineBlameSettings`
- `impl Instrument for InlineBlameSettings`
- `impl WithSubscriber for InlineBlameSettings`
- `impl FromAngle for InlineBlameSettings`
- `impl IntoAngle for InlineBlameSettings`
- `impl IntoCam16Unclamped for InlineBlameSettings`
- `impl Cam16IntoUnclamped for InlineBlameSettings`
- `impl ArraysFrom for InlineBlameSettings`
- `impl ArraysInto for InlineBlameSettings`
- `impl ComponentsFrom for InlineBlameSettings`
- `impl TryComponentsInto for InlineBlameSettings`
- `impl UintsFrom for InlineBlameSettings`
- `impl UintsInto for InlineBlameSettings`
- `impl AdaptIntoUnclamped for InlineBlameSettings`
- `impl AdaptInto for InlineBlameSettings`
- `impl IntoColor for InlineBlameSettings`
- `impl IntoColorUnclamped for InlineBlameSettings`
- `impl TryIntoColor for InlineBlameSettings`
- `impl FromStimulus for InlineBlameSettings`
- `impl IntoStimulus for InlineBlameSettings`
- `impl ResetDiscriminant for InlineBlameSettings`
- `impl Clone for InlineBlameSettings`
- `impl Copy for InlineBlameSettings`
- `impl Debug for InlineBlameSettings`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

