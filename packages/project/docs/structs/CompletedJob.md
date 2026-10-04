---
id: CompletedJob
title: CompletedJob
---

# Struct: CompletedJob

Defined in: [`packages/project/src/git_store/job_debug_queue.rs:33`](../../../../packages/project/src/git_store/job_debug_queue.rs#L33)

## Definition

```rust
pub struct CompletedJob {
    pub id: u64,
    pub description: SharedString,
    pub key: Option<SharedString>,
    pub enqueued_at: Instant,
    pub started_at: Option<Instant>,
    pub completed_at: Instant,
    pub status: CompletedJobStatus,
}
```

## Trait Implementations

- `impl Borrow for CompletedJob`
- `impl BorrowMut for CompletedJob`
- `impl CloneToUninit for CompletedJob`
- `impl Into for CompletedJob`
- `impl From for CompletedJob`
- `impl TryInto for CompletedJob`
- `impl TryFrom for CompletedJob`
- `impl Any for CompletedJob`
- `impl ToOwned for CompletedJob`
- `impl DynClone for CompletedJob`
- `impl VZip for CompletedJob`
- `impl CastableFrom for CompletedJob`
- `impl CastableFrom for CompletedJob`
- `impl Read for CompletedJob`
- `impl IntoEither for CompletedJob`
- `impl ErasedDestructor for CompletedJob`
- `impl Same for CompletedJob`
- `impl Pointable for CompletedJob`
- `impl Instrument for CompletedJob`
- `impl WithSubscriber for CompletedJob`
- `impl FromAngle for CompletedJob`
- `impl IntoAngle for CompletedJob`
- `impl IntoCam16Unclamped for CompletedJob`
- `impl Cam16IntoUnclamped for CompletedJob`
- `impl ArraysFrom for CompletedJob`
- `impl ArraysInto for CompletedJob`
- `impl ComponentsFrom for CompletedJob`
- `impl TryComponentsInto for CompletedJob`
- `impl UintsFrom for CompletedJob`
- `impl UintsInto for CompletedJob`
- `impl AdaptIntoUnclamped for CompletedJob`
- `impl AdaptInto for CompletedJob`
- `impl IntoColor for CompletedJob`
- `impl IntoColorUnclamped for CompletedJob`
- `impl TryIntoColor for CompletedJob`
- `impl FromStimulus for CompletedJob`
- `impl IntoStimulus for CompletedJob`
- `impl Clone for CompletedJob`
- `impl Debug for CompletedJob`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

