---
id: OAuthClientSettings
title: OAuthClientSettings
---

# Struct: OAuthClientSettings

Defined in: [`packages/project/src/project_settings.rs:313`](../../../../packages/project/src/project_settings.rs#L313)

Pre-registered OAuth client credentials for MCP servers that don't support
Dynamic Client Registration.

## Definition

```rust
pub struct OAuthClientSettings {
    // The OAuth client ID obtained from out-of-band registration with the authorization server.
    pub client_id: String,
    // The OAuth client secret, if this is a confidential client. For security, prefer providing this interactively; we will prompt and store it in the system keychain.
    pub client_secret: Option<String>,
}
```

## Fields

### client_id

Defined in: [`packages/project/src/project_settings.rs:316`](../../../../packages/project/src/project_settings.rs#L316)

The OAuth client ID obtained from out-of-band registration with the
authorization server.


***

### client_secret

Defined in: [`packages/project/src/project_settings.rs:321`](../../../../packages/project/src/project_settings.rs#L321)

The OAuth client secret, if this is a confidential client. For security,
prefer providing this interactively; we will prompt and store it in
the system keychain.

## Trait Implementations

- `impl Borrow for OAuthClientSettings`
- `impl BorrowMut for OAuthClientSettings`
- `impl CloneToUninit for OAuthClientSettings`
- `impl Into for OAuthClientSettings`
- `impl From for OAuthClientSettings`
- `impl TryInto for OAuthClientSettings`
- `impl TryFrom for OAuthClientSettings`
- `impl Any for OAuthClientSettings`
- `impl ToOwned for OAuthClientSettings`
- `impl DeserializeOwned for OAuthClientSettings`
- `impl Equivalent for OAuthClientSettings`
- `impl Serialize for OAuthClientSettings`
- `impl DynClone for OAuthClientSettings`
- `impl VZip for OAuthClientSettings`
- `impl CastableFrom for OAuthClientSettings`
- `impl CastableFrom for OAuthClientSettings`
- `impl Read for OAuthClientSettings`
- `impl IntoEither for OAuthClientSettings`
- `impl ErasedDestructor for OAuthClientSettings`
- `impl Same for OAuthClientSettings`
- `impl Pointable for OAuthClientSettings`
- `impl Instrument for OAuthClientSettings`
- `impl WithSubscriber for OAuthClientSettings`
- `impl FromAngle for OAuthClientSettings`
- `impl IntoAngle for OAuthClientSettings`
- `impl IntoCam16Unclamped for OAuthClientSettings`
- `impl Cam16IntoUnclamped for OAuthClientSettings`
- `impl ArraysFrom for OAuthClientSettings`
- `impl ArraysInto for OAuthClientSettings`
- `impl ComponentsFrom for OAuthClientSettings`
- `impl TryComponentsInto for OAuthClientSettings`
- `impl UintsFrom for OAuthClientSettings`
- `impl UintsInto for OAuthClientSettings`
- `impl AdaptIntoUnclamped for OAuthClientSettings`
- `impl AdaptInto for OAuthClientSettings`
- `impl IntoColor for OAuthClientSettings`
- `impl IntoColorUnclamped for OAuthClientSettings`
- `impl TryIntoColor for OAuthClientSettings`
- `impl FromStimulus for OAuthClientSettings`
- `impl IntoStimulus for OAuthClientSettings`
- `impl Equivalent for OAuthClientSettings`
- `impl Deserialize for OAuthClientSettings`
- `impl Serialize for OAuthClientSettings`
- `impl Clone for OAuthClientSettings`
- `impl StructuralPartialEq for OAuthClientSettings`
- `impl PartialEq for OAuthClientSettings`
- `impl Eq for OAuthClientSettings`
- `impl JsonSchema for OAuthClientSettings`
- `impl Debug for OAuthClientSettings`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

