---
id: GlobalLspSettings
title: GlobalLspSettings
---

# Struct: GlobalLspSettings

Defined in: [`packages/project/src/project_settings.rs:124`](../../../../packages/project/src/project_settings.rs#L124)

Common language server settings.

## Definition

```rust
pub struct GlobalLspSettings {
    // Whether to show the LSP servers button in the status bar. Default: `true`
    pub button: bool,
    // The maximum amount of time to wait for responses from language servers, in seconds. A value of `0` will result in no timeout being applied (causing all LSP responses to wait indefinitely until completed). This should not be used outside of serialization/de-serialization in favor of get_request_timeout. Default: `120`
    pub request_timeout: u64,
    // The maximum line length a buffer may contain before language server features are disabled for the entire buffer. Default: `20000`
    pub max_buffer_line_length: u32,
    pub notifications: LspNotificationSettings,
    // Rules for highlighting semantic tokens.
    pub semantic_token_rules: SemanticTokenRules,
}
```

## Fields

### button

Defined in: [`packages/project/src/project_settings.rs:128`](../../../../packages/project/src/project_settings.rs#L128)

Whether to show the LSP servers button in the status bar.

Default: `true`


***

### request_timeout

Defined in: [`packages/project/src/project_settings.rs:135`](../../../../packages/project/src/project_settings.rs#L135)

The maximum amount of time to wait for responses from language servers, in seconds.
A value of `0` will result in no timeout being applied (causing all LSP responses to wait
indefinitely until completed).
This should not be used outside of serialization/de-serialization in favor of get_request_timeout.

Default: `120`


***

### max_buffer_line_length

Defined in: [`packages/project/src/project_settings.rs:139`](../../../../packages/project/src/project_settings.rs#L139)

The maximum line length a buffer may contain before language server features are disabled for the entire buffer.

Default: `20000`


***

### semantic_token_rules

Defined in: [`packages/project/src/project_settings.rs:143`](../../../../packages/project/src/project_settings.rs#L143)

Rules for highlighting semantic tokens.

## Implementations

### get_request_timeout()

```rust
pub const fn get_request_timeout(&self) -> Duration
```

Defined in: [`packages/project/src/project_settings.rs:162`](../../../../packages/project/src/project_settings.rs#L162)

Returns the timeout duration for LSP-related interactions, or Duration::ZERO if no timeout should be applied.
Zero durations are treated as no timeout by language servers, so code using this in an async context can
simply call unwrap_or_default.

#### Returns

`Duration`

## Trait Implementations

- `impl Borrow for GlobalLspSettings`
- `impl BorrowMut for GlobalLspSettings`
- `impl CloneToUninit for GlobalLspSettings`
- `impl Into for GlobalLspSettings`
- `impl From for GlobalLspSettings`
- `impl TryInto for GlobalLspSettings`
- `impl TryFrom for GlobalLspSettings`
- `impl Any for GlobalLspSettings`
- `impl ToOwned for GlobalLspSettings`
- `impl DeserializeOwned for GlobalLspSettings`
- `impl Serialize for GlobalLspSettings`
- `impl DynClone for GlobalLspSettings`
- `impl VZip for GlobalLspSettings`
- `impl CastableFrom for GlobalLspSettings`
- `impl CastableFrom for GlobalLspSettings`
- `impl Read for GlobalLspSettings`
- `impl IntoEither for GlobalLspSettings`
- `impl ErasedDestructor for GlobalLspSettings`
- `impl Same for GlobalLspSettings`
- `impl ReadPrimitive for GlobalLspSettings`
- `impl Pointable for GlobalLspSettings`
- `impl Instrument for GlobalLspSettings`
- `impl WithSubscriber for GlobalLspSettings`
- `impl FromAngle for GlobalLspSettings`
- `impl IntoAngle for GlobalLspSettings`
- `impl IntoCam16Unclamped for GlobalLspSettings`
- `impl Cam16IntoUnclamped for GlobalLspSettings`
- `impl ArraysFrom for GlobalLspSettings`
- `impl ArraysInto for GlobalLspSettings`
- `impl ComponentsFrom for GlobalLspSettings`
- `impl TryComponentsInto for GlobalLspSettings`
- `impl UintsFrom for GlobalLspSettings`
- `impl UintsInto for GlobalLspSettings`
- `impl AdaptIntoUnclamped for GlobalLspSettings`
- `impl AdaptInto for GlobalLspSettings`
- `impl IntoColor for GlobalLspSettings`
- `impl IntoColorUnclamped for GlobalLspSettings`
- `impl TryIntoColor for GlobalLspSettings`
- `impl FromStimulus for GlobalLspSettings`
- `impl IntoStimulus for GlobalLspSettings`
- `impl Debug for GlobalLspSettings`
- `impl Clone for GlobalLspSettings`
- `impl StructuralPartialEq for GlobalLspSettings`
- `impl PartialEq for GlobalLspSettings`
- `impl Serialize for GlobalLspSettings`
- `impl Deserialize for GlobalLspSettings`
- `impl JsonSchema for GlobalLspSettings`
- `impl Default for GlobalLspSettings`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

