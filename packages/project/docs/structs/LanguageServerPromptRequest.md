---
id: LanguageServerPromptRequest
title: LanguageServerPromptRequest
---

# Struct: LanguageServerPromptRequest

Defined in: [`packages/project/src/lsp_store/mod.rs:15940`](../../../../packages/project/src/lsp_store/mod.rs#L15940)

A prompt requested by LSP server.

## Definition

```rust
pub struct LanguageServerPromptRequest
{
    pub id: usize,
    pub level: PromptLevel,
    pub message: String,
    pub actions: Vec<MessageActionItem>,
    pub lsp_name: String,
}
```

_（存在非公开字段）_

## Implementations

### new()

```rust
pub fn new(level: PromptLevel, message: String, actions: Vec<MessageActionItem>, lsp_name: String, response_channel: Sender<MessageActionItem>) -> Self
```

Defined in: [`packages/project/src/lsp_store/mod.rs:15950`](../../../../packages/project/src/lsp_store/mod.rs#L15950)

#### Parameters

##### level

`PromptLevel`

##### message

`String`

##### actions

`Vec<MessageActionItem>`

##### lsp_name

`String`

##### response_channel

`Sender<MessageActionItem>`

#### Returns

`Self`


***

### respond()

```rust
pub async fn respond(self, index: usize) -> Option<()>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:15967`](../../../../packages/project/src/lsp_store/mod.rs#L15967)

#### Parameters

##### index

`usize`

#### Returns

`Option<()>`

## Trait Implementations

- `impl Borrow for LanguageServerPromptRequest`
- `impl BorrowMut for LanguageServerPromptRequest`
- `impl CloneToUninit for LanguageServerPromptRequest`
- `impl Into for LanguageServerPromptRequest`
- `impl From for LanguageServerPromptRequest`
- `impl TryInto for LanguageServerPromptRequest`
- `impl TryFrom for LanguageServerPromptRequest`
- `impl Any for LanguageServerPromptRequest`
- `impl ToOwned for LanguageServerPromptRequest`
- `impl DynClone for LanguageServerPromptRequest`
- `impl VZip for LanguageServerPromptRequest`
- `impl CastableFrom for LanguageServerPromptRequest`
- `impl CastableFrom for LanguageServerPromptRequest`
- `impl Read for LanguageServerPromptRequest`
- `impl IntoEither for LanguageServerPromptRequest`
- `impl ErasedDestructor for LanguageServerPromptRequest`
- `impl Same for LanguageServerPromptRequest`
- `impl Pointable for LanguageServerPromptRequest`
- `impl Instrument for LanguageServerPromptRequest`
- `impl WithSubscriber for LanguageServerPromptRequest`
- `impl FromAngle for LanguageServerPromptRequest`
- `impl IntoAngle for LanguageServerPromptRequest`
- `impl IntoCam16Unclamped for LanguageServerPromptRequest`
- `impl Cam16IntoUnclamped for LanguageServerPromptRequest`
- `impl ArraysFrom for LanguageServerPromptRequest`
- `impl ArraysInto for LanguageServerPromptRequest`
- `impl ComponentsFrom for LanguageServerPromptRequest`
- `impl TryComponentsInto for LanguageServerPromptRequest`
- `impl UintsFrom for LanguageServerPromptRequest`
- `impl UintsInto for LanguageServerPromptRequest`
- `impl AdaptIntoUnclamped for LanguageServerPromptRequest`
- `impl AdaptInto for LanguageServerPromptRequest`
- `impl IntoColor for LanguageServerPromptRequest`
- `impl IntoColorUnclamped for LanguageServerPromptRequest`
- `impl TryIntoColor for LanguageServerPromptRequest`
- `impl FromStimulus for LanguageServerPromptRequest`
- `impl IntoStimulus for LanguageServerPromptRequest`
- `impl Clone for LanguageServerPromptRequest`
- `impl Debug for LanguageServerPromptRequest`
- `impl PartialEq for LanguageServerPromptRequest`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

