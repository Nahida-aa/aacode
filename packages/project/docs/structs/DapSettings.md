---
id: DapSettings
title: DapSettings
---

# Struct: DapSettings

Defined in: [`packages/project/src/project_settings.rs:1611`](../../../../packages/project/src/project_settings.rs#L1611)

## Definition

```rust
pub struct DapSettings
{
    pub binary: DapBinary,
    pub args: Option<Vec<String>>,
    pub env: Option<HashMap<String, String>>,
}
```

## Trait Implementations

- `impl Borrow for DapSettings`
- `impl BorrowMut for DapSettings`
- `impl CloneToUninit for DapSettings`
- `impl Into for DapSettings`
- `impl From for DapSettings`
- `impl TryInto for DapSettings`
- `impl TryFrom for DapSettings`
- `impl Any for DapSettings`
- `impl ToOwned for DapSettings`
- `impl DynClone for DapSettings`
- `impl VZip for DapSettings`
- `impl CastableFrom for DapSettings`
- `impl CastableFrom for DapSettings`
- `impl Read for DapSettings`
- `impl IntoEither for DapSettings`
- `impl ErasedDestructor for DapSettings`
- `impl Same for DapSettings`
- `impl Pointable for DapSettings`
- `impl Instrument for DapSettings`
- `impl WithSubscriber for DapSettings`
- `impl FromAngle for DapSettings`
- `impl IntoAngle for DapSettings`
- `impl IntoCam16Unclamped for DapSettings`
- `impl Cam16IntoUnclamped for DapSettings`
- `impl ArraysFrom for DapSettings`
- `impl ArraysInto for DapSettings`
- `impl ComponentsFrom for DapSettings`
- `impl TryComponentsInto for DapSettings`
- `impl UintsFrom for DapSettings`
- `impl UintsInto for DapSettings`
- `impl AdaptIntoUnclamped for DapSettings`
- `impl AdaptInto for DapSettings`
- `impl IntoColor for DapSettings`
- `impl IntoColorUnclamped for DapSettings`
- `impl TryIntoColor for DapSettings`
- `impl FromStimulus for DapSettings`
- `impl IntoStimulus for DapSettings`
- `impl Debug for DapSettings`
- `impl Clone for DapSettings`
- `impl From for DapSettings`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

