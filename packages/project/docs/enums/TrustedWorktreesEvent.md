---
id: TrustedWorktreesEvent
title: TrustedWorktreesEvent
---

# Enum: TrustedWorktreesEvent

Defined in: [`packages/project/src/trusted_worktrees.rs:227`](../../../../packages/project/src/trusted_worktrees.rs#L227)

A change of trust on a certain host.

## Definition

```rust
pub enum TrustedWorktreesEvent {
    Trusted(WeakEntity<WorktreeStore>, HashSet<PathTrust>),
    Restricted(WeakEntity<WorktreeStore>, HashSet<PathTrust>),
}
```

## Trait Implementations

- `impl Borrow for TrustedWorktreesEvent`
- `impl BorrowMut for TrustedWorktreesEvent`
- `impl Into for TrustedWorktreesEvent`
- `impl From for TrustedWorktreesEvent`
- `impl TryInto for TrustedWorktreesEvent`
- `impl TryFrom for TrustedWorktreesEvent`
- `impl Any for TrustedWorktreesEvent`
- `impl VZip for TrustedWorktreesEvent`
- `impl CastableFrom for TrustedWorktreesEvent`
- `impl CastableFrom for TrustedWorktreesEvent`
- `impl Read for TrustedWorktreesEvent`
- `impl IntoEither for TrustedWorktreesEvent`
- `impl ErasedDestructor for TrustedWorktreesEvent`
- `impl Same for TrustedWorktreesEvent`
- `impl Pointable for TrustedWorktreesEvent`
- `impl Instrument for TrustedWorktreesEvent`
- `impl WithSubscriber for TrustedWorktreesEvent`
- `impl FromAngle for TrustedWorktreesEvent`
- `impl IntoAngle for TrustedWorktreesEvent`
- `impl IntoCam16Unclamped for TrustedWorktreesEvent`
- `impl Cam16IntoUnclamped for TrustedWorktreesEvent`
- `impl ArraysFrom for TrustedWorktreesEvent`
- `impl ArraysInto for TrustedWorktreesEvent`
- `impl ComponentsFrom for TrustedWorktreesEvent`
- `impl TryComponentsInto for TrustedWorktreesEvent`
- `impl UintsFrom for TrustedWorktreesEvent`
- `impl UintsInto for TrustedWorktreesEvent`
- `impl AdaptIntoUnclamped for TrustedWorktreesEvent`
- `impl AdaptInto for TrustedWorktreesEvent`
- `impl IntoColor for TrustedWorktreesEvent`
- `impl IntoColorUnclamped for TrustedWorktreesEvent`
- `impl TryIntoColor for TrustedWorktreesEvent`
- `impl FromStimulus for TrustedWorktreesEvent`
- `impl IntoStimulus for TrustedWorktreesEvent`
- `impl Debug for TrustedWorktreesEvent`
- `impl EventEmitter for TrustedWorktreesEvent`

## Auto Trait Implementations

`Freeze` `Send` `Sync` `Unpin` `UnsafeUnpin`

