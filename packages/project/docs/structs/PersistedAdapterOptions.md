---
id: PersistedAdapterOptions
title: PersistedAdapterOptions
---

# Struct: PersistedAdapterOptions

Defined in: [`packages/project/src/debugger/dap_store.rs:113`](../../../../packages/project/src/debugger/dap_store.rs#L113)

Represents best-effort serialization of adapter state during last session (e.g. watches)

## Definition

```rust
pub struct PersistedAdapterOptions {
    // Which exception breakpoints were enabled during the last session with this adapter?
    pub exception_breakpoints: BTreeMap<String, PersistedExceptionBreakpoint>,
}
```

## Fields

### exception_breakpoints

Defined in: [`packages/project/src/debugger/dap_store.rs:115`](../../../../packages/project/src/debugger/dap_store.rs#L115)

Which exception breakpoints were enabled during the last session with this adapter?

## Trait Implementations

- `impl Borrow for PersistedAdapterOptions`
- `impl BorrowMut for PersistedAdapterOptions`
- `impl CloneToUninit for PersistedAdapterOptions`
- `impl Into for PersistedAdapterOptions`
- `impl From for PersistedAdapterOptions`
- `impl TryInto for PersistedAdapterOptions`
- `impl TryFrom for PersistedAdapterOptions`
- `impl Any for PersistedAdapterOptions`
- `impl ToOwned for PersistedAdapterOptions`
- `impl DeserializeOwned for PersistedAdapterOptions`
- `impl Serialize for PersistedAdapterOptions`
- `impl DynClone for PersistedAdapterOptions`
- `impl VZip for PersistedAdapterOptions`
- `impl CastableFrom for PersistedAdapterOptions`
- `impl CastableFrom for PersistedAdapterOptions`
- `impl Read for PersistedAdapterOptions`
- `impl IntoEither for PersistedAdapterOptions`
- `impl ErasedDestructor for PersistedAdapterOptions`
- `impl Same for PersistedAdapterOptions`
- `impl ReadPrimitive for PersistedAdapterOptions`
- `impl Pointable for PersistedAdapterOptions`
- `impl Instrument for PersistedAdapterOptions`
- `impl WithSubscriber for PersistedAdapterOptions`
- `impl FromAngle for PersistedAdapterOptions`
- `impl IntoAngle for PersistedAdapterOptions`
- `impl IntoCam16Unclamped for PersistedAdapterOptions`
- `impl Cam16IntoUnclamped for PersistedAdapterOptions`
- `impl ArraysFrom for PersistedAdapterOptions`
- `impl ArraysInto for PersistedAdapterOptions`
- `impl ComponentsFrom for PersistedAdapterOptions`
- `impl TryComponentsInto for PersistedAdapterOptions`
- `impl UintsFrom for PersistedAdapterOptions`
- `impl UintsInto for PersistedAdapterOptions`
- `impl AdaptIntoUnclamped for PersistedAdapterOptions`
- `impl AdaptInto for PersistedAdapterOptions`
- `impl IntoColor for PersistedAdapterOptions`
- `impl IntoColorUnclamped for PersistedAdapterOptions`
- `impl TryIntoColor for PersistedAdapterOptions`
- `impl FromStimulus for PersistedAdapterOptions`
- `impl IntoStimulus for PersistedAdapterOptions`
- `impl Clone for PersistedAdapterOptions`
- `impl Default for PersistedAdapterOptions`
- `impl Serialize for PersistedAdapterOptions`
- `impl Deserialize for PersistedAdapterOptions`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

