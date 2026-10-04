---
id: PendingJob
title: PendingJob
---

# Struct: PendingJob

Defined in: [`packages/project/src/git_store/job_debug_queue.rs:16`](../../../../packages/project/src/git_store/job_debug_queue.rs#L16)

## Definition

```rust
pub struct PendingJob {
    pub id: u64,
    pub description: SharedString,
    pub key: Option<SharedString>,
    pub enqueued_at: Instant,
}
```

## Trait Implementations

- `impl Borrow for PendingJob`
- `impl BorrowMut for PendingJob`
- `impl CloneToUninit for PendingJob`
- `impl Into for PendingJob`
- `impl From for PendingJob`
- `impl TryInto for PendingJob`
- `impl TryFrom for PendingJob`
- `impl Any for PendingJob`
- `impl ToOwned for PendingJob`
- `impl DynClone for PendingJob`
- `impl VZip for PendingJob`
- `impl CastableFrom for PendingJob`
- `impl CastableFrom for PendingJob`
- `impl Read for PendingJob`
- `impl IntoEither for PendingJob`
- `impl ErasedDestructor for PendingJob`
- `impl Same for PendingJob`
- `impl Pointable for PendingJob`
- `impl Instrument for PendingJob`
- `impl WithSubscriber for PendingJob`
- `impl FromAngle for PendingJob`
- `impl IntoAngle for PendingJob`
- `impl IntoCam16Unclamped for PendingJob`
- `impl Cam16IntoUnclamped for PendingJob`
- `impl ArraysFrom for PendingJob`
- `impl ArraysInto for PendingJob`
- `impl ComponentsFrom for PendingJob`
- `impl TryComponentsInto for PendingJob`
- `impl UintsFrom for PendingJob`
- `impl UintsInto for PendingJob`
- `impl AdaptIntoUnclamped for PendingJob`
- `impl AdaptInto for PendingJob`
- `impl IntoColor for PendingJob`
- `impl IntoColorUnclamped for PendingJob`
- `impl TryIntoColor for PendingJob`
- `impl FromStimulus for PendingJob`
- `impl IntoStimulus for PendingJob`
- `impl Clone for PendingJob`
- `impl Debug for PendingJob`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

