---
id: InlayId
title: InlayId
---

# Enum: InlayId

Defined in: [`packages/project/src/types/inlay_hint.rs:4`](../../../../packages/project/src/types/inlay_hint.rs#L4)

## Definition

```rust
pub enum InlayId {
    EditPrediction(usize),
    DebuggerValue(usize),
    Hint(usize),
    Color(usize),
    ReplResult(usize),
}
```

## Implementations

### id()

```rust
pub fn id(&self) -> usize
```

Defined in: [`packages/project/src/types/inlay_hint.rs:14`](../../../../packages/project/src/types/inlay_hint.rs#L14)

#### Returns

`usize`

## Trait Implementations

- `impl Borrow for InlayId`
- `impl BorrowMut for InlayId`
- `impl CloneToUninit for InlayId`
- `impl Into for InlayId`
- `impl From for InlayId`
- `impl TryInto for InlayId`
- `impl TryFrom for InlayId`
- `impl Any for InlayId`
- `impl ToOwned for InlayId`
- `impl Equivalent for InlayId`
- `impl Comparable for InlayId`
- `impl DynClone for InlayId`
- `impl VZip for InlayId`
- `impl CastableFrom for InlayId`
- `impl CastableFrom for InlayId`
- `impl Read for InlayId`
- `impl IntoEither for InlayId`
- `impl ErasedDestructor for InlayId`
- `impl Same for InlayId`
- `impl Pointable for InlayId`
- `impl MapSeekTarget for InlayId`
- `impl Instrument for InlayId`
- `impl WithSubscriber for InlayId`
- `impl FromAngle for InlayId`
- `impl IntoAngle for InlayId`
- `impl IntoCam16Unclamped for InlayId`
- `impl Cam16IntoUnclamped for InlayId`
- `impl ArraysFrom for InlayId`
- `impl ArraysInto for InlayId`
- `impl ComponentsFrom for InlayId`
- `impl TryComponentsInto for InlayId`
- `impl UintsFrom for InlayId`
- `impl UintsInto for InlayId`
- `impl AdaptIntoUnclamped for InlayId`
- `impl AdaptInto for InlayId`
- `impl IntoColor for InlayId`
- `impl IntoColorUnclamped for InlayId`
- `impl TryIntoColor for InlayId`
- `impl FromStimulus for InlayId`
- `impl IntoStimulus for InlayId`
- `impl Equivalent for InlayId`
- `impl ResetDiscriminant for InlayId`
- `impl Clone for InlayId`
- `impl Copy for InlayId`
- `impl Debug for InlayId`
- `impl StructuralPartialEq for InlayId`
- `impl PartialEq for InlayId`
- `impl Eq for InlayId`
- `impl PartialOrd for InlayId`
- `impl Ord for InlayId`
- `impl Hash for InlayId`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

