---
id: PageContents
title: PageContents
---

# Enum: PageContents

Defined in: [`packages/project/src/debugger/memory.rs:26`](../../../../packages/project/src/debugger/memory.rs#L26)

Represents the contents of a single page. We special-case unmapped pages to be allocation-free,
since they're going to make up the majority of the memory in a program space (even though the user might not even get to see them - ever).

## Definition

```rust
pub enum PageContents
{
    // Whole page is unreadable.
    Unmapped,
    Mapped(Arc<MappedPageContents>),
}
```

## Variants

### Unmapped

Defined in: [`packages/project/src/debugger/memory.rs:28`](../../../../packages/project/src/debugger/memory.rs#L28)

Whole page is unreadable.

## Trait Implementations

- `impl Borrow for PageContents`
- `impl BorrowMut for PageContents`
- `impl CloneToUninit for PageContents`
- `impl Into for PageContents`
- `impl From for PageContents`
- `impl TryInto for PageContents`
- `impl TryFrom for PageContents`
- `impl Any for PageContents`
- `impl ToOwned for PageContents`
- `impl DynClone for PageContents`
- `impl VZip for PageContents`
- `impl CastableFrom for PageContents`
- `impl CastableFrom for PageContents`
- `impl Read for PageContents`
- `impl IntoEither for PageContents`
- `impl ErasedDestructor for PageContents`
- `impl Same for PageContents`
- `impl Pointable for PageContents`
- `impl Instrument for PageContents`
- `impl WithSubscriber for PageContents`
- `impl FromAngle for PageContents`
- `impl IntoAngle for PageContents`
- `impl IntoCam16Unclamped for PageContents`
- `impl Cam16IntoUnclamped for PageContents`
- `impl ArraysFrom for PageContents`
- `impl ArraysInto for PageContents`
- `impl ComponentsFrom for PageContents`
- `impl TryComponentsInto for PageContents`
- `impl UintsFrom for PageContents`
- `impl UintsInto for PageContents`
- `impl AdaptIntoUnclamped for PageContents`
- `impl AdaptInto for PageContents`
- `impl IntoColor for PageContents`
- `impl IntoColorUnclamped for PageContents`
- `impl TryIntoColor for PageContents`
- `impl FromStimulus for PageContents`
- `impl IntoStimulus for PageContents`
- `impl Clone for PageContents`
- `impl Debug for PageContents`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

