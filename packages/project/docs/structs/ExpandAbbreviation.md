---
id: ExpandAbbreviation
title: ExpandAbbreviation
---

# Struct: ExpandAbbreviation

Defined in: [`packages/project/src/lsp_store/emmet_ext.rs:59`](../../../../packages/project/src/lsp_store/emmet_ext.rs#L59)

## Definition

```rust
pub struct ExpandAbbreviation {
    pub abbreviation: String,
    pub text: Option<Vec<String>>,
    pub language: String,
    pub server_id: LanguageServerId,
    pub indent: String,
    pub base_indent: String,
    pub comment_filter: bool,
    pub bem_filter: bool,
}
```

## Trait Implementations

- `impl Borrow for ExpandAbbreviation`
- `impl BorrowMut for ExpandAbbreviation`
- `impl Into for ExpandAbbreviation`
- `impl From for ExpandAbbreviation`
- `impl TryInto for ExpandAbbreviation`
- `impl TryFrom for ExpandAbbreviation`
- `impl Any for ExpandAbbreviation`
- `impl VZip for ExpandAbbreviation`
- `impl CastableFrom for ExpandAbbreviation`
- `impl CastableFrom for ExpandAbbreviation`
- `impl Read for ExpandAbbreviation`
- `impl IntoEither for ExpandAbbreviation`
- `impl ErasedDestructor for ExpandAbbreviation`
- `impl Same for ExpandAbbreviation`
- `impl Pointable for ExpandAbbreviation`
- `impl Instrument for ExpandAbbreviation`
- `impl WithSubscriber for ExpandAbbreviation`
- `impl FromAngle for ExpandAbbreviation`
- `impl IntoAngle for ExpandAbbreviation`
- `impl IntoCam16Unclamped for ExpandAbbreviation`
- `impl Cam16IntoUnclamped for ExpandAbbreviation`
- `impl ArraysFrom for ExpandAbbreviation`
- `impl ArraysInto for ExpandAbbreviation`
- `impl ComponentsFrom for ExpandAbbreviation`
- `impl TryComponentsInto for ExpandAbbreviation`
- `impl UintsFrom for ExpandAbbreviation`
- `impl UintsInto for ExpandAbbreviation`
- `impl AdaptIntoUnclamped for ExpandAbbreviation`
- `impl AdaptInto for ExpandAbbreviation`
- `impl IntoColor for ExpandAbbreviation`
- `impl IntoColorUnclamped for ExpandAbbreviation`
- `impl TryIntoColor for ExpandAbbreviation`
- `impl FromStimulus for ExpandAbbreviation`
- `impl IntoStimulus for ExpandAbbreviation`
- `impl Debug for ExpandAbbreviation`
- `impl LspCommand for ExpandAbbreviation`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

