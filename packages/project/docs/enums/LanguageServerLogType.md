---
id: LanguageServerLogType
title: LanguageServerLogType
---

# Enum: LanguageServerLogType

Defined in: [`packages/project/src/lsp_store/mod.rs:16017`](../../../../packages/project/src/lsp_store/mod.rs#L16017)

## Definition

```rust
pub enum LanguageServerLogType
{
    Log(MessageType),
    Trace{ .. },
    Rpc{ .. },
}
```

## Implementations

### to_proto()

```rust
pub fn to_proto(&self) -> LogType
```

Defined in: [`packages/project/src/lsp_store/mod.rs:16029`](../../../../packages/project/src/lsp_store/mod.rs#L16029)

#### Returns

`LogType`


***

### from_proto()

```rust
pub fn from_proto(log_type: LogType) -> Self
```

Defined in: [`packages/project/src/lsp_store/mod.rs:16069`](../../../../packages/project/src/lsp_store/mod.rs#L16069)

#### Parameters

##### log_type

`LogType`

#### Returns

`Self`

## Trait Implementations

- `impl Borrow for LanguageServerLogType`
- `impl BorrowMut for LanguageServerLogType`
- `impl CloneToUninit for LanguageServerLogType`
- `impl Into for LanguageServerLogType`
- `impl From for LanguageServerLogType`
- `impl TryInto for LanguageServerLogType`
- `impl TryFrom for LanguageServerLogType`
- `impl Any for LanguageServerLogType`
- `impl ToOwned for LanguageServerLogType`
- `impl DynClone for LanguageServerLogType`
- `impl VZip for LanguageServerLogType`
- `impl CastableFrom for LanguageServerLogType`
- `impl CastableFrom for LanguageServerLogType`
- `impl Read for LanguageServerLogType`
- `impl IntoEither for LanguageServerLogType`
- `impl ErasedDestructor for LanguageServerLogType`
- `impl Same for LanguageServerLogType`
- `impl Pointable for LanguageServerLogType`
- `impl Instrument for LanguageServerLogType`
- `impl WithSubscriber for LanguageServerLogType`
- `impl FromAngle for LanguageServerLogType`
- `impl IntoAngle for LanguageServerLogType`
- `impl IntoCam16Unclamped for LanguageServerLogType`
- `impl Cam16IntoUnclamped for LanguageServerLogType`
- `impl ArraysFrom for LanguageServerLogType`
- `impl ArraysInto for LanguageServerLogType`
- `impl ComponentsFrom for LanguageServerLogType`
- `impl TryComponentsInto for LanguageServerLogType`
- `impl UintsFrom for LanguageServerLogType`
- `impl UintsInto for LanguageServerLogType`
- `impl AdaptIntoUnclamped for LanguageServerLogType`
- `impl AdaptInto for LanguageServerLogType`
- `impl IntoColor for LanguageServerLogType`
- `impl IntoColorUnclamped for LanguageServerLogType`
- `impl TryIntoColor for LanguageServerLogType`
- `impl FromStimulus for LanguageServerLogType`
- `impl IntoStimulus for LanguageServerLogType`
- `impl Clone for LanguageServerLogType`
- `impl Debug for LanguageServerLogType`
- `impl StructuralPartialEq for LanguageServerLogType`
- `impl PartialEq for LanguageServerLogType`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

