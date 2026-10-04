---
id: LanguageServerProgress
title: LanguageServerProgress
---

# Struct: LanguageServerProgress

Defined in: [`packages/project/src/lsp_store/mod.rs:16165`](../../../../packages/project/src/lsp_store/mod.rs#L16165)

## Definition

```rust
pub struct LanguageServerProgress
{
    pub is_disk_based_diagnostics_progress: bool,
    pub is_cancellable: bool,
    pub title: Option<String>,
    pub message: Option<String>,
    pub percentage: Option<usize>,
    pub last_update_at: Instant,
}
```

## Trait Implementations

- `impl Borrow for LanguageServerProgress`
- `impl BorrowMut for LanguageServerProgress`
- `impl CloneToUninit for LanguageServerProgress`
- `impl Into for LanguageServerProgress`
- `impl From for LanguageServerProgress`
- `impl TryInto for LanguageServerProgress`
- `impl TryFrom for LanguageServerProgress`
- `impl Any for LanguageServerProgress`
- `impl ToOwned for LanguageServerProgress`
- `impl Serialize for LanguageServerProgress`
- `impl DynClone for LanguageServerProgress`
- `impl VZip for LanguageServerProgress`
- `impl CastableFrom for LanguageServerProgress`
- `impl CastableFrom for LanguageServerProgress`
- `impl Read for LanguageServerProgress`
- `impl IntoEither for LanguageServerProgress`
- `impl ErasedDestructor for LanguageServerProgress`
- `impl Same for LanguageServerProgress`
- `impl Pointable for LanguageServerProgress`
- `impl Instrument for LanguageServerProgress`
- `impl WithSubscriber for LanguageServerProgress`
- `impl FromAngle for LanguageServerProgress`
- `impl IntoAngle for LanguageServerProgress`
- `impl IntoCam16Unclamped for LanguageServerProgress`
- `impl Cam16IntoUnclamped for LanguageServerProgress`
- `impl ArraysFrom for LanguageServerProgress`
- `impl ArraysInto for LanguageServerProgress`
- `impl ComponentsFrom for LanguageServerProgress`
- `impl TryComponentsInto for LanguageServerProgress`
- `impl UintsFrom for LanguageServerProgress`
- `impl UintsInto for LanguageServerProgress`
- `impl AdaptIntoUnclamped for LanguageServerProgress`
- `impl AdaptInto for LanguageServerProgress`
- `impl IntoColor for LanguageServerProgress`
- `impl IntoColorUnclamped for LanguageServerProgress`
- `impl TryIntoColor for LanguageServerProgress`
- `impl FromStimulus for LanguageServerProgress`
- `impl IntoStimulus for LanguageServerProgress`
- `impl Clone for LanguageServerProgress`
- `impl Debug for LanguageServerProgress`
- `impl Serialize for LanguageServerProgress`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

