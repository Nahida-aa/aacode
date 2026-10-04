---
id: ParsedCompletionEdit
title: ParsedCompletionEdit
---

# Struct: ParsedCompletionEdit

Defined in: [`packages/project/src/lsp_command/mod.rs:3474`](../../../../packages/project/src/lsp_command/mod.rs#L3474)

## Definition

```rust
pub struct ParsedCompletionEdit
{
    pub replace_range: Range<Anchor>,
    pub insert_range: Option<Range<Anchor>>,
    pub new_text: String,
}
```

## Trait Implementations

- `impl Borrow for ParsedCompletionEdit`
- `impl BorrowMut for ParsedCompletionEdit`
- `impl Into for ParsedCompletionEdit`
- `impl From for ParsedCompletionEdit`
- `impl TryInto for ParsedCompletionEdit`
- `impl TryFrom for ParsedCompletionEdit`
- `impl Any for ParsedCompletionEdit`
- `impl VZip for ParsedCompletionEdit`
- `impl CastableFrom for ParsedCompletionEdit`
- `impl CastableFrom for ParsedCompletionEdit`
- `impl Read for ParsedCompletionEdit`
- `impl IntoEither for ParsedCompletionEdit`
- `impl ErasedDestructor for ParsedCompletionEdit`
- `impl Same for ParsedCompletionEdit`
- `impl Pointable for ParsedCompletionEdit`
- `impl Instrument for ParsedCompletionEdit`
- `impl WithSubscriber for ParsedCompletionEdit`
- `impl FromAngle for ParsedCompletionEdit`
- `impl IntoAngle for ParsedCompletionEdit`
- `impl IntoCam16Unclamped for ParsedCompletionEdit`
- `impl Cam16IntoUnclamped for ParsedCompletionEdit`
- `impl ArraysFrom for ParsedCompletionEdit`
- `impl ArraysInto for ParsedCompletionEdit`
- `impl ComponentsFrom for ParsedCompletionEdit`
- `impl TryComponentsInto for ParsedCompletionEdit`
- `impl UintsFrom for ParsedCompletionEdit`
- `impl UintsInto for ParsedCompletionEdit`
- `impl AdaptIntoUnclamped for ParsedCompletionEdit`
- `impl AdaptInto for ParsedCompletionEdit`
- `impl IntoColor for ParsedCompletionEdit`
- `impl IntoColorUnclamped for ParsedCompletionEdit`
- `impl TryIntoColor for ParsedCompletionEdit`
- `impl FromStimulus for ParsedCompletionEdit`
- `impl IntoStimulus for ParsedCompletionEdit`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

