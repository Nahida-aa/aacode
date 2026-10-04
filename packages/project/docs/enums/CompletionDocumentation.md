---
id: CompletionDocumentation
title: CompletionDocumentation
---

# Enum: CompletionDocumentation

Defined in: [`packages/project/src/lsp_store/mod.rs:16220`](../../../../packages/project/src/lsp_store/mod.rs#L16220)

## Definition

```rust
pub enum CompletionDocumentation {
    // There is no documentation for this completion.
    Undocumented,
    // A single line of documentation.
    SingleLine(SharedString),
    // Multiple lines of plain text documentation.
    MultiLinePlainText(SharedString),
    // Markdown documentation.
    MultiLineMarkdown(SharedString),
    // Both single line and multiple lines of plain text documentation.
    SingleLineAndMultiLinePlainText{ .. },
}
```

## Variants

### Undocumented

Defined in: [`packages/project/src/lsp_store/mod.rs:16222`](../../../../packages/project/src/lsp_store/mod.rs#L16222)

There is no documentation for this completion.


***

### SingleLine

Defined in: [`packages/project/src/lsp_store/mod.rs:16224`](../../../../packages/project/src/lsp_store/mod.rs#L16224)

A single line of documentation.


***

### MultiLinePlainText

Defined in: [`packages/project/src/lsp_store/mod.rs:16226`](../../../../packages/project/src/lsp_store/mod.rs#L16226)

Multiple lines of plain text documentation.


***

### MultiLineMarkdown

Defined in: [`packages/project/src/lsp_store/mod.rs:16228`](../../../../packages/project/src/lsp_store/mod.rs#L16228)

Markdown documentation.


***

### SingleLineAndMultiLinePlainText

Defined in: [`packages/project/src/lsp_store/mod.rs:16230`](../../../../packages/project/src/lsp_store/mod.rs#L16230)

Both single line and multiple lines of plain text documentation.

## Trait Implementations

- `impl Borrow for CompletionDocumentation`
- `impl BorrowMut for CompletionDocumentation`
- `impl CloneToUninit for CompletionDocumentation`
- `impl Into for CompletionDocumentation`
- `impl From for CompletionDocumentation`
- `impl TryInto for CompletionDocumentation`
- `impl TryFrom for CompletionDocumentation`
- `impl Any for CompletionDocumentation`
- `impl ToOwned for CompletionDocumentation`
- `impl DynClone for CompletionDocumentation`
- `impl VZip for CompletionDocumentation`
- `impl CastableFrom for CompletionDocumentation`
- `impl CastableFrom for CompletionDocumentation`
- `impl Read for CompletionDocumentation`
- `impl IntoEither for CompletionDocumentation`
- `impl ErasedDestructor for CompletionDocumentation`
- `impl Same for CompletionDocumentation`
- `impl Pointable for CompletionDocumentation`
- `impl Instrument for CompletionDocumentation`
- `impl WithSubscriber for CompletionDocumentation`
- `impl FromAngle for CompletionDocumentation`
- `impl IntoAngle for CompletionDocumentation`
- `impl IntoCam16Unclamped for CompletionDocumentation`
- `impl Cam16IntoUnclamped for CompletionDocumentation`
- `impl ArraysFrom for CompletionDocumentation`
- `impl ArraysInto for CompletionDocumentation`
- `impl ComponentsFrom for CompletionDocumentation`
- `impl TryComponentsInto for CompletionDocumentation`
- `impl UintsFrom for CompletionDocumentation`
- `impl UintsInto for CompletionDocumentation`
- `impl AdaptIntoUnclamped for CompletionDocumentation`
- `impl AdaptInto for CompletionDocumentation`
- `impl IntoColor for CompletionDocumentation`
- `impl IntoColorUnclamped for CompletionDocumentation`
- `impl TryIntoColor for CompletionDocumentation`
- `impl FromStimulus for CompletionDocumentation`
- `impl IntoStimulus for CompletionDocumentation`
- `impl Clone for CompletionDocumentation`
- `impl Debug for CompletionDocumentation`
- `impl From for CompletionDocumentation`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

