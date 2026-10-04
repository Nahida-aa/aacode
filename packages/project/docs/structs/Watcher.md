---
id: Watcher
title: Watcher
---

# Struct: Watcher

Defined in: [`packages/project/src/debugger/session.rs:140`](../../../../packages/project/src/debugger/session.rs#L140)

## Definition

```rust
pub struct Watcher
{
    pub expression: SharedString,
    pub value: SharedString,
    pub variables_reference: u64,
    pub presentation_hint: Option<VariablePresentationHint>,
}
```

## Trait Implementations

- `impl Borrow for Watcher`
- `impl BorrowMut for Watcher`
- `impl CloneToUninit for Watcher`
- `impl Into for Watcher`
- `impl From for Watcher`
- `impl TryInto for Watcher`
- `impl TryFrom for Watcher`
- `impl Any for Watcher`
- `impl ToOwned for Watcher`
- `impl DynClone for Watcher`
- `impl VZip for Watcher`
- `impl CastableFrom for Watcher`
- `impl CastableFrom for Watcher`
- `impl Read for Watcher`
- `impl IntoEither for Watcher`
- `impl ErasedDestructor for Watcher`
- `impl Same for Watcher`
- `impl Pointable for Watcher`
- `impl Instrument for Watcher`
- `impl WithSubscriber for Watcher`
- `impl FromAngle for Watcher`
- `impl IntoAngle for Watcher`
- `impl IntoCam16Unclamped for Watcher`
- `impl Cam16IntoUnclamped for Watcher`
- `impl ArraysFrom for Watcher`
- `impl ArraysInto for Watcher`
- `impl ComponentsFrom for Watcher`
- `impl TryComponentsInto for Watcher`
- `impl UintsFrom for Watcher`
- `impl UintsInto for Watcher`
- `impl AdaptIntoUnclamped for Watcher`
- `impl AdaptInto for Watcher`
- `impl IntoColor for Watcher`
- `impl IntoColorUnclamped for Watcher`
- `impl TryIntoColor for Watcher`
- `impl FromStimulus for Watcher`
- `impl IntoStimulus for Watcher`
- `impl Debug for Watcher`
- `impl Clone for Watcher`
- `impl StructuralPartialEq for Watcher`
- `impl PartialEq for Watcher`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

