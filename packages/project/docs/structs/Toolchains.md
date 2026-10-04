---
id: Toolchains
title: Toolchains
---

# Struct: Toolchains

Defined in: [`packages/project/src/toolchain_store.rs:44`](../../../../packages/project/src/toolchain_store.rs#L44)

## Definition

```rust
pub struct Toolchains
{
    // Auto-detected toolchains.
    pub toolchains: ToolchainList,
    // Path of the project root at which we ran the automatic toolchain detection.
    pub root_path: Arc<RelPath>,
    pub user_toolchains: BTreeMap<ToolchainScope, IndexSet<Toolchain>>,
}
```

## Fields

### toolchains

Defined in: [`packages/project/src/toolchain_store.rs:46`](../../../../packages/project/src/toolchain_store.rs#L46)

Auto-detected toolchains.


***

### root_path

Defined in: [`packages/project/src/toolchain_store.rs:48`](../../../../packages/project/src/toolchain_store.rs#L48)

Path of the project root at which we ran the automatic toolchain detection.

## Trait Implementations

- `impl Borrow for Toolchains`
- `impl BorrowMut for Toolchains`
- `impl Into for Toolchains`
- `impl From for Toolchains`
- `impl TryInto for Toolchains`
- `impl TryFrom for Toolchains`
- `impl Any for Toolchains`
- `impl VZip for Toolchains`
- `impl CastableFrom for Toolchains`
- `impl CastableFrom for Toolchains`
- `impl Read for Toolchains`
- `impl IntoEither for Toolchains`
- `impl ErasedDestructor for Toolchains`
- `impl Same for Toolchains`
- `impl Pointable for Toolchains`
- `impl Instrument for Toolchains`
- `impl WithSubscriber for Toolchains`
- `impl FromAngle for Toolchains`
- `impl IntoAngle for Toolchains`
- `impl IntoCam16Unclamped for Toolchains`
- `impl Cam16IntoUnclamped for Toolchains`
- `impl ArraysFrom for Toolchains`
- `impl ArraysInto for Toolchains`
- `impl ComponentsFrom for Toolchains`
- `impl TryComponentsInto for Toolchains`
- `impl UintsFrom for Toolchains`
- `impl UintsInto for Toolchains`
- `impl AdaptIntoUnclamped for Toolchains`
- `impl AdaptInto for Toolchains`
- `impl IntoColor for Toolchains`
- `impl IntoColorUnclamped for Toolchains`
- `impl TryIntoColor for Toolchains`
- `impl FromStimulus for Toolchains`
- `impl IntoStimulus for Toolchains`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

