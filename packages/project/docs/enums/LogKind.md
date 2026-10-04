---
id: LogKind
title: LogKind
---

# Enum: LogKind

Defined in: [`packages/project/src/lsp_store/log_store.rs:456`](../../../../packages/project/src/lsp_store/log_store.rs#L456)

## Definition

```rust
pub enum LogKind {
    Rpc,
    Trace,
    Logs,
    ServerInfo,
}
```

## Implementations

### from_server_log_type()

```rust
pub fn from_server_log_type(log_type: &LanguageServerLogType) -> Self
```

Defined in: [`packages/project/src/lsp_store/log_store.rs:465`](../../../../packages/project/src/lsp_store/log_store.rs#L465)

#### Parameters

##### log_type

`&LanguageServerLogType`

#### Returns

`Self`

## Trait Implementations

- `impl Borrow for LogKind`
- `impl BorrowMut for LogKind`
- `impl CloneToUninit for LogKind`
- `impl Into for LogKind`
- `impl From for LogKind`
- `impl TryInto for LogKind`
- `impl TryFrom for LogKind`
- `impl Any for LogKind`
- `impl ToOwned for LogKind`
- `impl Equivalent for LogKind`
- `impl DynClone for LogKind`
- `impl VZip for LogKind`
- `impl CastableFrom for LogKind`
- `impl CastableFrom for LogKind`
- `impl Read for LogKind`
- `impl IntoEither for LogKind`
- `impl ErasedDestructor for LogKind`
- `impl Same for LogKind`
- `impl ReadPrimitive for LogKind`
- `impl Pointable for LogKind`
- `impl Instrument for LogKind`
- `impl WithSubscriber for LogKind`
- `impl FromAngle for LogKind`
- `impl IntoAngle for LogKind`
- `impl IntoCam16Unclamped for LogKind`
- `impl Cam16IntoUnclamped for LogKind`
- `impl ArraysFrom for LogKind`
- `impl ArraysInto for LogKind`
- `impl ComponentsFrom for LogKind`
- `impl TryComponentsInto for LogKind`
- `impl UintsFrom for LogKind`
- `impl UintsInto for LogKind`
- `impl AdaptIntoUnclamped for LogKind`
- `impl AdaptInto for LogKind`
- `impl IntoColor for LogKind`
- `impl IntoColorUnclamped for LogKind`
- `impl TryIntoColor for LogKind`
- `impl FromStimulus for LogKind`
- `impl IntoStimulus for LogKind`
- `impl Equivalent for LogKind`
- `impl ResetDiscriminant for LogKind`
- `impl Clone for LogKind`
- `impl Copy for LogKind`
- `impl Debug for LogKind`
- `impl Default for LogKind`
- `impl StructuralPartialEq for LogKind`
- `impl PartialEq for LogKind`
- `impl Eq for LogKind`
- `impl Hash for LogKind`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

