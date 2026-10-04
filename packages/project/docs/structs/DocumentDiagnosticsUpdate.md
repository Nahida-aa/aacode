---
id: DocumentDiagnosticsUpdate
title: DocumentDiagnosticsUpdate
---

# Struct: DocumentDiagnosticsUpdate

Defined in: [`packages/project/src/lsp_store/mod.rs:292`](../../../../packages/project/src/lsp_store/mod.rs#L292)

## Definition

```rust
pub struct DocumentDiagnosticsUpdate<'a, D>
{
    pub diagnostics: D,
    pub result_id: Option<SharedString>,
    pub registration_id: Option<SharedString>,
    pub server_id: LanguageServerId,
    pub disk_based_sources: Cow<'a, [String]>,
}
```

## Trait Implementations

- `impl Borrow for DocumentDiagnosticsUpdate`
- `impl BorrowMut for DocumentDiagnosticsUpdate`
- `impl Into for DocumentDiagnosticsUpdate`
- `impl From for DocumentDiagnosticsUpdate`
- `impl TryInto for DocumentDiagnosticsUpdate`
- `impl TryFrom for DocumentDiagnosticsUpdate`
- `impl Any for DocumentDiagnosticsUpdate`
- `impl VZip for DocumentDiagnosticsUpdate`
- `impl CastableFrom for DocumentDiagnosticsUpdate`
- `impl CastableFrom for DocumentDiagnosticsUpdate`
- `impl Read for DocumentDiagnosticsUpdate`
- `impl IntoEither for DocumentDiagnosticsUpdate`
- `impl ErasedDestructor for DocumentDiagnosticsUpdate`
- `impl Same for DocumentDiagnosticsUpdate`
- `impl Pointable for DocumentDiagnosticsUpdate`
- `impl Instrument for DocumentDiagnosticsUpdate`
- `impl WithSubscriber for DocumentDiagnosticsUpdate`
- `impl FromAngle for DocumentDiagnosticsUpdate`
- `impl IntoAngle for DocumentDiagnosticsUpdate`
- `impl IntoCam16Unclamped for DocumentDiagnosticsUpdate`
- `impl Cam16IntoUnclamped for DocumentDiagnosticsUpdate`
- `impl ArraysFrom for DocumentDiagnosticsUpdate`
- `impl ArraysInto for DocumentDiagnosticsUpdate`
- `impl ComponentsFrom for DocumentDiagnosticsUpdate`
- `impl TryComponentsInto for DocumentDiagnosticsUpdate`
- `impl UintsFrom for DocumentDiagnosticsUpdate`
- `impl UintsInto for DocumentDiagnosticsUpdate`
- `impl AdaptIntoUnclamped for DocumentDiagnosticsUpdate`
- `impl AdaptInto for DocumentDiagnosticsUpdate`
- `impl IntoColor for DocumentDiagnosticsUpdate`
- `impl IntoColorUnclamped for DocumentDiagnosticsUpdate`
- `impl TryIntoColor for DocumentDiagnosticsUpdate`
- `impl FromStimulus for DocumentDiagnosticsUpdate`
- `impl IntoStimulus for DocumentDiagnosticsUpdate`
- `impl Debug for DocumentDiagnosticsUpdate`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

