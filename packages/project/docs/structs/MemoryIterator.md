---
id: MemoryIterator
title: MemoryIterator
---

# Struct: MemoryIterator

Defined in: [`packages/project/src/debugger/memory.rs:268`](../../../../packages/project/src/debugger/memory.rs#L268)

Defines an iteration over a range of memory. Some of this memory might be unmapped or straight up missing.
Thus, this iterator alternates between synthesizing values and yielding known memory.

## Definition

```rust
pub struct MemoryIterator
```

_（存在非公开字段）_

## Implementations

### new()

```rust
pub fn new(range: RangeInclusive<u64>, pages: IntoIter<(PageAddress, PageContents)>) -> Self
```

Defined in: [`packages/project/src/debugger/memory.rs:276`](../../../../packages/project/src/debugger/memory.rs#L276)

#### Parameters

##### range

`RangeInclusive<u64>`

##### pages

`IntoIter<(PageAddress, PageContents)>`

#### Returns

`Self`

## Trait Implementations

- `impl Borrow for MemoryIterator`
- `impl BorrowMut for MemoryIterator`
- `impl Into for MemoryIterator`
- `impl From for MemoryIterator`
- `impl TryInto for MemoryIterator`
- `impl TryFrom for MemoryIterator`
- `impl Any for MemoryIterator`
- `impl IntoIterator for MemoryIterator`
- `impl IteratorRandom for MemoryIterator`
- `impl VZip for MemoryIterator`
- `impl CastableFrom for MemoryIterator`
- `impl CastableFrom for MemoryIterator`
- `impl Read for MemoryIterator`
- `impl Itertools for MemoryIterator`
- `impl IntoEither for MemoryIterator`
- `impl ErasedDestructor for MemoryIterator`
- `impl Same for MemoryIterator`
- `impl TakeUntilExt for MemoryIterator`
- `impl Pointable for MemoryIterator`
- `impl Instrument for MemoryIterator`
- `impl WithSubscriber for MemoryIterator`
- `impl IntoStreamingIterator for MemoryIterator`
- `impl TryCollect for MemoryIterator`
- `impl IteratorRandom for MemoryIterator`
- `impl FromAngle for MemoryIterator`
- `impl IntoAngle for MemoryIterator`
- `impl IntoCam16Unclamped for MemoryIterator`
- `impl Cam16IntoUnclamped for MemoryIterator`
- `impl ArraysFrom for MemoryIterator`
- `impl ArraysInto for MemoryIterator`
- `impl ComponentsFrom for MemoryIterator`
- `impl TryComponentsInto for MemoryIterator`
- `impl UintsFrom for MemoryIterator`
- `impl UintsInto for MemoryIterator`
- `impl AdaptIntoUnclamped for MemoryIterator`
- `impl AdaptInto for MemoryIterator`
- `impl IntoColor for MemoryIterator`
- `impl IntoColorUnclamped for MemoryIterator`
- `impl TryIntoColor for MemoryIterator`
- `impl FromStimulus for MemoryIterator`
- `impl IntoStimulus for MemoryIterator`
- `impl Iterator for MemoryIterator`

## Auto Trait Implementations

`Freeze` `Unpin` `UnsafeUnpin`

