---
id: ContextServerSettings
title: ContextServerSettings
---

# Enum: ContextServerSettings

Defined in: [`packages/project/src/project_settings.rs:187`](../../../../packages/project/src/project_settings.rs#L187)

## Definition

```rust
pub enum ContextServerSettings
{
    Stdio{ .. },
    Http{ .. },
    Extension{ .. },
}
```

## Implementations

### default_extension()

```rust
pub fn default_extension() -> Self
```

Defined in: [`packages/project/src/project_settings.rs:325`](../../../../packages/project/src/project_settings.rs#L325)

#### Returns

`Self`


***

### enabled()

```rust
pub fn enabled(&self) -> bool
```

Defined in: [`packages/project/src/project_settings.rs:333`](../../../../packages/project/src/project_settings.rs#L333)

#### Returns

`bool`


***

### set_enabled()

```rust
pub fn set_enabled(&self, enabled: bool)
```

Defined in: [`packages/project/src/project_settings.rs:341`](../../../../packages/project/src/project_settings.rs#L341)

#### Parameters

##### enabled

`bool`

## Trait Implementations

- `impl Borrow for ContextServerSettings`
- `impl BorrowMut for ContextServerSettings`
- `impl CloneToUninit for ContextServerSettings`
- `impl Into for ContextServerSettings`
- `impl From for ContextServerSettings`
- `impl TryInto for ContextServerSettings`
- `impl TryFrom for ContextServerSettings`
- `impl Any for ContextServerSettings`
- `impl ToOwned for ContextServerSettings`
- `impl DeserializeOwned for ContextServerSettings`
- `impl Equivalent for ContextServerSettings`
- `impl Serialize for ContextServerSettings`
- `impl DynClone for ContextServerSettings`
- `impl VZip for ContextServerSettings`
- `impl CastableFrom for ContextServerSettings`
- `impl CastableFrom for ContextServerSettings`
- `impl Read for ContextServerSettings`
- `impl IntoEither for ContextServerSettings`
- `impl ErasedDestructor for ContextServerSettings`
- `impl Same for ContextServerSettings`
- `impl Pointable for ContextServerSettings`
- `impl Instrument for ContextServerSettings`
- `impl WithSubscriber for ContextServerSettings`
- `impl FromAngle for ContextServerSettings`
- `impl IntoAngle for ContextServerSettings`
- `impl IntoCam16Unclamped for ContextServerSettings`
- `impl Cam16IntoUnclamped for ContextServerSettings`
- `impl ArraysFrom for ContextServerSettings`
- `impl ArraysInto for ContextServerSettings`
- `impl ComponentsFrom for ContextServerSettings`
- `impl TryComponentsInto for ContextServerSettings`
- `impl UintsFrom for ContextServerSettings`
- `impl UintsInto for ContextServerSettings`
- `impl AdaptIntoUnclamped for ContextServerSettings`
- `impl AdaptInto for ContextServerSettings`
- `impl IntoColor for ContextServerSettings`
- `impl IntoColorUnclamped for ContextServerSettings`
- `impl TryIntoColor for ContextServerSettings`
- `impl FromStimulus for ContextServerSettings`
- `impl IntoStimulus for ContextServerSettings`
- `impl Equivalent for ContextServerSettings`
- `impl Deserialize for ContextServerSettings`
- `impl Serialize for ContextServerSettings`
- `impl Clone for ContextServerSettings`
- `impl StructuralPartialEq for ContextServerSettings`
- `impl PartialEq for ContextServerSettings`
- `impl Eq for ContextServerSettings`
- `impl JsonSchema for ContextServerSettings`
- `impl Debug for ContextServerSettings`
- `impl From for ContextServerSettings`
- `impl Into for ContextServerSettings`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

