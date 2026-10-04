---
id: DiffBuffer
title: DiffBuffer
---

# Struct: DiffBuffer

Defined in: [`packages/project/src/git_store/diff_buffer_list.rs:540`](../../../../packages/project/src/git_store/diff_buffer_list.rs#L540)

## Definition

```rust
pub struct DiffBuffer
{
    pub repo_path: RepoPath,
    pub file_status: FileStatus,
    pub load: Task<Result<LoadedDiffBuffer>>,
}
```

## Trait Implementations

- `impl Borrow for DiffBuffer`
- `impl BorrowMut for DiffBuffer`
- `impl Into for DiffBuffer`
- `impl From for DiffBuffer`
- `impl TryInto for DiffBuffer`
- `impl TryFrom for DiffBuffer`
- `impl Any for DiffBuffer`
- `impl VZip for DiffBuffer`
- `impl CastableFrom for DiffBuffer`
- `impl CastableFrom for DiffBuffer`
- `impl Read for DiffBuffer`
- `impl IntoEither for DiffBuffer`
- `impl ErasedDestructor for DiffBuffer`
- `impl Same for DiffBuffer`
- `impl Pointable for DiffBuffer`
- `impl Instrument for DiffBuffer`
- `impl WithSubscriber for DiffBuffer`
- `impl FromAngle for DiffBuffer`
- `impl IntoAngle for DiffBuffer`
- `impl IntoCam16Unclamped for DiffBuffer`
- `impl Cam16IntoUnclamped for DiffBuffer`
- `impl ArraysFrom for DiffBuffer`
- `impl ArraysInto for DiffBuffer`
- `impl ComponentsFrom for DiffBuffer`
- `impl TryComponentsInto for DiffBuffer`
- `impl UintsFrom for DiffBuffer`
- `impl UintsInto for DiffBuffer`
- `impl AdaptIntoUnclamped for DiffBuffer`
- `impl AdaptInto for DiffBuffer`
- `impl IntoColor for DiffBuffer`
- `impl IntoColorUnclamped for DiffBuffer`
- `impl TryIntoColor for DiffBuffer`
- `impl FromStimulus for DiffBuffer`
- `impl IntoStimulus for DiffBuffer`
- `impl Debug for DiffBuffer`

## Auto Trait Implementations

`Freeze` `Send` `Sync` `Unpin` `UnsafeUnpin`

