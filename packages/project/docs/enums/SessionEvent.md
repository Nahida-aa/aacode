---
id: SessionEvent
title: SessionEvent
---

# Enum: SessionEvent

Defined in: [`packages/project/src/debugger/session.rs:793`](../../../../packages/project/src/debugger/session.rs#L793)

## Definition

```rust
pub enum SessionEvent {
    Modules,
    LoadedSources,
    Stopped(Option<ThreadId>),
    StackTrace,
    Variables,
    Watchers,
    Threads,
    InvalidateInlineValue,
    CapabilitiesLoaded,
    RunInTerminal{ .. },
    DataBreakpointInfo,
    ConsoleOutput,
    HistoricSnapshotSelected,
}
```

## Trait Implementations

- `impl Borrow for SessionEvent`
- `impl BorrowMut for SessionEvent`
- `impl Into for SessionEvent`
- `impl From for SessionEvent`
- `impl TryInto for SessionEvent`
- `impl TryFrom for SessionEvent`
- `impl Any for SessionEvent`
- `impl VZip for SessionEvent`
- `impl CastableFrom for SessionEvent`
- `impl CastableFrom for SessionEvent`
- `impl Read for SessionEvent`
- `impl IntoEither for SessionEvent`
- `impl ErasedDestructor for SessionEvent`
- `impl Same for SessionEvent`
- `impl Pointable for SessionEvent`
- `impl Instrument for SessionEvent`
- `impl WithSubscriber for SessionEvent`
- `impl FromAngle for SessionEvent`
- `impl IntoAngle for SessionEvent`
- `impl IntoCam16Unclamped for SessionEvent`
- `impl Cam16IntoUnclamped for SessionEvent`
- `impl ArraysFrom for SessionEvent`
- `impl ArraysInto for SessionEvent`
- `impl ComponentsFrom for SessionEvent`
- `impl TryComponentsInto for SessionEvent`
- `impl UintsFrom for SessionEvent`
- `impl UintsInto for SessionEvent`
- `impl AdaptIntoUnclamped for SessionEvent`
- `impl AdaptInto for SessionEvent`
- `impl IntoColor for SessionEvent`
- `impl IntoColorUnclamped for SessionEvent`
- `impl TryIntoColor for SessionEvent`
- `impl FromStimulus for SessionEvent`
- `impl IntoStimulus for SessionEvent`
- `impl Debug for SessionEvent`
- `impl EventEmitter for SessionEvent`

## Auto Trait Implementations

`Freeze` `Send` `Sync` `Unpin` `UnsafeUnpin`

