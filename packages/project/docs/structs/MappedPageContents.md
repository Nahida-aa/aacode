---
id: MappedPageContents
title: MappedPageContents
---

# Struct: MappedPageContents

Defined in: [`packages/project/src/debugger/memory.rs:71`](../../../../packages/project/src/debugger/memory.rs#L71)

We hope for the whole page to be mapped in a single chunk, but we do leave the possibility open
of having interleaved read permissions in a single page; debuggee's execution environment might either
have a different page size OR it might not have paged memory layout altogether
(which might be relevant to embedded systems).

As stated previously, the concept of a page in this module has to do more
with optimizing fetching of the memory and not with the underlying bits and pieces
of the memory of a debuggee.

## Definition

```rust
pub struct MappedPageContents
```

## Trait Implementations

- `impl Borrow for MappedPageContents`
- `impl BorrowMut for MappedPageContents`
- `impl Into for MappedPageContents`
- `impl From for MappedPageContents`
- `impl TryInto for MappedPageContents`
- `impl TryFrom for MappedPageContents`
- `impl Any for MappedPageContents`
- `impl VZip for MappedPageContents`
- `impl CastableFrom for MappedPageContents`
- `impl CastableFrom for MappedPageContents`
- `impl Read for MappedPageContents`
- `impl IntoEither for MappedPageContents`
- `impl ErasedDestructor for MappedPageContents`
- `impl Same for MappedPageContents`
- `impl ReadPrimitive for MappedPageContents`
- `impl Pointable for MappedPageContents`
- `impl Instrument for MappedPageContents`
- `impl WithSubscriber for MappedPageContents`
- `impl FromAngle for MappedPageContents`
- `impl IntoAngle for MappedPageContents`
- `impl IntoCam16Unclamped for MappedPageContents`
- `impl Cam16IntoUnclamped for MappedPageContents`
- `impl ArraysFrom for MappedPageContents`
- `impl ArraysInto for MappedPageContents`
- `impl ComponentsFrom for MappedPageContents`
- `impl TryComponentsInto for MappedPageContents`
- `impl UintsFrom for MappedPageContents`
- `impl UintsInto for MappedPageContents`
- `impl AdaptIntoUnclamped for MappedPageContents`
- `impl AdaptInto for MappedPageContents`
- `impl IntoColor for MappedPageContents`
- `impl IntoColorUnclamped for MappedPageContents`
- `impl TryIntoColor for MappedPageContents`
- `impl FromStimulus for MappedPageContents`
- `impl IntoStimulus for MappedPageContents`
- `impl Default for MappedPageContents`
- `impl Debug for MappedPageContents`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

