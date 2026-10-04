---
id: LspNotificationSettings
title: LspNotificationSettings
---

# Struct: LspNotificationSettings

Defined in: [`packages/project/src/project_settings.rs:169`](../../../../packages/project/src/project_settings.rs#L169)

## Definition

```rust
pub struct LspNotificationSettings
{
    // Timeout in milliseconds for automatically dismissing language server notifications. Set to 0 to disable auto-dismiss. Default: 5000
    pub dismiss_timeout_ms: Option<u64>,
}
```

## Fields

### dismiss_timeout_ms

Defined in: [`packages/project/src/project_settings.rs:174`](../../../../packages/project/src/project_settings.rs#L174)

Timeout in milliseconds for automatically dismissing language server notifications.
Set to 0 to disable auto-dismiss.

Default: 5000

## Trait Implementations

- `impl Borrow for LspNotificationSettings`
- `impl BorrowMut for LspNotificationSettings`
- `impl CloneToUninit for LspNotificationSettings`
- `impl Into for LspNotificationSettings`
- `impl From for LspNotificationSettings`
- `impl TryInto for LspNotificationSettings`
- `impl TryFrom for LspNotificationSettings`
- `impl Any for LspNotificationSettings`
- `impl ToOwned for LspNotificationSettings`
- `impl DeserializeOwned for LspNotificationSettings`
- `impl Equivalent for LspNotificationSettings`
- `impl Serialize for LspNotificationSettings`
- `impl DynClone for LspNotificationSettings`
- `impl VZip for LspNotificationSettings`
- `impl CastableFrom for LspNotificationSettings`
- `impl CastableFrom for LspNotificationSettings`
- `impl Read for LspNotificationSettings`
- `impl IntoEither for LspNotificationSettings`
- `impl ErasedDestructor for LspNotificationSettings`
- `impl Same for LspNotificationSettings`
- `impl ReadPrimitive for LspNotificationSettings`
- `impl Pointable for LspNotificationSettings`
- `impl Instrument for LspNotificationSettings`
- `impl WithSubscriber for LspNotificationSettings`
- `impl FromAngle for LspNotificationSettings`
- `impl IntoAngle for LspNotificationSettings`
- `impl IntoCam16Unclamped for LspNotificationSettings`
- `impl Cam16IntoUnclamped for LspNotificationSettings`
- `impl ArraysFrom for LspNotificationSettings`
- `impl ArraysInto for LspNotificationSettings`
- `impl ComponentsFrom for LspNotificationSettings`
- `impl TryComponentsInto for LspNotificationSettings`
- `impl UintsFrom for LspNotificationSettings`
- `impl UintsInto for LspNotificationSettings`
- `impl AdaptIntoUnclamped for LspNotificationSettings`
- `impl AdaptInto for LspNotificationSettings`
- `impl IntoColor for LspNotificationSettings`
- `impl IntoColorUnclamped for LspNotificationSettings`
- `impl TryIntoColor for LspNotificationSettings`
- `impl FromStimulus for LspNotificationSettings`
- `impl IntoStimulus for LspNotificationSettings`
- `impl Equivalent for LspNotificationSettings`
- `impl Deserialize for LspNotificationSettings`
- `impl Serialize for LspNotificationSettings`
- `impl Clone for LspNotificationSettings`
- `impl StructuralPartialEq for LspNotificationSettings`
- `impl PartialEq for LspNotificationSettings`
- `impl Eq for LspNotificationSettings`
- `impl JsonSchema for LspNotificationSettings`
- `impl Debug for LspNotificationSettings`
- `impl Default for LspNotificationSettings`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

