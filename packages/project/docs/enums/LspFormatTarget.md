---
id: LspFormatTarget
title: LspFormatTarget
---

# Enum: LspFormatTarget

Defined in: [`packages/project/src/lsp_store/mod.rs:247`](../../../../packages/project/src/lsp_store/mod.rs#L247)

## Definition

```rust
pub enum LspFormatTarget
{
    Buffers,
    Ranges(BTreeMap<BufferId, Vec<Range<Anchor>>>),
}
```

## Trait Implementations

- `impl Borrow for LspFormatTarget`
- `impl BorrowMut for LspFormatTarget`
- `impl Into for LspFormatTarget`
- `impl From for LspFormatTarget`
- `impl TryInto for LspFormatTarget`
- `impl TryFrom for LspFormatTarget`
- `impl Any for LspFormatTarget`
- `impl VZip for LspFormatTarget`
- `impl CastableFrom for LspFormatTarget`
- `impl CastableFrom for LspFormatTarget`
- `impl Read for LspFormatTarget`
- `impl IntoEither for LspFormatTarget`
- `impl ErasedDestructor for LspFormatTarget`
- `impl Same for LspFormatTarget`
- `impl Pointable for LspFormatTarget`
- `impl Instrument for LspFormatTarget`
- `impl WithSubscriber for LspFormatTarget`
- `impl FromAngle for LspFormatTarget`
- `impl IntoAngle for LspFormatTarget`
- `impl IntoCam16Unclamped for LspFormatTarget`
- `impl Cam16IntoUnclamped for LspFormatTarget`
- `impl ArraysFrom for LspFormatTarget`
- `impl ArraysInto for LspFormatTarget`
- `impl ComponentsFrom for LspFormatTarget`
- `impl TryComponentsInto for LspFormatTarget`
- `impl UintsFrom for LspFormatTarget`
- `impl UintsInto for LspFormatTarget`
- `impl AdaptIntoUnclamped for LspFormatTarget`
- `impl AdaptInto for LspFormatTarget`
- `impl IntoColor for LspFormatTarget`
- `impl IntoColorUnclamped for LspFormatTarget`
- `impl TryIntoColor for LspFormatTarget`
- `impl FromStimulus for LspFormatTarget`
- `impl IntoStimulus for LspFormatTarget`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

