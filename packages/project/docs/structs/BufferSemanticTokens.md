---
id: BufferSemanticTokens
title: BufferSemanticTokens
---

# Struct: BufferSemanticTokens

Defined in: [`packages/project/src/lsp_store/semantic_tokens.rs:635`](../../../../packages/project/src/lsp_store/semantic_tokens.rs#L635)

## Definition

```rust
pub struct BufferSemanticTokens
{
    pub tokens: Option<HashMap<LanguageServerId, Arc<[BufferSemanticToken]>>>,
}
```

## Trait Implementations

- `impl Borrow for BufferSemanticTokens`
- `impl BorrowMut for BufferSemanticTokens`
- `impl CloneToUninit for BufferSemanticTokens`
- `impl Into for BufferSemanticTokens`
- `impl From for BufferSemanticTokens`
- `impl TryInto for BufferSemanticTokens`
- `impl TryFrom for BufferSemanticTokens`
- `impl Any for BufferSemanticTokens`
- `impl ToOwned for BufferSemanticTokens`
- `impl DynClone for BufferSemanticTokens`
- `impl VZip for BufferSemanticTokens`
- `impl CastableFrom for BufferSemanticTokens`
- `impl CastableFrom for BufferSemanticTokens`
- `impl Read for BufferSemanticTokens`
- `impl IntoEither for BufferSemanticTokens`
- `impl ErasedDestructor for BufferSemanticTokens`
- `impl Same for BufferSemanticTokens`
- `impl ReadPrimitive for BufferSemanticTokens`
- `impl Pointable for BufferSemanticTokens`
- `impl Instrument for BufferSemanticTokens`
- `impl WithSubscriber for BufferSemanticTokens`
- `impl FromAngle for BufferSemanticTokens`
- `impl IntoAngle for BufferSemanticTokens`
- `impl IntoCam16Unclamped for BufferSemanticTokens`
- `impl Cam16IntoUnclamped for BufferSemanticTokens`
- `impl ArraysFrom for BufferSemanticTokens`
- `impl ArraysInto for BufferSemanticTokens`
- `impl ComponentsFrom for BufferSemanticTokens`
- `impl TryComponentsInto for BufferSemanticTokens`
- `impl UintsFrom for BufferSemanticTokens`
- `impl UintsInto for BufferSemanticTokens`
- `impl AdaptIntoUnclamped for BufferSemanticTokens`
- `impl AdaptInto for BufferSemanticTokens`
- `impl IntoColor for BufferSemanticTokens`
- `impl IntoColorUnclamped for BufferSemanticTokens`
- `impl TryIntoColor for BufferSemanticTokens`
- `impl FromStimulus for BufferSemanticTokens`
- `impl IntoStimulus for BufferSemanticTokens`
- `impl Debug for BufferSemanticTokens`
- `impl Default for BufferSemanticTokens`
- `impl Clone for BufferSemanticTokens`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

