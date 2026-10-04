---
id: SessionState
title: SessionState
---

# Enum: SessionState

Defined in: [`packages/project/src/debugger/session.rs:154`](../../../../packages/project/src/debugger/session.rs#L154)

## Definition

```rust
pub enum SessionState
{
    // Represents a session that is building/initializing even if a session doesn't have a pre build task this state is used to run all the async tasks that are required to start the session
    Booting(Option<Task<Result<()>>>),
    Running(RunningMode),
}
```

## Variants

### Booting

Defined in: [`packages/project/src/debugger/session.rs:158`](../../../../packages/project/src/debugger/session.rs#L158)

Represents a session that is building/initializing
even if a session doesn't have a pre build task this state
is used to run all the async tasks that are required to start the session

## Trait Implementations

- `impl Borrow for SessionState`
- `impl BorrowMut for SessionState`
- `impl Into for SessionState`
- `impl From for SessionState`
- `impl TryInto for SessionState`
- `impl TryFrom for SessionState`
- `impl Any for SessionState`
- `impl VZip for SessionState`
- `impl CastableFrom for SessionState`
- `impl CastableFrom for SessionState`
- `impl Read for SessionState`
- `impl IntoEither for SessionState`
- `impl ErasedDestructor for SessionState`
- `impl Same for SessionState`
- `impl Pointable for SessionState`
- `impl Instrument for SessionState`
- `impl WithSubscriber for SessionState`
- `impl FromAngle for SessionState`
- `impl IntoAngle for SessionState`
- `impl IntoCam16Unclamped for SessionState`
- `impl Cam16IntoUnclamped for SessionState`
- `impl ArraysFrom for SessionState`
- `impl ArraysInto for SessionState`
- `impl ComponentsFrom for SessionState`
- `impl TryComponentsInto for SessionState`
- `impl UintsFrom for SessionState`
- `impl UintsInto for SessionState`
- `impl AdaptIntoUnclamped for SessionState`
- `impl AdaptInto for SessionState`
- `impl IntoColor for SessionState`
- `impl IntoColorUnclamped for SessionState`
- `impl TryIntoColor for SessionState`
- `impl FromStimulus for SessionState`
- `impl IntoStimulus for SessionState`

## Auto Trait Implementations

`Freeze` `Send` `Sync` `Unpin` `UnsafeUnpin`

