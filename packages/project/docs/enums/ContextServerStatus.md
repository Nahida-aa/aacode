---
id: ContextServerStatus
title: ContextServerStatus
---

# Enum: ContextServerStatus

Defined in: [`packages/project/src/context_server_store/mod.rs:49`](../../../../packages/project/src/context_server_store/mod.rs#L49)

## Definition

```rust
pub enum ContextServerStatus
{
    Starting,
    Running,
    Stopped,
    Error(Arc<str>),
    // The server returned 401 and OAuth authorization is needed. The UI should show an "Authenticate" button.
    AuthRequired,
    // The server has a pre-registered OAuth client_id, but a client_secret is needed and not available in settings or the keychain.
    ClientSecretRequired{ .. },
    // The OAuth browser flow is in progress — the user has been redirected to the authorization server and we're waiting for the callback.
    Authenticating,
}
```

## Variants

### AuthRequired

Defined in: [`packages/project/src/context_server_store/mod.rs:56`](../../../../packages/project/src/context_server_store/mod.rs#L56)

The server returned 401 and OAuth authorization is needed. The UI
should show an "Authenticate" button.


***

### ClientSecretRequired

Defined in: [`packages/project/src/context_server_store/mod.rs:59`](../../../../packages/project/src/context_server_store/mod.rs#L59)

The server has a pre-registered OAuth client_id, but a client_secret
is needed and not available in settings or the keychain.


***

### Authenticating

Defined in: [`packages/project/src/context_server_store/mod.rs:64`](../../../../packages/project/src/context_server_store/mod.rs#L64)

The OAuth browser flow is in progress — the user has been redirected
to the authorization server and we're waiting for the callback.

## Trait Implementations

- `impl Borrow for ContextServerStatus`
- `impl BorrowMut for ContextServerStatus`
- `impl CloneToUninit for ContextServerStatus`
- `impl Into for ContextServerStatus`
- `impl From for ContextServerStatus`
- `impl TryInto for ContextServerStatus`
- `impl TryFrom for ContextServerStatus`
- `impl Any for ContextServerStatus`
- `impl ToOwned for ContextServerStatus`
- `impl Equivalent for ContextServerStatus`
- `impl DynClone for ContextServerStatus`
- `impl VZip for ContextServerStatus`
- `impl CastableFrom for ContextServerStatus`
- `impl CastableFrom for ContextServerStatus`
- `impl Read for ContextServerStatus`
- `impl IntoEither for ContextServerStatus`
- `impl ErasedDestructor for ContextServerStatus`
- `impl Same for ContextServerStatus`
- `impl Pointable for ContextServerStatus`
- `impl Instrument for ContextServerStatus`
- `impl WithSubscriber for ContextServerStatus`
- `impl FromAngle for ContextServerStatus`
- `impl IntoAngle for ContextServerStatus`
- `impl IntoCam16Unclamped for ContextServerStatus`
- `impl Cam16IntoUnclamped for ContextServerStatus`
- `impl ArraysFrom for ContextServerStatus`
- `impl ArraysInto for ContextServerStatus`
- `impl ComponentsFrom for ContextServerStatus`
- `impl TryComponentsInto for ContextServerStatus`
- `impl UintsFrom for ContextServerStatus`
- `impl UintsInto for ContextServerStatus`
- `impl AdaptIntoUnclamped for ContextServerStatus`
- `impl AdaptInto for ContextServerStatus`
- `impl IntoColor for ContextServerStatus`
- `impl IntoColorUnclamped for ContextServerStatus`
- `impl TryIntoColor for ContextServerStatus`
- `impl FromStimulus for ContextServerStatus`
- `impl IntoStimulus for ContextServerStatus`
- `impl Equivalent for ContextServerStatus`
- `impl Debug for ContextServerStatus`
- `impl Clone for ContextServerStatus`
- `impl StructuralPartialEq for ContextServerStatus`
- `impl PartialEq for ContextServerStatus`
- `impl Eq for ContextServerStatus`
- `impl Hash for ContextServerStatus`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

