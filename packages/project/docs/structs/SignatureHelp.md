---
id: SignatureHelp
title: SignatureHelp
---

# Struct: SignatureHelp

Defined in: [`packages/project/src/lsp_command/signature_help.rs:11`](../../../../packages/project/src/lsp_command/signature_help.rs#L11)

## Definition

```rust
pub struct SignatureHelp
{
    pub active_signature: usize,
    pub signatures: Vec<SignatureHelpData>,
}
```

_（存在非公开字段）_

## Implementations

### new()

```rust
pub fn new(help: SignatureHelp, language_registry: Option<Arc<LanguageRegistry>>, lang_server_id: Option<LanguageServerId>, cx: &App) -> Option<Self>
```

Defined in: [`packages/project/src/lsp_command/signature_help.rs:33`](../../../../packages/project/src/lsp_command/signature_help.rs#L33)

#### Parameters

##### help

`SignatureHelp`

##### language_registry

`Option<Arc<LanguageRegistry>>`

##### lang_server_id

`Option<LanguageServerId>`

##### cx

`&App`

#### Returns

`Option<Self>`

## Trait Implementations

- `impl Borrow for SignatureHelp`
- `impl BorrowMut for SignatureHelp`
- `impl Into for SignatureHelp`
- `impl From for SignatureHelp`
- `impl TryInto for SignatureHelp`
- `impl TryFrom for SignatureHelp`
- `impl Any for SignatureHelp`
- `impl VZip for SignatureHelp`
- `impl CastableFrom for SignatureHelp`
- `impl CastableFrom for SignatureHelp`
- `impl Read for SignatureHelp`
- `impl IntoEither for SignatureHelp`
- `impl ErasedDestructor for SignatureHelp`
- `impl Same for SignatureHelp`
- `impl Pointable for SignatureHelp`
- `impl Instrument for SignatureHelp`
- `impl WithSubscriber for SignatureHelp`
- `impl FromAngle for SignatureHelp`
- `impl IntoAngle for SignatureHelp`
- `impl IntoCam16Unclamped for SignatureHelp`
- `impl Cam16IntoUnclamped for SignatureHelp`
- `impl ArraysFrom for SignatureHelp`
- `impl ArraysInto for SignatureHelp`
- `impl ComponentsFrom for SignatureHelp`
- `impl TryComponentsInto for SignatureHelp`
- `impl UintsFrom for SignatureHelp`
- `impl UintsInto for SignatureHelp`
- `impl AdaptIntoUnclamped for SignatureHelp`
- `impl AdaptInto for SignatureHelp`
- `impl IntoColor for SignatureHelp`
- `impl IntoColorUnclamped for SignatureHelp`
- `impl TryIntoColor for SignatureHelp`
- `impl FromStimulus for SignatureHelp`
- `impl IntoStimulus for SignatureHelp`
- `impl Debug for SignatureHelp`

## Auto Trait Implementations

`Freeze` `Send` `Sync` `Unpin` `UnsafeUnpin`

