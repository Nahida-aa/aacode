---
id: RemoteHostLocation
title: RemoteHostLocation
---

# Struct: RemoteHostLocation

Defined in: [`packages/project/src/trusted_worktrees.rs:154`](../../../../packages/project/src/trusted_worktrees.rs#L154)

An identifier of a host to split the trust questions by.
Each trusted data change and event is done for a particular host.
A host may contain more than one worktree or even project open concurrently.

## Definition

```rust
pub struct RemoteHostLocation
{
    pub user_name: Option<SharedString>,
    pub host_identifier: SharedString,
}
```

## Trait Implementations

- `impl Borrow for RemoteHostLocation`
- `impl BorrowMut for RemoteHostLocation`
- `impl CloneToUninit for RemoteHostLocation`
- `impl Into for RemoteHostLocation`
- `impl From for RemoteHostLocation`
- `impl TryInto for RemoteHostLocation`
- `impl TryFrom for RemoteHostLocation`
- `impl Any for RemoteHostLocation`
- `impl ToOwned for RemoteHostLocation`
- `impl Equivalent for RemoteHostLocation`
- `impl DynClone for RemoteHostLocation`
- `impl VZip for RemoteHostLocation`
- `impl CastableFrom for RemoteHostLocation`
- `impl CastableFrom for RemoteHostLocation`
- `impl Read for RemoteHostLocation`
- `impl IntoEither for RemoteHostLocation`
- `impl ErasedDestructor for RemoteHostLocation`
- `impl Same for RemoteHostLocation`
- `impl Pointable for RemoteHostLocation`
- `impl Instrument for RemoteHostLocation`
- `impl WithSubscriber for RemoteHostLocation`
- `impl FromAngle for RemoteHostLocation`
- `impl IntoAngle for RemoteHostLocation`
- `impl IntoCam16Unclamped for RemoteHostLocation`
- `impl Cam16IntoUnclamped for RemoteHostLocation`
- `impl ArraysFrom for RemoteHostLocation`
- `impl ArraysInto for RemoteHostLocation`
- `impl ComponentsFrom for RemoteHostLocation`
- `impl TryComponentsInto for RemoteHostLocation`
- `impl UintsFrom for RemoteHostLocation`
- `impl UintsInto for RemoteHostLocation`
- `impl AdaptIntoUnclamped for RemoteHostLocation`
- `impl AdaptInto for RemoteHostLocation`
- `impl IntoColor for RemoteHostLocation`
- `impl IntoColorUnclamped for RemoteHostLocation`
- `impl TryIntoColor for RemoteHostLocation`
- `impl FromStimulus for RemoteHostLocation`
- `impl IntoStimulus for RemoteHostLocation`
- `impl Equivalent for RemoteHostLocation`
- `impl Debug for RemoteHostLocation`
- `impl StructuralPartialEq for RemoteHostLocation`
- `impl PartialEq for RemoteHostLocation`
- `impl Eq for RemoteHostLocation`
- `impl Clone for RemoteHostLocation`
- `impl Hash for RemoteHostLocation`
- `impl From for RemoteHostLocation`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

