---
id: CompletionIntent
title: CompletionIntent
---

# Enum: CompletionIntent

Defined in: [`packages/project/src/types/completion.rs:4`](../../../../packages/project/src/types/completion.rs#L4)

## Definition

```rust
pub enum CompletionIntent {
    // The user intends to 'commit' this result, if possible. Completion confirmations should run side effects. For LSP completions, will respect the setting `completions.lsp_insert_mode`.
    Complete,
    // Similar to [Self::Complete], but behaves like `lsp_insert_mode` is set to `insert`.
    CompleteWithInsert,
    // Similar to [Self::Complete], but behaves like `lsp_insert_mode` is set to `replace`.
    CompleteWithReplace,
    // The user intends to continue 'composing' this completion. Completion confirmations should not run side effects and let the user continue composing their action.
    Compose,
}
```

## Variants

### Complete

Defined in: [`packages/project/src/types/completion.rs:9`](../../../../packages/project/src/types/completion.rs#L9)

The user intends to 'commit' this result, if possible.
Completion confirmations should run side effects.

For LSP completions, will respect the setting `completions.lsp_insert_mode`.


***

### CompleteWithInsert

Defined in: [`packages/project/src/types/completion.rs:11`](../../../../packages/project/src/types/completion.rs#L11)

Similar to [Self::Complete](CompletionIntent.md), but behaves like `lsp_insert_mode` is set to `insert`.


***

### CompleteWithReplace

Defined in: [`packages/project/src/types/completion.rs:13`](../../../../packages/project/src/types/completion.rs#L13)

Similar to [Self::Complete](CompletionIntent.md), but behaves like `lsp_insert_mode` is set to `replace`.


***

### Compose

Defined in: [`packages/project/src/types/completion.rs:17`](../../../../packages/project/src/types/completion.rs#L17)

The user intends to continue 'composing' this completion.
Completion confirmations should not run side effects and
let the user continue composing their action.

## Implementations

### is_complete()

```rust
pub fn is_complete(&self) -> bool
```

Defined in: [`packages/project/src/types/completion.rs:21`](../../../../packages/project/src/types/completion.rs#L21)

#### Returns

`bool`


***

### is_compose()

```rust
pub fn is_compose(&self) -> bool
```

Defined in: [`packages/project/src/types/completion.rs:25`](../../../../packages/project/src/types/completion.rs#L25)

#### Returns

`bool`

## Trait Implementations

- `impl Borrow for CompletionIntent`
- `impl BorrowMut for CompletionIntent`
- `impl CloneToUninit for CompletionIntent`
- `impl Into for CompletionIntent`
- `impl From for CompletionIntent`
- `impl TryInto for CompletionIntent`
- `impl TryFrom for CompletionIntent`
- `impl Any for CompletionIntent`
- `impl ToOwned for CompletionIntent`
- `impl Equivalent for CompletionIntent`
- `impl DynClone for CompletionIntent`
- `impl VZip for CompletionIntent`
- `impl CastableFrom for CompletionIntent`
- `impl CastableFrom for CompletionIntent`
- `impl Read for CompletionIntent`
- `impl IntoEither for CompletionIntent`
- `impl ErasedDestructor for CompletionIntent`
- `impl Same for CompletionIntent`
- `impl Pointable for CompletionIntent`
- `impl Instrument for CompletionIntent`
- `impl WithSubscriber for CompletionIntent`
- `impl FromAngle for CompletionIntent`
- `impl IntoAngle for CompletionIntent`
- `impl IntoCam16Unclamped for CompletionIntent`
- `impl Cam16IntoUnclamped for CompletionIntent`
- `impl ArraysFrom for CompletionIntent`
- `impl ArraysInto for CompletionIntent`
- `impl ComponentsFrom for CompletionIntent`
- `impl TryComponentsInto for CompletionIntent`
- `impl UintsFrom for CompletionIntent`
- `impl UintsInto for CompletionIntent`
- `impl AdaptIntoUnclamped for CompletionIntent`
- `impl AdaptInto for CompletionIntent`
- `impl IntoColor for CompletionIntent`
- `impl IntoColorUnclamped for CompletionIntent`
- `impl TryIntoColor for CompletionIntent`
- `impl FromStimulus for CompletionIntent`
- `impl IntoStimulus for CompletionIntent`
- `impl Equivalent for CompletionIntent`
- `impl ResetDiscriminant for CompletionIntent`
- `impl StructuralPartialEq for CompletionIntent`
- `impl PartialEq for CompletionIntent`
- `impl Eq for CompletionIntent`
- `impl Hash for CompletionIntent`
- `impl Debug for CompletionIntent`
- `impl Clone for CompletionIntent`
- `impl Copy for CompletionIntent`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

