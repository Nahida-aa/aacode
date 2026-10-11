---
id: CompletionResponse
title: CompletionResponse
---

# Struct: CompletionResponse

Defined in: [`packages/project/src/types/completion.rs:210`](../../../../packages/project/src/types/completion.rs#L210)

Response from a source of completions.

## Definition

```rust
pub struct CompletionResponse {
    pub completions: Vec<Completion>,
    pub display_options: CompletionDisplayOptions,
    // When false, indicates that the list is complete and does not need to be re-queried if it can be filtered instead.
    pub is_incomplete: bool,
}
```

## Fields

### is_incomplete

Defined in: [`packages/project/src/types/completion.rs:215`](../../../../packages/project/src/types/completion.rs#L215)

When false, indicates that the list is complete and does not need to be re-queried if it
can be filtered instead.

## Trait Implementations

- `impl Borrow for CompletionResponse`
- `impl BorrowMut for CompletionResponse`
- `impl Into for CompletionResponse`
- `impl From for CompletionResponse`
- `impl TryInto for CompletionResponse`
- `impl TryFrom for CompletionResponse`
- `impl Any for CompletionResponse`
- `impl VZip for CompletionResponse`
- `impl CastableFrom for CompletionResponse`
- `impl CastableFrom for CompletionResponse`
- `impl Read for CompletionResponse`
- `impl IntoEither for CompletionResponse`
- `impl ErasedDestructor for CompletionResponse`
- `impl Same for CompletionResponse`
- `impl Pointable for CompletionResponse`
- `impl Instrument for CompletionResponse`
- `impl WithSubscriber for CompletionResponse`
- `impl FromAngle for CompletionResponse`
- `impl IntoAngle for CompletionResponse`
- `impl IntoCam16Unclamped for CompletionResponse`
- `impl Cam16IntoUnclamped for CompletionResponse`
- `impl ArraysFrom for CompletionResponse`
- `impl ArraysInto for CompletionResponse`
- `impl ComponentsFrom for CompletionResponse`
- `impl TryComponentsInto for CompletionResponse`
- `impl UintsFrom for CompletionResponse`
- `impl UintsInto for CompletionResponse`
- `impl AdaptIntoUnclamped for CompletionResponse`
- `impl AdaptInto for CompletionResponse`
- `impl IntoColor for CompletionResponse`
- `impl IntoColorUnclamped for CompletionResponse`
- `impl TryIntoColor for CompletionResponse`
- `impl FromStimulus for CompletionResponse`
- `impl IntoStimulus for CompletionResponse`

## Auto Trait Implementations

`Freeze` `Send` `Sync` `Unpin` `UnsafeUnpin`

