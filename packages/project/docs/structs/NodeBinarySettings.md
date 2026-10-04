---
id: NodeBinarySettings
title: NodeBinarySettings
---

# Struct: NodeBinarySettings

Defined in: [`packages/project/src/project_settings.rs:102`](../../../../packages/project/src/project_settings.rs#L102)

## Definition

```rust
pub struct NodeBinarySettings
{
    // The path to the Node binary.
    pub path: Option<String>,
    // The path to the npm binary Zed should use (defaults to `.path/../npm`).
    pub npm_path: Option<String>,
    // If enabled, Zed will download its own copy of Node.
    pub ignore_system_version: bool,
}
```

## Fields

### path

Defined in: [`packages/project/src/project_settings.rs:104`](../../../../packages/project/src/project_settings.rs#L104)

The path to the Node binary.


***

### npm_path

Defined in: [`packages/project/src/project_settings.rs:106`](../../../../packages/project/src/project_settings.rs#L106)

The path to the npm binary Zed should use (defaults to `.path/../npm`).


***

### ignore_system_version

Defined in: [`packages/project/src/project_settings.rs:108`](../../../../packages/project/src/project_settings.rs#L108)

If enabled, Zed will download its own copy of Node.

## Trait Implementations

- `impl Borrow for NodeBinarySettings`
- `impl BorrowMut for NodeBinarySettings`
- `impl CloneToUninit for NodeBinarySettings`
- `impl Into for NodeBinarySettings`
- `impl From for NodeBinarySettings`
- `impl TryInto for NodeBinarySettings`
- `impl TryFrom for NodeBinarySettings`
- `impl Any for NodeBinarySettings`
- `impl ToOwned for NodeBinarySettings`
- `impl DeserializeOwned for NodeBinarySettings`
- `impl Serialize for NodeBinarySettings`
- `impl DynClone for NodeBinarySettings`
- `impl VZip for NodeBinarySettings`
- `impl CastableFrom for NodeBinarySettings`
- `impl CastableFrom for NodeBinarySettings`
- `impl Read for NodeBinarySettings`
- `impl IntoEither for NodeBinarySettings`
- `impl ErasedDestructor for NodeBinarySettings`
- `impl Same for NodeBinarySettings`
- `impl ReadPrimitive for NodeBinarySettings`
- `impl Pointable for NodeBinarySettings`
- `impl Instrument for NodeBinarySettings`
- `impl WithSubscriber for NodeBinarySettings`
- `impl FromAngle for NodeBinarySettings`
- `impl IntoAngle for NodeBinarySettings`
- `impl IntoCam16Unclamped for NodeBinarySettings`
- `impl Cam16IntoUnclamped for NodeBinarySettings`
- `impl ArraysFrom for NodeBinarySettings`
- `impl ArraysInto for NodeBinarySettings`
- `impl ComponentsFrom for NodeBinarySettings`
- `impl TryComponentsInto for NodeBinarySettings`
- `impl UintsFrom for NodeBinarySettings`
- `impl UintsInto for NodeBinarySettings`
- `impl AdaptIntoUnclamped for NodeBinarySettings`
- `impl AdaptInto for NodeBinarySettings`
- `impl IntoColor for NodeBinarySettings`
- `impl IntoColorUnclamped for NodeBinarySettings`
- `impl TryIntoColor for NodeBinarySettings`
- `impl FromStimulus for NodeBinarySettings`
- `impl IntoStimulus for NodeBinarySettings`
- `impl Debug for NodeBinarySettings`
- `impl Clone for NodeBinarySettings`
- `impl Default for NodeBinarySettings`
- `impl StructuralPartialEq for NodeBinarySettings`
- `impl PartialEq for NodeBinarySettings`
- `impl Serialize for NodeBinarySettings`
- `impl Deserialize for NodeBinarySettings`
- `impl JsonSchema for NodeBinarySettings`
- `impl From for NodeBinarySettings`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

