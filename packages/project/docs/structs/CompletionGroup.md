---
id: CompletionGroup
title: CompletionGroup
---

# Struct: CompletionGroup

Defined in: [`packages/project/src/types/completion.rs:34`](../../../../packages/project/src/types/completion.rs#L34)

Describes a visual group for a completion item in the menu.
When the group changes between consecutive completions, the menu inserts a divider.
If a label is provided, a non-selectable header row is also rendered.

## Definition

```rust
pub struct CompletionGroup {
    // Identity of this group, used to detect transitions between consecutive items.
    pub key: SharedString,
    // When set, a non-selectable header with this text is rendered below the divider.
    pub label: Option<SharedString>,
}
```

## Fields

### key

Defined in: [`packages/project/src/types/completion.rs:36`](../../../../packages/project/src/types/completion.rs#L36)

Identity of this group, used to detect transitions between consecutive items.


***

### label

Defined in: [`packages/project/src/types/completion.rs:38`](../../../../packages/project/src/types/completion.rs#L38)

When set, a non-selectable header with this text is rendered below the divider.

## Trait Implementations

- `impl Borrow for CompletionGroup`
- `impl BorrowMut for CompletionGroup`
- `impl CloneToUninit for CompletionGroup`
- `impl Into for CompletionGroup`
- `impl From for CompletionGroup`
- `impl TryInto for CompletionGroup`
- `impl TryFrom for CompletionGroup`
- `impl Any for CompletionGroup`
- `impl ToOwned for CompletionGroup`
- `impl Equivalent for CompletionGroup`
- `impl DynClone for CompletionGroup`
- `impl VZip for CompletionGroup`
- `impl CastableFrom for CompletionGroup`
- `impl CastableFrom for CompletionGroup`
- `impl Read for CompletionGroup`
- `impl IntoEither for CompletionGroup`
- `impl ErasedDestructor for CompletionGroup`
- `impl Same for CompletionGroup`
- `impl Pointable for CompletionGroup`
- `impl Instrument for CompletionGroup`
- `impl WithSubscriber for CompletionGroup`
- `impl FromAngle for CompletionGroup`
- `impl IntoAngle for CompletionGroup`
- `impl IntoCam16Unclamped for CompletionGroup`
- `impl Cam16IntoUnclamped for CompletionGroup`
- `impl ArraysFrom for CompletionGroup`
- `impl ArraysInto for CompletionGroup`
- `impl ComponentsFrom for CompletionGroup`
- `impl TryComponentsInto for CompletionGroup`
- `impl UintsFrom for CompletionGroup`
- `impl UintsInto for CompletionGroup`
- `impl AdaptIntoUnclamped for CompletionGroup`
- `impl AdaptInto for CompletionGroup`
- `impl IntoColor for CompletionGroup`
- `impl IntoColorUnclamped for CompletionGroup`
- `impl TryIntoColor for CompletionGroup`
- `impl FromStimulus for CompletionGroup`
- `impl IntoStimulus for CompletionGroup`
- `impl Equivalent for CompletionGroup`
- `impl Clone for CompletionGroup`
- `impl Debug for CompletionGroup`
- `impl StructuralPartialEq for CompletionGroup`
- `impl PartialEq for CompletionGroup`
- `impl Eq for CompletionGroup`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

