---
id: RunningJob
title: RunningJob
---

# Struct: RunningJob

Defined in: [`packages/project/src/git_store/job_debug_queue.rs:24`](../../../../packages/project/src/git_store/job_debug_queue.rs#L24)

## Definition

```rust
pub struct RunningJob
{
    pub id: u64,
    pub description: SharedString,
    pub key: Option<SharedString>,
    pub enqueued_at: Instant,
    pub started_at: Instant,
}
```

## Trait Implementations

- `impl Borrow for RunningJob`
- `impl BorrowMut for RunningJob`
- `impl CloneToUninit for RunningJob`
- `impl Into for RunningJob`
- `impl From for RunningJob`
- `impl TryInto for RunningJob`
- `impl TryFrom for RunningJob`
- `impl Any for RunningJob`
- `impl ToOwned for RunningJob`
- `impl DynClone for RunningJob`
- `impl VZip for RunningJob`
- `impl CastableFrom for RunningJob`
- `impl CastableFrom for RunningJob`
- `impl Read for RunningJob`
- `impl IntoEither for RunningJob`
- `impl ErasedDestructor for RunningJob`
- `impl Same for RunningJob`
- `impl Pointable for RunningJob`
- `impl Instrument for RunningJob`
- `impl WithSubscriber for RunningJob`
- `impl FromAngle for RunningJob`
- `impl IntoAngle for RunningJob`
- `impl IntoCam16Unclamped for RunningJob`
- `impl Cam16IntoUnclamped for RunningJob`
- `impl ArraysFrom for RunningJob`
- `impl ArraysInto for RunningJob`
- `impl ComponentsFrom for RunningJob`
- `impl TryComponentsInto for RunningJob`
- `impl UintsFrom for RunningJob`
- `impl UintsInto for RunningJob`
- `impl AdaptIntoUnclamped for RunningJob`
- `impl AdaptInto for RunningJob`
- `impl IntoColor for RunningJob`
- `impl IntoColorUnclamped for RunningJob`
- `impl TryIntoColor for RunningJob`
- `impl FromStimulus for RunningJob`
- `impl IntoStimulus for RunningJob`
- `impl Clone for RunningJob`
- `impl Debug for RunningJob`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

