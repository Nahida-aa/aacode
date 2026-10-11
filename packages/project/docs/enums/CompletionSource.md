---
id: CompletionSource
title: CompletionSource
---

# Enum: CompletionSource

Defined in: [`packages/project/src/types/completion.rs:79`](../../../../packages/project/src/types/completion.rs#L79)

## Definition

```rust
pub enum CompletionSource {
    Lsp{ .. },
    Dap{ .. },
    Custom,
    BufferWord{ .. },
}
```

## Implementations

### server_id()

```rust
pub fn server_id(&self) -> Option<LanguageServerId>
```

Defined in: [`packages/project/src/types/completion.rs:104`](../../../../packages/project/src/types/completion.rs#L104)

#### Returns

`Option<LanguageServerId>`


***

### filter_text()

```rust
pub fn filter_text(&self) -> Option<&str>
```

Defined in: [`packages/project/src/types/completion.rs:112`](../../../../packages/project/src/types/completion.rs#L112)

#### Returns

`Option<&str>`


***

### lsp_completion()

```rust
pub fn lsp_completion(&self, apply_defaults: bool) -> Option<Cow<'_, CompletionItem>>
```

Defined in: [`packages/project/src/types/completion.rs:125`](../../../../packages/project/src/types/completion.rs#L125)

#### Parameters

##### apply_defaults

`bool`

#### Returns

`Option<Cow<'_, CompletionItem>>`

## Trait Implementations

- `impl Borrow for CompletionSource`
- `impl BorrowMut for CompletionSource`
- `impl CloneToUninit for CompletionSource`
- `impl Into for CompletionSource`
- `impl From for CompletionSource`
- `impl TryInto for CompletionSource`
- `impl TryFrom for CompletionSource`
- `impl Any for CompletionSource`
- `impl ToOwned for CompletionSource`
- `impl DynClone for CompletionSource`
- `impl VZip for CompletionSource`
- `impl CastableFrom for CompletionSource`
- `impl CastableFrom for CompletionSource`
- `impl Read for CompletionSource`
- `impl IntoEither for CompletionSource`
- `impl ErasedDestructor for CompletionSource`
- `impl Same for CompletionSource`
- `impl Pointable for CompletionSource`
- `impl Instrument for CompletionSource`
- `impl WithSubscriber for CompletionSource`
- `impl FromAngle for CompletionSource`
- `impl IntoAngle for CompletionSource`
- `impl IntoCam16Unclamped for CompletionSource`
- `impl Cam16IntoUnclamped for CompletionSource`
- `impl ArraysFrom for CompletionSource`
- `impl ArraysInto for CompletionSource`
- `impl ComponentsFrom for CompletionSource`
- `impl TryComponentsInto for CompletionSource`
- `impl UintsFrom for CompletionSource`
- `impl UintsInto for CompletionSource`
- `impl AdaptIntoUnclamped for CompletionSource`
- `impl AdaptInto for CompletionSource`
- `impl IntoColor for CompletionSource`
- `impl IntoColorUnclamped for CompletionSource`
- `impl TryIntoColor for CompletionSource`
- `impl FromStimulus for CompletionSource`
- `impl IntoStimulus for CompletionSource`
- `impl Debug for CompletionSource`
- `impl Clone for CompletionSource`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

