---
id: LoadedDiffBuffer
title: LoadedDiffBuffer
---

# Struct: LoadedDiffBuffer

Defined in: [`packages/project/src/git_store/diff_buffer_list.rs:532`](../../../../packages/project/src/git_store/diff_buffer_list.rs#L532)

## Definition

```rust
pub struct LoadedDiffBuffer
{
    pub display_buffer: Entity<Buffer>,
    pub main_buffer: Entity<Buffer>,
    pub diff: Entity<BufferDiff>,
    pub conflict_set: Option<Entity<ConflictSet>>,
}
```

## Trait Implementations

- `impl Borrow for LoadedDiffBuffer`
- `impl BorrowMut for LoadedDiffBuffer`
- `impl Into for LoadedDiffBuffer`
- `impl From for LoadedDiffBuffer`
- `impl TryInto for LoadedDiffBuffer`
- `impl TryFrom for LoadedDiffBuffer`
- `impl Any for LoadedDiffBuffer`
- `impl VZip for LoadedDiffBuffer`
- `impl CastableFrom for LoadedDiffBuffer`
- `impl CastableFrom for LoadedDiffBuffer`
- `impl Read for LoadedDiffBuffer`
- `impl IntoEither for LoadedDiffBuffer`
- `impl ErasedDestructor for LoadedDiffBuffer`
- `impl Same for LoadedDiffBuffer`
- `impl Pointable for LoadedDiffBuffer`
- `impl Instrument for LoadedDiffBuffer`
- `impl WithSubscriber for LoadedDiffBuffer`
- `impl FromAngle for LoadedDiffBuffer`
- `impl IntoAngle for LoadedDiffBuffer`
- `impl IntoCam16Unclamped for LoadedDiffBuffer`
- `impl Cam16IntoUnclamped for LoadedDiffBuffer`
- `impl ArraysFrom for LoadedDiffBuffer`
- `impl ArraysInto for LoadedDiffBuffer`
- `impl ComponentsFrom for LoadedDiffBuffer`
- `impl TryComponentsInto for LoadedDiffBuffer`
- `impl UintsFrom for LoadedDiffBuffer`
- `impl UintsInto for LoadedDiffBuffer`
- `impl AdaptIntoUnclamped for LoadedDiffBuffer`
- `impl AdaptInto for LoadedDiffBuffer`
- `impl IntoColor for LoadedDiffBuffer`
- `impl IntoColorUnclamped for LoadedDiffBuffer`
- `impl TryIntoColor for LoadedDiffBuffer`
- `impl FromStimulus for LoadedDiffBuffer`
- `impl IntoStimulus for LoadedDiffBuffer`
- `impl Debug for LoadedDiffBuffer`

## Auto Trait Implementations

`Freeze` `Send` `Sync` `Unpin` `UnsafeUnpin`

