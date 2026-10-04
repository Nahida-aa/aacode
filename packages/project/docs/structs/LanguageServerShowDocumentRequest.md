---
id: LanguageServerShowDocumentRequest
title: LanguageServerShowDocumentRequest
---

# Struct: LanguageServerShowDocumentRequest

Defined in: [`packages/project/src/lsp_store/mod.rs:15993`](../../../../packages/project/src/lsp_store/mod.rs#L15993)

## Definition

```rust
pub struct LanguageServerShowDocumentRequest {
    pub uri: Uri,
    pub external: bool,
    pub take_focus: bool,
    pub selection: Option<Range>,
}
```

_（存在非公开字段）_

## Implementations

### respond()

```rust
pub fn respond(self, success: bool)
```

Defined in: [`packages/project/src/lsp_store/mod.rs:16002`](../../../../packages/project/src/lsp_store/mod.rs#L16002)

#### Parameters

##### success

`bool`

## Trait Implementations

- `impl Borrow for LanguageServerShowDocumentRequest`
- `impl BorrowMut for LanguageServerShowDocumentRequest`
- `impl CloneToUninit for LanguageServerShowDocumentRequest`
- `impl Into for LanguageServerShowDocumentRequest`
- `impl From for LanguageServerShowDocumentRequest`
- `impl TryInto for LanguageServerShowDocumentRequest`
- `impl TryFrom for LanguageServerShowDocumentRequest`
- `impl Any for LanguageServerShowDocumentRequest`
- `impl ToOwned for LanguageServerShowDocumentRequest`
- `impl DynClone for LanguageServerShowDocumentRequest`
- `impl VZip for LanguageServerShowDocumentRequest`
- `impl CastableFrom for LanguageServerShowDocumentRequest`
- `impl CastableFrom for LanguageServerShowDocumentRequest`
- `impl Read for LanguageServerShowDocumentRequest`
- `impl IntoEither for LanguageServerShowDocumentRequest`
- `impl ErasedDestructor for LanguageServerShowDocumentRequest`
- `impl Same for LanguageServerShowDocumentRequest`
- `impl Pointable for LanguageServerShowDocumentRequest`
- `impl Instrument for LanguageServerShowDocumentRequest`
- `impl WithSubscriber for LanguageServerShowDocumentRequest`
- `impl FromAngle for LanguageServerShowDocumentRequest`
- `impl IntoAngle for LanguageServerShowDocumentRequest`
- `impl IntoCam16Unclamped for LanguageServerShowDocumentRequest`
- `impl Cam16IntoUnclamped for LanguageServerShowDocumentRequest`
- `impl ArraysFrom for LanguageServerShowDocumentRequest`
- `impl ArraysInto for LanguageServerShowDocumentRequest`
- `impl ComponentsFrom for LanguageServerShowDocumentRequest`
- `impl TryComponentsInto for LanguageServerShowDocumentRequest`
- `impl UintsFrom for LanguageServerShowDocumentRequest`
- `impl UintsInto for LanguageServerShowDocumentRequest`
- `impl AdaptIntoUnclamped for LanguageServerShowDocumentRequest`
- `impl AdaptInto for LanguageServerShowDocumentRequest`
- `impl IntoColor for LanguageServerShowDocumentRequest`
- `impl IntoColorUnclamped for LanguageServerShowDocumentRequest`
- `impl TryIntoColor for LanguageServerShowDocumentRequest`
- `impl FromStimulus for LanguageServerShowDocumentRequest`
- `impl IntoStimulus for LanguageServerShowDocumentRequest`
- `impl Clone for LanguageServerShowDocumentRequest`
- `impl Debug for LanguageServerShowDocumentRequest`
- `impl PartialEq for LanguageServerShowDocumentRequest`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

