---
id: Symbol
title: Symbol
---

# Struct: Symbol

Defined in: [`packages/project/src/types/symbol.rs:4`](../../../../packages/project/src/types/symbol.rs#L4)

## Definition

```rust
pub struct Symbol {
    pub language_server_name: LanguageServerName,
    pub source_worktree_id: WorktreeId,
    pub source_language_server_id: LanguageServerId,
    pub path: SymbolLocation,
    pub label: CodeLabel,
    pub name: String,
    pub kind: SymbolKind,
    pub range: Range<Unclipped<PointUtf16>>,
    pub container_name: Option<String>,
}
```

## Trait Implementations

- `impl Borrow for Symbol`
- `impl BorrowMut for Symbol`
- `impl CloneToUninit for Symbol`
- `impl Into for Symbol`
- `impl From for Symbol`
- `impl TryInto for Symbol`
- `impl TryFrom for Symbol`
- `impl Any for Symbol`
- `impl ToOwned for Symbol`
- `impl DynClone for Symbol`
- `impl VZip for Symbol`
- `impl CastableFrom for Symbol`
- `impl CastableFrom for Symbol`
- `impl Read for Symbol`
- `impl IntoEither for Symbol`
- `impl ErasedDestructor for Symbol`
- `impl Same for Symbol`
- `impl Pointable for Symbol`
- `impl Instrument for Symbol`
- `impl WithSubscriber for Symbol`
- `impl FromAngle for Symbol`
- `impl IntoAngle for Symbol`
- `impl IntoCam16Unclamped for Symbol`
- `impl Cam16IntoUnclamped for Symbol`
- `impl ArraysFrom for Symbol`
- `impl ArraysInto for Symbol`
- `impl ComponentsFrom for Symbol`
- `impl TryComponentsInto for Symbol`
- `impl UintsFrom for Symbol`
- `impl UintsInto for Symbol`
- `impl AdaptIntoUnclamped for Symbol`
- `impl AdaptInto for Symbol`
- `impl IntoColor for Symbol`
- `impl IntoColorUnclamped for Symbol`
- `impl TryIntoColor for Symbol`
- `impl FromStimulus for Symbol`
- `impl IntoStimulus for Symbol`
- `impl Clone for Symbol`
- `impl Debug for Symbol`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

