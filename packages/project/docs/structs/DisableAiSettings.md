---
id: DisableAiSettings
title: DisableAiSettings
---

# Struct: DisableAiSettings

Defined in: [`packages/project/src/settings.rs:9`](../../../../packages/project/src/settings.rs#L9)

AI 功能的全局开关（对齐 Zed `project::DisableAiSettings`）。

## Definition

```rust
pub struct DisableAiSettings
{
    pub disable_ai: bool,
}
```

## Implementations

### is_ai_disabled_for_buffer()

```rust
pub fn is_ai_disabled_for_buffer(buffer: Option<&Entity<Buffer>>, cx: &App) -> bool
```

Defined in: [`packages/project/src/settings.rs:26`](../../../../packages/project/src/settings.rs#L26)

Returns whether AI is disabled for the given buffer。

按 buffer 所在文件定位 worktree + path，读取该位置的 DisableAiSettings。
对齐 Zed `project::DisableAiSettings::is_ai_disabled_for_buffer`。

#### Parameters

##### buffer

`Option<&Entity<Buffer>>`

##### cx

`&App`

#### Returns

`bool`


***

### is_ai_disabled_for_file()

```rust
pub fn is_ai_disabled_for_file(file: Option<&Arc<dyn File>>, cx: &App) -> bool
```

Defined in: [`packages/project/src/settings.rs:33`](../../../../packages/project/src/settings.rs#L33)

#### Parameters

##### file

`Option<&Arc<dyn File>>`

##### cx

`&App`

#### Returns

`bool`

## Trait Implementations

- `impl Borrow for DisableAiSettings`
- `impl BorrowMut for DisableAiSettings`
- `impl CloneToUninit for DisableAiSettings`
- `impl Into for DisableAiSettings`
- `impl From for DisableAiSettings`
- `impl TryInto for DisableAiSettings`
- `impl TryFrom for DisableAiSettings`
- `impl Any for DisableAiSettings`
- `impl ToOwned for DisableAiSettings`
- `impl DynClone for DisableAiSettings`
- `impl VZip for DisableAiSettings`
- `impl CastableFrom for DisableAiSettings`
- `impl CastableFrom for DisableAiSettings`
- `impl Read for DisableAiSettings`
- `impl IntoEither for DisableAiSettings`
- `impl ErasedDestructor for DisableAiSettings`
- `impl Same for DisableAiSettings`
- `impl Pointable for DisableAiSettings`
- `impl Instrument for DisableAiSettings`
- `impl WithSubscriber for DisableAiSettings`
- `impl FromAngle for DisableAiSettings`
- `impl IntoAngle for DisableAiSettings`
- `impl IntoCam16Unclamped for DisableAiSettings`
- `impl Cam16IntoUnclamped for DisableAiSettings`
- `impl ArraysFrom for DisableAiSettings`
- `impl ArraysInto for DisableAiSettings`
- `impl ComponentsFrom for DisableAiSettings`
- `impl TryComponentsInto for DisableAiSettings`
- `impl UintsFrom for DisableAiSettings`
- `impl UintsInto for DisableAiSettings`
- `impl AdaptIntoUnclamped for DisableAiSettings`
- `impl AdaptInto for DisableAiSettings`
- `impl IntoColor for DisableAiSettings`
- `impl IntoColorUnclamped for DisableAiSettings`
- `impl TryIntoColor for DisableAiSettings`
- `impl FromStimulus for DisableAiSettings`
- `impl IntoStimulus for DisableAiSettings`
- `impl ResetDiscriminant for DisableAiSettings`
- `impl Copy for DisableAiSettings`
- `impl Clone for DisableAiSettings`
- `impl Debug for DisableAiSettings`
- `impl Settings for DisableAiSettings`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

