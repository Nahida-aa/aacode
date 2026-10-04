---
id: BreakpointState
title: BreakpointState
---

# Enum: BreakpointState

Defined in: [`packages/project/src/debugger/breakpoint_store.rs:936`](../../../../packages/project/src/debugger/breakpoint_store.rs#L936)

## Definition

```rust
pub enum BreakpointState {
    Enabled,
    Disabled,
}
```

## Implementations

### is_enabled()

```rust
pub fn is_enabled(&self) -> bool
```

Defined in: [`packages/project/src/debugger/breakpoint_store.rs:943`](../../../../packages/project/src/debugger/breakpoint_store.rs#L943)

#### Returns

`bool`


***

### is_disabled()

```rust
pub fn is_disabled(&self) -> bool
```

Defined in: [`packages/project/src/debugger/breakpoint_store.rs:948`](../../../../packages/project/src/debugger/breakpoint_store.rs#L948)

#### Returns

`bool`


***

### to_int()

```rust
pub fn to_int(self) -> i32
```

Defined in: [`packages/project/src/debugger/breakpoint_store.rs:953`](../../../../packages/project/src/debugger/breakpoint_store.rs#L953)

#### Returns

`i32`

## Trait Implementations

- `impl Borrow for BreakpointState`
- `impl BorrowMut for BreakpointState`
- `impl CloneToUninit for BreakpointState`
- `impl Into for BreakpointState`
- `impl From for BreakpointState`
- `impl TryInto for BreakpointState`
- `impl TryFrom for BreakpointState`
- `impl Any for BreakpointState`
- `impl ToOwned for BreakpointState`
- `impl Equivalent for BreakpointState`
- `impl DynClone for BreakpointState`
- `impl VZip for BreakpointState`
- `impl CastableFrom for BreakpointState`
- `impl CastableFrom for BreakpointState`
- `impl Read for BreakpointState`
- `impl IntoEither for BreakpointState`
- `impl ErasedDestructor for BreakpointState`
- `impl Same for BreakpointState`
- `impl Pointable for BreakpointState`
- `impl Instrument for BreakpointState`
- `impl WithSubscriber for BreakpointState`
- `impl FromAngle for BreakpointState`
- `impl IntoAngle for BreakpointState`
- `impl IntoCam16Unclamped for BreakpointState`
- `impl Cam16IntoUnclamped for BreakpointState`
- `impl ArraysFrom for BreakpointState`
- `impl ArraysInto for BreakpointState`
- `impl ComponentsFrom for BreakpointState`
- `impl TryComponentsInto for BreakpointState`
- `impl UintsFrom for BreakpointState`
- `impl UintsInto for BreakpointState`
- `impl AdaptIntoUnclamped for BreakpointState`
- `impl AdaptInto for BreakpointState`
- `impl IntoColor for BreakpointState`
- `impl IntoColorUnclamped for BreakpointState`
- `impl TryIntoColor for BreakpointState`
- `impl FromStimulus for BreakpointState`
- `impl IntoStimulus for BreakpointState`
- `impl Equivalent for BreakpointState`
- `impl ResetDiscriminant for BreakpointState`
- `impl Copy for BreakpointState`
- `impl Clone for BreakpointState`
- `impl Debug for BreakpointState`
- `impl Hash for BreakpointState`
- `impl StructuralPartialEq for BreakpointState`
- `impl PartialEq for BreakpointState`
- `impl Eq for BreakpointState`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

