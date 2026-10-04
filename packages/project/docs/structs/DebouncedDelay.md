---
id: DebouncedDelay
title: DebouncedDelay
---

# Struct: DebouncedDelay

Defined in: [`packages/project/src/debounced_delay.rs:5`](../../../../packages/project/src/debounced_delay.rs#L5)

## Definition

```rust
pub struct DebouncedDelay<E>
```

_（存在非公开字段）_

## Implementations

### new()

```rust
pub fn new() -> Self
```

Defined in: [`packages/project/src/debounced_delay.rs:18`](../../../../packages/project/src/debounced_delay.rs#L18)

#### Returns

`Self`


***

### fire_new()

```rust
pub fn fire_new<F>(&self, delay: Duration, cx: &Context<'_, E>, func: F)
where
    F: 'static + Send + FnOnce(&E, &Context<'_, E>) -> Task<()>
```

Defined in: [`packages/project/src/debounced_delay.rs:26`](../../../../packages/project/src/debounced_delay.rs#L26)

#### Parameters

##### delay

`Duration`

##### cx

`&Context<'_, E>`

##### func

`F`

## Trait Implementations

- `impl Borrow for DebouncedDelay`
- `impl BorrowMut for DebouncedDelay`
- `impl Into for DebouncedDelay`
- `impl From for DebouncedDelay`
- `impl TryInto for DebouncedDelay`
- `impl TryFrom for DebouncedDelay`
- `impl Any for DebouncedDelay`
- `impl VZip for DebouncedDelay`
- `impl CastableFrom for DebouncedDelay`
- `impl CastableFrom for DebouncedDelay`
- `impl Read for DebouncedDelay`
- `impl IntoEither for DebouncedDelay`
- `impl ErasedDestructor for DebouncedDelay`
- `impl Same for DebouncedDelay`
- `impl ReadPrimitive for DebouncedDelay`
- `impl Pointable for DebouncedDelay`
- `impl Instrument for DebouncedDelay`
- `impl WithSubscriber for DebouncedDelay`
- `impl FromAngle for DebouncedDelay`
- `impl IntoAngle for DebouncedDelay`
- `impl IntoCam16Unclamped for DebouncedDelay`
- `impl Cam16IntoUnclamped for DebouncedDelay`
- `impl ArraysFrom for DebouncedDelay`
- `impl ArraysInto for DebouncedDelay`
- `impl ComponentsFrom for DebouncedDelay`
- `impl TryComponentsInto for DebouncedDelay`
- `impl UintsFrom for DebouncedDelay`
- `impl UintsInto for DebouncedDelay`
- `impl AdaptIntoUnclamped for DebouncedDelay`
- `impl AdaptInto for DebouncedDelay`
- `impl IntoColor for DebouncedDelay`
- `impl IntoColorUnclamped for DebouncedDelay`
- `impl TryIntoColor for DebouncedDelay`
- `impl FromStimulus for DebouncedDelay`
- `impl IntoStimulus for DebouncedDelay`
- `impl Default for DebouncedDelay`

## Auto Trait Implementations

`Freeze` `Send` `Sync` `Unpin` `UnsafeUnpin`

