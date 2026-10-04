---
id: LargestWorktreeDiagnostics
title: LargestWorktreeDiagnostics
---

# Struct: LargestWorktreeDiagnostics

Defined in: [`packages/project/src/worktree_store.rs:201`](../../../../packages/project/src/worktree_store.rs#L201)

Identifies the worktree with the largest current snapshot.

## Definition

```rust
pub struct LargestWorktreeDiagnostics {
    pub path: PathBuf,
    pub entries: usize,
    pub visible_entries: usize,
}
```

## Trait Implementations

- `impl Borrow for LargestWorktreeDiagnostics`
- `impl BorrowMut for LargestWorktreeDiagnostics`
- `impl Into for LargestWorktreeDiagnostics`
- `impl From for LargestWorktreeDiagnostics`
- `impl TryInto for LargestWorktreeDiagnostics`
- `impl TryFrom for LargestWorktreeDiagnostics`
- `impl Any for LargestWorktreeDiagnostics`
- `impl VZip for LargestWorktreeDiagnostics`
- `impl CastableFrom for LargestWorktreeDiagnostics`
- `impl CastableFrom for LargestWorktreeDiagnostics`
- `impl Read for LargestWorktreeDiagnostics`
- `impl IntoEither for LargestWorktreeDiagnostics`
- `impl ErasedDestructor for LargestWorktreeDiagnostics`
- `impl Same for LargestWorktreeDiagnostics`
- `impl Pointable for LargestWorktreeDiagnostics`
- `impl Instrument for LargestWorktreeDiagnostics`
- `impl WithSubscriber for LargestWorktreeDiagnostics`
- `impl FromAngle for LargestWorktreeDiagnostics`
- `impl IntoAngle for LargestWorktreeDiagnostics`
- `impl IntoCam16Unclamped for LargestWorktreeDiagnostics`
- `impl Cam16IntoUnclamped for LargestWorktreeDiagnostics`
- `impl ArraysFrom for LargestWorktreeDiagnostics`
- `impl ArraysInto for LargestWorktreeDiagnostics`
- `impl ComponentsFrom for LargestWorktreeDiagnostics`
- `impl TryComponentsInto for LargestWorktreeDiagnostics`
- `impl UintsFrom for LargestWorktreeDiagnostics`
- `impl UintsInto for LargestWorktreeDiagnostics`
- `impl AdaptIntoUnclamped for LargestWorktreeDiagnostics`
- `impl AdaptInto for LargestWorktreeDiagnostics`
- `impl IntoColor for LargestWorktreeDiagnostics`
- `impl IntoColorUnclamped for LargestWorktreeDiagnostics`
- `impl TryIntoColor for LargestWorktreeDiagnostics`
- `impl FromStimulus for LargestWorktreeDiagnostics`
- `impl IntoStimulus for LargestWorktreeDiagnostics`
- `impl Debug for LargestWorktreeDiagnostics`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

