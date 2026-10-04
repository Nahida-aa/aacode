---
id: Breakpoint
title: Breakpoint
---

# Struct: Breakpoint

Defined in: [`packages/project/src/debugger/breakpoint_store.rs:962`](../../../../packages/project/src/debugger/breakpoint_store.rs#L962)

## Definition

```rust
pub struct Breakpoint {
    pub message: Option<Arc<str>>,
    // How many times do we hit the breakpoint until we actually stop at it e.g. (2 = 2 times of the breakpoint action)
    pub hit_condition: Option<Arc<str>>,
    pub condition: Option<Arc<str>>,
    pub state: BreakpointState,
}
```

## Fields

### hit_condition

Defined in: [`packages/project/src/debugger/breakpoint_store.rs:965`](../../../../packages/project/src/debugger/breakpoint_store.rs#L965)

How many times do we hit the breakpoint until we actually stop at it e.g. (2 = 2 times of the breakpoint action)

## Implementations

### new_standard()

```rust
pub fn new_standard() -> Self
```

Defined in: [`packages/project/src/debugger/breakpoint_store.rs:971`](../../../../packages/project/src/debugger/breakpoint_store.rs#L971)

#### Returns

`Self`


***

### new_condition()

```rust
pub fn new_condition(hit_condition: &str) -> Self
```

Defined in: [`packages/project/src/debugger/breakpoint_store.rs:980`](../../../../packages/project/src/debugger/breakpoint_store.rs#L980)

#### Parameters

##### hit_condition

`&str`

#### Returns

`Self`


***

### new_log()

```rust
pub fn new_log(log_message: &str) -> Self
```

Defined in: [`packages/project/src/debugger/breakpoint_store.rs:989`](../../../../packages/project/src/debugger/breakpoint_store.rs#L989)

#### Parameters

##### log_message

`&str`

#### Returns

`Self`


***

### is_enabled()

```rust
pub fn is_enabled(&self) -> bool
```

Defined in: [`packages/project/src/debugger/breakpoint_store.rs:1044`](../../../../packages/project/src/debugger/breakpoint_store.rs#L1044)

#### Returns

`bool`


***

### is_disabled()

```rust
pub fn is_disabled(&self) -> bool
```

Defined in: [`packages/project/src/debugger/breakpoint_store.rs:1049`](../../../../packages/project/src/debugger/breakpoint_store.rs#L1049)

#### Returns

`bool`

## Trait Implementations

- `impl Borrow for Breakpoint`
- `impl BorrowMut for Breakpoint`
- `impl CloneToUninit for Breakpoint`
- `impl Into for Breakpoint`
- `impl From for Breakpoint`
- `impl TryInto for Breakpoint`
- `impl TryFrom for Breakpoint`
- `impl Any for Breakpoint`
- `impl ToOwned for Breakpoint`
- `impl Equivalent for Breakpoint`
- `impl DynClone for Breakpoint`
- `impl VZip for Breakpoint`
- `impl CastableFrom for Breakpoint`
- `impl CastableFrom for Breakpoint`
- `impl Read for Breakpoint`
- `impl IntoEither for Breakpoint`
- `impl ErasedDestructor for Breakpoint`
- `impl Same for Breakpoint`
- `impl Pointable for Breakpoint`
- `impl Instrument for Breakpoint`
- `impl WithSubscriber for Breakpoint`
- `impl FromAngle for Breakpoint`
- `impl IntoAngle for Breakpoint`
- `impl IntoCam16Unclamped for Breakpoint`
- `impl Cam16IntoUnclamped for Breakpoint`
- `impl ArraysFrom for Breakpoint`
- `impl ArraysInto for Breakpoint`
- `impl ComponentsFrom for Breakpoint`
- `impl TryComponentsInto for Breakpoint`
- `impl UintsFrom for Breakpoint`
- `impl UintsInto for Breakpoint`
- `impl AdaptIntoUnclamped for Breakpoint`
- `impl AdaptInto for Breakpoint`
- `impl IntoColor for Breakpoint`
- `impl IntoColorUnclamped for Breakpoint`
- `impl TryIntoColor for Breakpoint`
- `impl FromStimulus for Breakpoint`
- `impl IntoStimulus for Breakpoint`
- `impl Equivalent for Breakpoint`
- `impl Clone for Breakpoint`
- `impl Debug for Breakpoint`
- `impl Hash for Breakpoint`
- `impl StructuralPartialEq for Breakpoint`
- `impl PartialEq for Breakpoint`
- `impl Eq for Breakpoint`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

