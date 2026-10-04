---
id: SettingsObserverMode
title: SettingsObserverMode
---

# Enum: SettingsObserverMode

Defined in: [`packages/project/src/project_settings.rs:807`](../../../../packages/project/src/project_settings.rs#L807)

## Definition

```rust
pub enum SettingsObserverMode {
    Local(Arc<dyn Fs>),
    Remote{ .. },
}
```

## Trait Implementations

- `impl Borrow for SettingsObserverMode`
- `impl BorrowMut for SettingsObserverMode`
- `impl Into for SettingsObserverMode`
- `impl From for SettingsObserverMode`
- `impl TryInto for SettingsObserverMode`
- `impl TryFrom for SettingsObserverMode`
- `impl Any for SettingsObserverMode`
- `impl VZip for SettingsObserverMode`
- `impl CastableFrom for SettingsObserverMode`
- `impl CastableFrom for SettingsObserverMode`
- `impl Read for SettingsObserverMode`
- `impl IntoEither for SettingsObserverMode`
- `impl ErasedDestructor for SettingsObserverMode`
- `impl Same for SettingsObserverMode`
- `impl Pointable for SettingsObserverMode`
- `impl Instrument for SettingsObserverMode`
- `impl WithSubscriber for SettingsObserverMode`
- `impl FromAngle for SettingsObserverMode`
- `impl IntoAngle for SettingsObserverMode`
- `impl IntoCam16Unclamped for SettingsObserverMode`
- `impl Cam16IntoUnclamped for SettingsObserverMode`
- `impl ArraysFrom for SettingsObserverMode`
- `impl ArraysInto for SettingsObserverMode`
- `impl ComponentsFrom for SettingsObserverMode`
- `impl TryComponentsInto for SettingsObserverMode`
- `impl UintsFrom for SettingsObserverMode`
- `impl UintsInto for SettingsObserverMode`
- `impl AdaptIntoUnclamped for SettingsObserverMode`
- `impl AdaptInto for SettingsObserverMode`
- `impl IntoColor for SettingsObserverMode`
- `impl IntoColorUnclamped for SettingsObserverMode`
- `impl TryIntoColor for SettingsObserverMode`
- `impl FromStimulus for SettingsObserverMode`
- `impl IntoStimulus for SettingsObserverMode`

## Auto Trait Implementations

`Freeze` `Send` `Sync` `Unpin` `UnsafeUnpin`

