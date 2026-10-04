---
id: CodeLensActionId
title: CodeLensActionId
---

# Struct: CodeLensActionId

Defined in: [`packages/project/src/lsp_store/code_lens.rs:41`](../../../../packages/project/src/lsp_store/code_lens.rs#L41)

Opaque per-action identifier issued by [`LspStore`](LspStore.md) at fetch time.

LSP `CodeLens.data` is the server's private payload for resolve
round-trips, so we can't use it (or anything derived from it) to
disambiguate sibling lenses that share the same buffer `range`
(TypeScript's references + implementations is the canonical case).
We tag every cached action with this id and require it back on resolve
so each lens routes to its own request and slot.

Ids are issued in fetch order; sorting by id reproduces server-emit
order, which is how callers recover a stable render order without
paying for an ordered map.

## Definition

```rust
pub struct CodeLensActionId
```

## Trait Implementations

- `impl Borrow for CodeLensActionId`
- `impl BorrowMut for CodeLensActionId`
- `impl CloneToUninit for CodeLensActionId`
- `impl Into for CodeLensActionId`
- `impl From for CodeLensActionId`
- `impl TryInto for CodeLensActionId`
- `impl TryFrom for CodeLensActionId`
- `impl Any for CodeLensActionId`
- `impl ToOwned for CodeLensActionId`
- `impl Equivalent for CodeLensActionId`
- `impl Comparable for CodeLensActionId`
- `impl DynClone for CodeLensActionId`
- `impl VZip for CodeLensActionId`
- `impl CastableFrom for CodeLensActionId`
- `impl CastableFrom for CodeLensActionId`
- `impl Read for CodeLensActionId`
- `impl IntoEither for CodeLensActionId`
- `impl ErasedDestructor for CodeLensActionId`
- `impl Same for CodeLensActionId`
- `impl Pointable for CodeLensActionId`
- `impl MapSeekTarget for CodeLensActionId`
- `impl Instrument for CodeLensActionId`
- `impl WithSubscriber for CodeLensActionId`
- `impl FromAngle for CodeLensActionId`
- `impl IntoAngle for CodeLensActionId`
- `impl IntoCam16Unclamped for CodeLensActionId`
- `impl Cam16IntoUnclamped for CodeLensActionId`
- `impl ArraysFrom for CodeLensActionId`
- `impl ArraysInto for CodeLensActionId`
- `impl ComponentsFrom for CodeLensActionId`
- `impl TryComponentsInto for CodeLensActionId`
- `impl UintsFrom for CodeLensActionId`
- `impl UintsInto for CodeLensActionId`
- `impl AdaptIntoUnclamped for CodeLensActionId`
- `impl AdaptInto for CodeLensActionId`
- `impl IntoColor for CodeLensActionId`
- `impl IntoColorUnclamped for CodeLensActionId`
- `impl TryIntoColor for CodeLensActionId`
- `impl FromStimulus for CodeLensActionId`
- `impl IntoStimulus for CodeLensActionId`
- `impl Equivalent for CodeLensActionId`
- `impl ResetDiscriminant for CodeLensActionId`
- `impl Copy for CodeLensActionId`
- `impl Clone for CodeLensActionId`
- `impl Hash for CodeLensActionId`
- `impl StructuralPartialEq for CodeLensActionId`
- `impl PartialEq for CodeLensActionId`
- `impl Eq for CodeLensActionId`
- `impl PartialOrd for CodeLensActionId`
- `impl Ord for CodeLensActionId`
- `impl Debug for CodeLensActionId`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

