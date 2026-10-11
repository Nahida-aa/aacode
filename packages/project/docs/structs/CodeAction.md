---
id: CodeAction
title: CodeAction
---

# Struct: CodeAction

Defined in: [`packages/project/src/types/code_action.rs:4`](../../../../packages/project/src/types/code_action.rs#L4)

## Definition

```rust
pub struct CodeAction {
    // The id of the language server that produced this code action.
    pub server_id: LanguageServerId,
    // The range of the buffer where this code action is applicable.
    pub range: Range<Anchor>,
    // The raw code action provided by the language server. Can be either an action or a command.
    pub lsp_action: LspAction,
    // Whether the action needs to be resolved using the language server.
    pub resolved: bool,
}
```

## Fields

### server_id

Defined in: [`packages/project/src/types/code_action.rs:6`](../../../../packages/project/src/types/code_action.rs#L6)

The id of the language server that produced this code action.


***

### range

Defined in: [`packages/project/src/types/code_action.rs:8`](../../../../packages/project/src/types/code_action.rs#L8)

The range of the buffer where this code action is applicable.


***

### lsp_action

Defined in: [`packages/project/src/types/code_action.rs:11`](../../../../packages/project/src/types/code_action.rs#L11)

The raw code action provided by the language server.
Can be either an action or a command.


***

### resolved

Defined in: [`packages/project/src/types/code_action.rs:13`](../../../../packages/project/src/types/code_action.rs#L13)

Whether the action needs to be resolved using the language server.

## Trait Implementations

- `impl Borrow for CodeAction`
- `impl BorrowMut for CodeAction`
- `impl CloneToUninit for CodeAction`
- `impl Into for CodeAction`
- `impl From for CodeAction`
- `impl TryInto for CodeAction`
- `impl TryFrom for CodeAction`
- `impl Any for CodeAction`
- `impl ToOwned for CodeAction`
- `impl DynClone for CodeAction`
- `impl VZip for CodeAction`
- `impl CastableFrom for CodeAction`
- `impl CastableFrom for CodeAction`
- `impl Read for CodeAction`
- `impl IntoEither for CodeAction`
- `impl ErasedDestructor for CodeAction`
- `impl Same for CodeAction`
- `impl Pointable for CodeAction`
- `impl Instrument for CodeAction`
- `impl WithSubscriber for CodeAction`
- `impl FromAngle for CodeAction`
- `impl IntoAngle for CodeAction`
- `impl IntoCam16Unclamped for CodeAction`
- `impl Cam16IntoUnclamped for CodeAction`
- `impl ArraysFrom for CodeAction`
- `impl ArraysInto for CodeAction`
- `impl ComponentsFrom for CodeAction`
- `impl TryComponentsInto for CodeAction`
- `impl UintsFrom for CodeAction`
- `impl UintsInto for CodeAction`
- `impl AdaptIntoUnclamped for CodeAction`
- `impl AdaptInto for CodeAction`
- `impl IntoColor for CodeAction`
- `impl IntoColorUnclamped for CodeAction`
- `impl TryIntoColor for CodeAction`
- `impl FromStimulus for CodeAction`
- `impl IntoStimulus for CodeAction`
- `impl Clone for CodeAction`
- `impl Debug for CodeAction`
- `impl StructuralPartialEq for CodeAction`
- `impl PartialEq for CodeAction`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

