---
id: BufferSemanticToken
title: BufferSemanticToken
---

# Struct: BufferSemanticToken

Defined in: [`packages/project/src/lsp_store/semantic_tokens.rs:643`](../../../../packages/project/src/lsp_store/semantic_tokens.rs#L643)

## Definition

```rust
pub struct BufferSemanticToken
{
    // The range of the token in the buffer. Guaranteed to contain a buffer id.
    pub range: Range<Anchor>,
    pub token_type: TokenType,
    pub token_modifiers: u32,
}
```

## Fields

### range

Defined in: [`packages/project/src/lsp_store/semantic_tokens.rs:647`](../../../../packages/project/src/lsp_store/semantic_tokens.rs#L647)

The range of the token in the buffer.

Guaranteed to contain a buffer id.

## Trait Implementations

- `impl Borrow for BufferSemanticToken`
- `impl BorrowMut for BufferSemanticToken`
- `impl CloneToUninit for BufferSemanticToken`
- `impl Into for BufferSemanticToken`
- `impl From for BufferSemanticToken`
- `impl TryInto for BufferSemanticToken`
- `impl TryFrom for BufferSemanticToken`
- `impl Any for BufferSemanticToken`
- `impl ToOwned for BufferSemanticToken`
- `impl DynClone for BufferSemanticToken`
- `impl VZip for BufferSemanticToken`
- `impl CastableFrom for BufferSemanticToken`
- `impl CastableFrom for BufferSemanticToken`
- `impl Read for BufferSemanticToken`
- `impl IntoEither for BufferSemanticToken`
- `impl ErasedDestructor for BufferSemanticToken`
- `impl Same for BufferSemanticToken`
- `impl Pointable for BufferSemanticToken`
- `impl Instrument for BufferSemanticToken`
- `impl WithSubscriber for BufferSemanticToken`
- `impl FromAngle for BufferSemanticToken`
- `impl IntoAngle for BufferSemanticToken`
- `impl IntoCam16Unclamped for BufferSemanticToken`
- `impl Cam16IntoUnclamped for BufferSemanticToken`
- `impl ArraysFrom for BufferSemanticToken`
- `impl ArraysInto for BufferSemanticToken`
- `impl ComponentsFrom for BufferSemanticToken`
- `impl TryComponentsInto for BufferSemanticToken`
- `impl UintsFrom for BufferSemanticToken`
- `impl UintsInto for BufferSemanticToken`
- `impl AdaptIntoUnclamped for BufferSemanticToken`
- `impl AdaptInto for BufferSemanticToken`
- `impl IntoColor for BufferSemanticToken`
- `impl IntoColorUnclamped for BufferSemanticToken`
- `impl TryIntoColor for BufferSemanticToken`
- `impl FromStimulus for BufferSemanticToken`
- `impl IntoStimulus for BufferSemanticToken`
- `impl Debug for BufferSemanticToken`
- `impl Clone for BufferSemanticToken`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

