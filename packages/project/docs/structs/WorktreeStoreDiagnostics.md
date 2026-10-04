---
id: WorktreeStoreDiagnostics
title: WorktreeStoreDiagnostics
---

# Struct: WorktreeStoreDiagnostics

Defined in: [`packages/project/src/worktree_store.rs:187`](../../../../packages/project/src/worktree_store.rs#L187)

Summarizes worktree ownership and current snapshot sizes.

## Definition

```rust
pub struct WorktreeStoreDiagnostics {
    pub worktree_slots: usize,
    pub live_worktrees: usize,
    pub visible_worktrees: usize,
    pub strong_handles: usize,
    pub dead_weak_handles: usize,
    pub loading_worktrees: usize,
    pub total_entries: usize,
    pub visible_entries: usize,
    pub largest_worktree: Option<LargestWorktreeDiagnostics>,
}
```

## Trait Implementations

- `impl Borrow for WorktreeStoreDiagnostics`
- `impl BorrowMut for WorktreeStoreDiagnostics`
- `impl Into for WorktreeStoreDiagnostics`
- `impl From for WorktreeStoreDiagnostics`
- `impl TryInto for WorktreeStoreDiagnostics`
- `impl TryFrom for WorktreeStoreDiagnostics`
- `impl Any for WorktreeStoreDiagnostics`
- `impl VZip for WorktreeStoreDiagnostics`
- `impl CastableFrom for WorktreeStoreDiagnostics`
- `impl CastableFrom for WorktreeStoreDiagnostics`
- `impl Read for WorktreeStoreDiagnostics`
- `impl IntoEither for WorktreeStoreDiagnostics`
- `impl ErasedDestructor for WorktreeStoreDiagnostics`
- `impl Same for WorktreeStoreDiagnostics`
- `impl ReadPrimitive for WorktreeStoreDiagnostics`
- `impl Pointable for WorktreeStoreDiagnostics`
- `impl Instrument for WorktreeStoreDiagnostics`
- `impl WithSubscriber for WorktreeStoreDiagnostics`
- `impl FromAngle for WorktreeStoreDiagnostics`
- `impl IntoAngle for WorktreeStoreDiagnostics`
- `impl IntoCam16Unclamped for WorktreeStoreDiagnostics`
- `impl Cam16IntoUnclamped for WorktreeStoreDiagnostics`
- `impl ArraysFrom for WorktreeStoreDiagnostics`
- `impl ArraysInto for WorktreeStoreDiagnostics`
- `impl ComponentsFrom for WorktreeStoreDiagnostics`
- `impl TryComponentsInto for WorktreeStoreDiagnostics`
- `impl UintsFrom for WorktreeStoreDiagnostics`
- `impl UintsInto for WorktreeStoreDiagnostics`
- `impl AdaptIntoUnclamped for WorktreeStoreDiagnostics`
- `impl AdaptInto for WorktreeStoreDiagnostics`
- `impl IntoColor for WorktreeStoreDiagnostics`
- `impl IntoColorUnclamped for WorktreeStoreDiagnostics`
- `impl TryIntoColor for WorktreeStoreDiagnostics`
- `impl FromStimulus for WorktreeStoreDiagnostics`
- `impl IntoStimulus for WorktreeStoreDiagnostics`
- `impl Debug for WorktreeStoreDiagnostics`
- `impl Default for WorktreeStoreDiagnostics`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

