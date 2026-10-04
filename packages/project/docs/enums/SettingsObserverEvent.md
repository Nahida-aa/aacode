---
id: SettingsObserverEvent
title: SettingsObserverEvent
---

# Enum: SettingsObserverEvent

Defined in: [`packages/project/src/project_settings.rs:813`](../../../../packages/project/src/project_settings.rs#L813)

## Definition

```rust
pub enum SettingsObserverEvent {
    LocalSettingsUpdated(Result<PathBuf, InvalidSettingsError>),
    LocalTasksUpdated(Result<PathBuf, InvalidSettingsError>),
    LocalDebugScenariosUpdated(Result<PathBuf, InvalidSettingsError>),
    GlobalTasksUpdated(Result<PathBuf, InvalidSettingsError>),
    GlobalDebugScenariosUpdated(Result<PathBuf, InvalidSettingsError>),
}
```

## Trait Implementations

- `impl Borrow for SettingsObserverEvent`
- `impl BorrowMut for SettingsObserverEvent`
- `impl CloneToUninit for SettingsObserverEvent`
- `impl Into for SettingsObserverEvent`
- `impl From for SettingsObserverEvent`
- `impl TryInto for SettingsObserverEvent`
- `impl TryFrom for SettingsObserverEvent`
- `impl Any for SettingsObserverEvent`
- `impl ToOwned for SettingsObserverEvent`
- `impl DynClone for SettingsObserverEvent`
- `impl VZip for SettingsObserverEvent`
- `impl CastableFrom for SettingsObserverEvent`
- `impl CastableFrom for SettingsObserverEvent`
- `impl Read for SettingsObserverEvent`
- `impl IntoEither for SettingsObserverEvent`
- `impl ErasedDestructor for SettingsObserverEvent`
- `impl Same for SettingsObserverEvent`
- `impl Pointable for SettingsObserverEvent`
- `impl Instrument for SettingsObserverEvent`
- `impl WithSubscriber for SettingsObserverEvent`
- `impl FromAngle for SettingsObserverEvent`
- `impl IntoAngle for SettingsObserverEvent`
- `impl IntoCam16Unclamped for SettingsObserverEvent`
- `impl Cam16IntoUnclamped for SettingsObserverEvent`
- `impl ArraysFrom for SettingsObserverEvent`
- `impl ArraysInto for SettingsObserverEvent`
- `impl ComponentsFrom for SettingsObserverEvent`
- `impl TryComponentsInto for SettingsObserverEvent`
- `impl UintsFrom for SettingsObserverEvent`
- `impl UintsInto for SettingsObserverEvent`
- `impl AdaptIntoUnclamped for SettingsObserverEvent`
- `impl AdaptInto for SettingsObserverEvent`
- `impl IntoColor for SettingsObserverEvent`
- `impl IntoColorUnclamped for SettingsObserverEvent`
- `impl TryIntoColor for SettingsObserverEvent`
- `impl FromStimulus for SettingsObserverEvent`
- `impl IntoStimulus for SettingsObserverEvent`
- `impl Clone for SettingsObserverEvent`
- `impl Debug for SettingsObserverEvent`
- `impl StructuralPartialEq for SettingsObserverEvent`
- `impl PartialEq for SettingsObserverEvent`
- `impl EventEmitter for SettingsObserverEvent`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

