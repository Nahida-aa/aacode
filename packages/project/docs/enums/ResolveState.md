---
id: ResolveState
title: ResolveState
---

# Enum: ResolveState

Defined in: [`packages/project/src/types/inlay_hint.rs:38`](../../../../packages/project/src/types/inlay_hint.rs#L38)

The user's intent behind a given completion confirmation.

## Definition

```rust
pub enum ResolveState {
    Resolved,
    CanResolve(LanguageServerId, Option<LSPAny>),
    Resolving,
}
```

## Trait Implementations

- `impl Borrow for ResolveState`
- `impl BorrowMut for ResolveState`
- `impl CloneToUninit for ResolveState`
- `impl Into for ResolveState`
- `impl From for ResolveState`
- `impl TryInto for ResolveState`
- `impl TryFrom for ResolveState`
- `impl Any for ResolveState`
- `impl ToOwned for ResolveState`
- `impl Equivalent for ResolveState`
- `impl DynClone for ResolveState`
- `impl VZip for ResolveState`
- `impl CastableFrom for ResolveState`
- `impl CastableFrom for ResolveState`
- `impl Read for ResolveState`
- `impl IntoEither for ResolveState`
- `impl ErasedDestructor for ResolveState`
- `impl Same for ResolveState`
- `impl Pointable for ResolveState`
- `impl Instrument for ResolveState`
- `impl WithSubscriber for ResolveState`
- `impl FromAngle for ResolveState`
- `impl IntoAngle for ResolveState`
- `impl IntoCam16Unclamped for ResolveState`
- `impl Cam16IntoUnclamped for ResolveState`
- `impl ArraysFrom for ResolveState`
- `impl ArraysInto for ResolveState`
- `impl ComponentsFrom for ResolveState`
- `impl TryComponentsInto for ResolveState`
- `impl UintsFrom for ResolveState`
- `impl UintsInto for ResolveState`
- `impl AdaptIntoUnclamped for ResolveState`
- `impl AdaptInto for ResolveState`
- `impl IntoColor for ResolveState`
- `impl IntoColorUnclamped for ResolveState`
- `impl TryIntoColor for ResolveState`
- `impl FromStimulus for ResolveState`
- `impl IntoStimulus for ResolveState`
- `impl Equivalent for ResolveState`
- `impl StructuralPartialEq for ResolveState`
- `impl PartialEq for ResolveState`
- `impl Eq for ResolveState`
- `impl Debug for ResolveState`
- `impl Clone for ResolveState`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

