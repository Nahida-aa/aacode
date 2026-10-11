---
id: DocumentSymbol
title: DocumentSymbol
---

# Struct: DocumentSymbol

Defined in: [`packages/project/src/types/symbol.rs:17`](../../../../packages/project/src/types/symbol.rs#L17)

## Definition

```rust
pub struct DocumentSymbol {
    pub name: String,
    pub kind: SymbolKind,
    pub range: Range<Unclipped<PointUtf16>>,
    pub selection_range: Range<Unclipped<PointUtf16>>,
    pub children: Vec<DocumentSymbol>,
}
```

## Trait Implementations

- `impl Borrow for DocumentSymbol`
- `impl BorrowMut for DocumentSymbol`
- `impl CloneToUninit for DocumentSymbol`
- `impl Into for DocumentSymbol`
- `impl From for DocumentSymbol`
- `impl TryInto for DocumentSymbol`
- `impl TryFrom for DocumentSymbol`
- `impl Any for DocumentSymbol`
- `impl ToOwned for DocumentSymbol`
- `impl DynClone for DocumentSymbol`
- `impl VZip for DocumentSymbol`
- `impl CastableFrom for DocumentSymbol`
- `impl CastableFrom for DocumentSymbol`
- `impl Read for DocumentSymbol`
- `impl IntoEither for DocumentSymbol`
- `impl ErasedDestructor for DocumentSymbol`
- `impl Same for DocumentSymbol`
- `impl Pointable for DocumentSymbol`
- `impl Instrument for DocumentSymbol`
- `impl WithSubscriber for DocumentSymbol`
- `impl FromAngle for DocumentSymbol`
- `impl IntoAngle for DocumentSymbol`
- `impl IntoCam16Unclamped for DocumentSymbol`
- `impl Cam16IntoUnclamped for DocumentSymbol`
- `impl ArraysFrom for DocumentSymbol`
- `impl ArraysInto for DocumentSymbol`
- `impl ComponentsFrom for DocumentSymbol`
- `impl TryComponentsInto for DocumentSymbol`
- `impl UintsFrom for DocumentSymbol`
- `impl UintsInto for DocumentSymbol`
- `impl AdaptIntoUnclamped for DocumentSymbol`
- `impl AdaptInto for DocumentSymbol`
- `impl IntoColor for DocumentSymbol`
- `impl IntoColorUnclamped for DocumentSymbol`
- `impl TryIntoColor for DocumentSymbol`
- `impl FromStimulus for DocumentSymbol`
- `impl IntoStimulus for DocumentSymbol`
- `impl Clone for DocumentSymbol`
- `impl Debug for DocumentSymbol`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

