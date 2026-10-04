---
id: ResolvedHint
title: ResolvedHint
---

# Enum: ResolvedHint

Defined in: [`packages/project/src/lsp_store/mod.rs:16279`](../../../../packages/project/src/lsp_store/mod.rs#L16279)

## Definition

```rust
pub enum ResolvedHint {
    Resolved(InlayHint),
    Resolving(Shared<Task<()>>),
}
```

## Trait Implementations

- `impl Borrow for ResolvedHint`
- `impl BorrowMut for ResolvedHint`
- `impl Into for ResolvedHint`
- `impl From for ResolvedHint`
- `impl TryInto for ResolvedHint`
- `impl TryFrom for ResolvedHint`
- `impl Any for ResolvedHint`
- `impl VZip for ResolvedHint`
- `impl CastableFrom for ResolvedHint`
- `impl CastableFrom for ResolvedHint`
- `impl Read for ResolvedHint`
- `impl IntoEither for ResolvedHint`
- `impl ErasedDestructor for ResolvedHint`
- `impl Same for ResolvedHint`
- `impl Pointable for ResolvedHint`
- `impl Instrument for ResolvedHint`
- `impl WithSubscriber for ResolvedHint`
- `impl FromAngle for ResolvedHint`
- `impl IntoAngle for ResolvedHint`
- `impl IntoCam16Unclamped for ResolvedHint`
- `impl Cam16IntoUnclamped for ResolvedHint`
- `impl ArraysFrom for ResolvedHint`
- `impl ArraysInto for ResolvedHint`
- `impl ComponentsFrom for ResolvedHint`
- `impl TryComponentsInto for ResolvedHint`
- `impl UintsFrom for ResolvedHint`
- `impl UintsInto for ResolvedHint`
- `impl AdaptIntoUnclamped for ResolvedHint`
- `impl AdaptInto for ResolvedHint`
- `impl IntoColor for ResolvedHint`
- `impl IntoColorUnclamped for ResolvedHint`
- `impl TryIntoColor for ResolvedHint`
- `impl FromStimulus for ResolvedHint`
- `impl IntoStimulus for ResolvedHint`

## Auto Trait Implementations

`Freeze` `Send` `Sync` `Unpin` `UnsafeUnpin`

