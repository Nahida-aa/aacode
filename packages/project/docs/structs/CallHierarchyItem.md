---
id: CallHierarchyItem
title: CallHierarchyItem
---

# Struct: CallHierarchyItem

Defined in: [`packages/project/src/lsp_command/mod.rs:334`](../../../../packages/project/src/lsp_command/mod.rs#L334)

## Definition

```rust
pub struct CallHierarchyItem
{
    pub buffer: Entity<Buffer>,
    pub server_id: LanguageServerId,
    pub name: String,
    pub kind: SymbolKind,
    pub detail: Option<String>,
    pub range: Range<Anchor>,
    pub selection_range: Range<Anchor>,
    pub data: Option<Value>,
}
```

## Trait Implementations

- `impl Borrow for CallHierarchyItem`
- `impl BorrowMut for CallHierarchyItem`
- `impl CloneToUninit for CallHierarchyItem`
- `impl Into for CallHierarchyItem`
- `impl From for CallHierarchyItem`
- `impl TryInto for CallHierarchyItem`
- `impl TryFrom for CallHierarchyItem`
- `impl Any for CallHierarchyItem`
- `impl ToOwned for CallHierarchyItem`
- `impl DynClone for CallHierarchyItem`
- `impl VZip for CallHierarchyItem`
- `impl CastableFrom for CallHierarchyItem`
- `impl CastableFrom for CallHierarchyItem`
- `impl Read for CallHierarchyItem`
- `impl IntoEither for CallHierarchyItem`
- `impl ErasedDestructor for CallHierarchyItem`
- `impl Same for CallHierarchyItem`
- `impl Pointable for CallHierarchyItem`
- `impl Instrument for CallHierarchyItem`
- `impl WithSubscriber for CallHierarchyItem`
- `impl FromAngle for CallHierarchyItem`
- `impl IntoAngle for CallHierarchyItem`
- `impl IntoCam16Unclamped for CallHierarchyItem`
- `impl Cam16IntoUnclamped for CallHierarchyItem`
- `impl ArraysFrom for CallHierarchyItem`
- `impl ArraysInto for CallHierarchyItem`
- `impl ComponentsFrom for CallHierarchyItem`
- `impl TryComponentsInto for CallHierarchyItem`
- `impl UintsFrom for CallHierarchyItem`
- `impl UintsInto for CallHierarchyItem`
- `impl AdaptIntoUnclamped for CallHierarchyItem`
- `impl AdaptInto for CallHierarchyItem`
- `impl IntoColor for CallHierarchyItem`
- `impl IntoColorUnclamped for CallHierarchyItem`
- `impl TryIntoColor for CallHierarchyItem`
- `impl FromStimulus for CallHierarchyItem`
- `impl IntoStimulus for CallHierarchyItem`
- `impl Debug for CallHierarchyItem`
- `impl Clone for CallHierarchyItem`

## Auto Trait Implementations

`Freeze` `Send` `Sync` `Unpin` `UnsafeUnpin`

