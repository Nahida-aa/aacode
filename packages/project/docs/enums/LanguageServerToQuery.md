---
id: LanguageServerToQuery
title: LanguageServerToQuery
---

# Enum: LanguageServerToQuery

Defined in: [`packages/project/src/lsp_store/mod.rs:15740`](../../../../packages/project/src/lsp_store/mod.rs#L15740)

## Definition

```rust
pub enum LanguageServerToQuery {
    // Query language servers in order of users preference, up until one capable of handling the request is found.
    FirstCapable,
    // Query a specific language server.
    Other(LanguageServerId),
}
```

## Variants

### FirstCapable

Defined in: [`packages/project/src/lsp_store/mod.rs:15742`](../../../../packages/project/src/lsp_store/mod.rs#L15742)

Query language servers in order of users preference, up until one capable of handling the request is found.


***

### Other

Defined in: [`packages/project/src/lsp_store/mod.rs:15744`](../../../../packages/project/src/lsp_store/mod.rs#L15744)

Query a specific language server.

## Trait Implementations

- `impl Borrow for LanguageServerToQuery`
- `impl BorrowMut for LanguageServerToQuery`
- `impl Into for LanguageServerToQuery`
- `impl From for LanguageServerToQuery`
- `impl TryInto for LanguageServerToQuery`
- `impl TryFrom for LanguageServerToQuery`
- `impl Any for LanguageServerToQuery`
- `impl VZip for LanguageServerToQuery`
- `impl CastableFrom for LanguageServerToQuery`
- `impl CastableFrom for LanguageServerToQuery`
- `impl Read for LanguageServerToQuery`
- `impl IntoEither for LanguageServerToQuery`
- `impl ErasedDestructor for LanguageServerToQuery`
- `impl Same for LanguageServerToQuery`
- `impl Pointable for LanguageServerToQuery`
- `impl Instrument for LanguageServerToQuery`
- `impl WithSubscriber for LanguageServerToQuery`
- `impl FromAngle for LanguageServerToQuery`
- `impl IntoAngle for LanguageServerToQuery`
- `impl IntoCam16Unclamped for LanguageServerToQuery`
- `impl Cam16IntoUnclamped for LanguageServerToQuery`
- `impl ArraysFrom for LanguageServerToQuery`
- `impl ArraysInto for LanguageServerToQuery`
- `impl ComponentsFrom for LanguageServerToQuery`
- `impl TryComponentsInto for LanguageServerToQuery`
- `impl UintsFrom for LanguageServerToQuery`
- `impl UintsInto for LanguageServerToQuery`
- `impl AdaptIntoUnclamped for LanguageServerToQuery`
- `impl AdaptInto for LanguageServerToQuery`
- `impl IntoColor for LanguageServerToQuery`
- `impl IntoColorUnclamped for LanguageServerToQuery`
- `impl TryIntoColor for LanguageServerToQuery`
- `impl FromStimulus for LanguageServerToQuery`
- `impl IntoStimulus for LanguageServerToQuery`
- `impl Debug for LanguageServerToQuery`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

