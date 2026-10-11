---
id: LspAction
title: LspAction
---

# Enum: LspAction

Defined in: [`packages/project/src/types/code_action.rs:18`](../../../../packages/project/src/types/code_action.rs#L18)

An action sent back by a language server.

## Definition

```rust
pub enum LspAction {
    // An action with the full data, may have a command or may not. May require resolving.
    Action(Box<CodeAction>),
    // A command data to run as an action.
    Command(Command),
    // A code lens data to run as an action.
    CodeLens(CodeLens),
}
```

## Variants

### Action

Defined in: [`packages/project/src/types/code_action.rs:21`](../../../../packages/project/src/types/code_action.rs#L21)

An action with the full data, may have a command or may not.
May require resolving.


***

### Command

Defined in: [`packages/project/src/types/code_action.rs:23`](../../../../packages/project/src/types/code_action.rs#L23)

A command data to run as an action.


***

### CodeLens

Defined in: [`packages/project/src/types/code_action.rs:25`](../../../../packages/project/src/types/code_action.rs#L25)

A code lens data to run as an action.

## Implementations

### title()

```rust
pub fn title(&self) -> &str
```

Defined in: [`packages/project/src/types/code_action.rs:29`](../../../../packages/project/src/types/code_action.rs#L29)

#### Returns

`&str`


***

### action_kind()

```rust
pub fn action_kind(&self) -> Option<CodeActionKind>
```

Defined in: [`packages/project/src/types/code_action.rs:41`](../../../../packages/project/src/types/code_action.rs#L41)

#### Returns

`Option<CodeActionKind>`


***

### edit()

```rust
pub fn edit(&self) -> Option<&WorkspaceEdit>
```

Defined in: [`packages/project/src/types/code_action.rs:49`](../../../../packages/project/src/types/code_action.rs#L49)

#### Returns

`Option<&WorkspaceEdit>`


***

### command()

```rust
pub fn command(&self) -> Option<&Command>
```

Defined in: [`packages/project/src/types/code_action.rs:57`](../../../../packages/project/src/types/code_action.rs#L57)

#### Returns

`Option<&Command>`

## Trait Implementations

- `impl Borrow for LspAction`
- `impl BorrowMut for LspAction`
- `impl CloneToUninit for LspAction`
- `impl Into for LspAction`
- `impl From for LspAction`
- `impl TryInto for LspAction`
- `impl TryFrom for LspAction`
- `impl Any for LspAction`
- `impl ToOwned for LspAction`
- `impl DynClone for LspAction`
- `impl VZip for LspAction`
- `impl CastableFrom for LspAction`
- `impl CastableFrom for LspAction`
- `impl Read for LspAction`
- `impl IntoEither for LspAction`
- `impl ErasedDestructor for LspAction`
- `impl Same for LspAction`
- `impl Pointable for LspAction`
- `impl Instrument for LspAction`
- `impl WithSubscriber for LspAction`
- `impl FromAngle for LspAction`
- `impl IntoAngle for LspAction`
- `impl IntoCam16Unclamped for LspAction`
- `impl Cam16IntoUnclamped for LspAction`
- `impl ArraysFrom for LspAction`
- `impl ArraysInto for LspAction`
- `impl ComponentsFrom for LspAction`
- `impl TryComponentsInto for LspAction`
- `impl UintsFrom for LspAction`
- `impl UintsInto for LspAction`
- `impl AdaptIntoUnclamped for LspAction`
- `impl AdaptInto for LspAction`
- `impl IntoColor for LspAction`
- `impl IntoColorUnclamped for LspAction`
- `impl TryIntoColor for LspAction`
- `impl FromStimulus for LspAction`
- `impl IntoStimulus for LspAction`
- `impl Clone for LspAction`
- `impl Debug for LspAction`
- `impl StructuralPartialEq for LspAction`
- `impl PartialEq for LspAction`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

