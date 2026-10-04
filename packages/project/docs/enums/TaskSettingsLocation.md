---
id: TaskSettingsLocation
title: TaskSettingsLocation
---

# Enum: TaskSettingsLocation

Defined in: [`packages/project/src/task_store.rs:54`](../../../../packages/project/src/task_store.rs#L54)

## Definition

```rust
pub enum TaskSettingsLocation<'a> {
    Global(&'a Path),
    Worktree(SettingsLocation<'a>),
}
```

## Trait Implementations

- `impl Borrow for TaskSettingsLocation`
- `impl BorrowMut for TaskSettingsLocation`
- `impl Into for TaskSettingsLocation`
- `impl From for TaskSettingsLocation`
- `impl TryInto for TaskSettingsLocation`
- `impl TryFrom for TaskSettingsLocation`
- `impl Any for TaskSettingsLocation`
- `impl VZip for TaskSettingsLocation`
- `impl CastableFrom for TaskSettingsLocation`
- `impl CastableFrom for TaskSettingsLocation`
- `impl Read for TaskSettingsLocation`
- `impl IntoEither for TaskSettingsLocation`
- `impl ErasedDestructor for TaskSettingsLocation`
- `impl Same for TaskSettingsLocation`
- `impl Pointable for TaskSettingsLocation`
- `impl Instrument for TaskSettingsLocation`
- `impl WithSubscriber for TaskSettingsLocation`
- `impl FromAngle for TaskSettingsLocation`
- `impl IntoAngle for TaskSettingsLocation`
- `impl IntoCam16Unclamped for TaskSettingsLocation`
- `impl Cam16IntoUnclamped for TaskSettingsLocation`
- `impl ArraysFrom for TaskSettingsLocation`
- `impl ArraysInto for TaskSettingsLocation`
- `impl ComponentsFrom for TaskSettingsLocation`
- `impl TryComponentsInto for TaskSettingsLocation`
- `impl UintsFrom for TaskSettingsLocation`
- `impl UintsInto for TaskSettingsLocation`
- `impl AdaptIntoUnclamped for TaskSettingsLocation`
- `impl AdaptInto for TaskSettingsLocation`
- `impl IntoColor for TaskSettingsLocation`
- `impl IntoColorUnclamped for TaskSettingsLocation`
- `impl TryIntoColor for TaskSettingsLocation`
- `impl FromStimulus for TaskSettingsLocation`
- `impl IntoStimulus for TaskSettingsLocation`
- `impl Debug for TaskSettingsLocation`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

