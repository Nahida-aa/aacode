---
id: ThreadStatus
title: ThreadStatus
---

# Enum: ThreadStatus

Defined in: [`packages/project/src/debugger/session.rs:99`](../../../../packages/project/src/debugger/session.rs#L99)

## Definition

```rust
pub enum ThreadStatus {
    Running,
    Stopped,
    Stepping,
    Exited,
    Ended,
}
```

## Implementations

### label()

```rust
pub fn label(&self) -> &'static str
```

Defined in: [`packages/project/src/debugger/session.rs:109`](../../../../packages/project/src/debugger/session.rs#L109)

#### Returns

`&'static str`

## Trait Implementations

- `impl Borrow for ThreadStatus`
- `impl BorrowMut for ThreadStatus`
- `impl CloneToUninit for ThreadStatus`
- `impl Into for ThreadStatus`
- `impl From for ThreadStatus`
- `impl TryInto for ThreadStatus`
- `impl TryFrom for ThreadStatus`
- `impl Any for ThreadStatus`
- `impl ToOwned for ThreadStatus`
- `impl Equivalent for ThreadStatus`
- `impl DynClone for ThreadStatus`
- `impl VZip for ThreadStatus`
- `impl CastableFrom for ThreadStatus`
- `impl CastableFrom for ThreadStatus`
- `impl Read for ThreadStatus`
- `impl IntoEither for ThreadStatus`
- `impl ErasedDestructor for ThreadStatus`
- `impl Same for ThreadStatus`
- `impl ReadPrimitive for ThreadStatus`
- `impl Pointable for ThreadStatus`
- `impl Instrument for ThreadStatus`
- `impl WithSubscriber for ThreadStatus`
- `impl FromAngle for ThreadStatus`
- `impl IntoAngle for ThreadStatus`
- `impl IntoCam16Unclamped for ThreadStatus`
- `impl Cam16IntoUnclamped for ThreadStatus`
- `impl ArraysFrom for ThreadStatus`
- `impl ArraysInto for ThreadStatus`
- `impl ComponentsFrom for ThreadStatus`
- `impl TryComponentsInto for ThreadStatus`
- `impl UintsFrom for ThreadStatus`
- `impl UintsInto for ThreadStatus`
- `impl AdaptIntoUnclamped for ThreadStatus`
- `impl AdaptInto for ThreadStatus`
- `impl IntoColor for ThreadStatus`
- `impl IntoColorUnclamped for ThreadStatus`
- `impl TryIntoColor for ThreadStatus`
- `impl FromStimulus for ThreadStatus`
- `impl IntoStimulus for ThreadStatus`
- `impl Equivalent for ThreadStatus`
- `impl ResetDiscriminant for ThreadStatus`
- `impl Copy for ThreadStatus`
- `impl Clone for ThreadStatus`
- `impl Debug for ThreadStatus`
- `impl Default for ThreadStatus`
- `impl StructuralPartialEq for ThreadStatus`
- `impl PartialEq for ThreadStatus`
- `impl Eq for ThreadStatus`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

