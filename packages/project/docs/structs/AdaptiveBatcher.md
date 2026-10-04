---
id: AdaptiveBatcher
title: AdaptiveBatcher
---

# Struct: AdaptiveBatcher

Defined in: [`packages/project/src/project_search.rs:1082`](../../../../packages/project/src/project_search.rs#L1082)

Adaptive batcher that starts eager (small batches) and grows batch size
when items arrive quickly, reducing RPC overhead while preserving low latency
for slow streams.

## Definition

```rust
pub struct AdaptiveBatcher<T>
```

_（存在非公开字段）_

## Implementations

### new()

```rust
pub fn new(cx: &BackgroundExecutor) -> (Self, Receiver<Vec<T>>)
```

Defined in: [`packages/project/src/project_search.rs:1089`](../../../../packages/project/src/project_search.rs#L1089)

#### Parameters

##### cx

`&BackgroundExecutor`

#### Returns

`(Self, Receiver<Vec<T>>)`


***

### push()

```rust
pub async fn push(&self, item: T)
```

Defined in: [`packages/project/src/project_search.rs:1166`](../../../../packages/project/src/project_search.rs#L1166)

#### Parameters

##### item

`T`


***

### flush()

```rust
pub async fn flush(self)
```

Defined in: [`packages/project/src/project_search.rs:1170`](../../../../packages/project/src/project_search.rs#L1170)

## Trait Implementations

- `impl Borrow for AdaptiveBatcher`
- `impl BorrowMut for AdaptiveBatcher`
- `impl Into for AdaptiveBatcher`
- `impl From for AdaptiveBatcher`
- `impl TryInto for AdaptiveBatcher`
- `impl TryFrom for AdaptiveBatcher`
- `impl Any for AdaptiveBatcher`
- `impl VZip for AdaptiveBatcher`
- `impl CastableFrom for AdaptiveBatcher`
- `impl CastableFrom for AdaptiveBatcher`
- `impl Read for AdaptiveBatcher`
- `impl IntoEither for AdaptiveBatcher`
- `impl ErasedDestructor for AdaptiveBatcher`
- `impl Same for AdaptiveBatcher`
- `impl Pointable for AdaptiveBatcher`
- `impl Instrument for AdaptiveBatcher`
- `impl WithSubscriber for AdaptiveBatcher`
- `impl FromAngle for AdaptiveBatcher`
- `impl IntoAngle for AdaptiveBatcher`
- `impl IntoCam16Unclamped for AdaptiveBatcher`
- `impl Cam16IntoUnclamped for AdaptiveBatcher`
- `impl ArraysFrom for AdaptiveBatcher`
- `impl ArraysInto for AdaptiveBatcher`
- `impl ComponentsFrom for AdaptiveBatcher`
- `impl TryComponentsInto for AdaptiveBatcher`
- `impl UintsFrom for AdaptiveBatcher`
- `impl UintsInto for AdaptiveBatcher`
- `impl AdaptIntoUnclamped for AdaptiveBatcher`
- `impl AdaptInto for AdaptiveBatcher`
- `impl IntoColor for AdaptiveBatcher`
- `impl IntoColorUnclamped for AdaptiveBatcher`
- `impl TryIntoColor for AdaptiveBatcher`
- `impl FromStimulus for AdaptiveBatcher`
- `impl IntoStimulus for AdaptiveBatcher`

## Auto Trait Implementations

`Freeze` `Send` `Sync` `Unpin` `UnsafeUnpin`

