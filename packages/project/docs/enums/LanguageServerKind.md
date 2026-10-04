---
id: LanguageServerKind
title: LanguageServerKind
---

# Enum: LanguageServerKind

Defined in: [`packages/project/src/lsp_store/log_store.rs:202`](../../../../packages/project/src/lsp_store/log_store.rs#L202)

## Definition

```rust
pub enum LanguageServerKind {
    Local{ .. },
    Remote{ .. },
    LocalSsh{ .. },
    Supplementary{ .. },
}
```

## Implementations

### project()

```rust
pub fn project(&self) -> Option<&WeakEntity<Project>>
```

Defined in: [`packages/project/src/lsp_store/log_store.rs:243`](../../../../packages/project/src/lsp_store/log_store.rs#L243)

#### Returns

`Option<&WeakEntity<Project>>`


***

### is_for_project()

```rust
pub fn is_for_project(&self, project: &WeakEntity<Project>, lsp_store: &WeakEntity<LspStore>) -> bool
```

Defined in: [`packages/project/src/lsp_store/log_store.rs:252`](../../../../packages/project/src/lsp_store/log_store.rs#L252)

#### Parameters

##### project

`&WeakEntity<Project>`

##### lsp_store

`&WeakEntity<LspStore>`

#### Returns

`bool`

## Trait Implementations

- `impl Borrow for LanguageServerKind`
- `impl BorrowMut for LanguageServerKind`
- `impl CloneToUninit for LanguageServerKind`
- `impl Into for LanguageServerKind`
- `impl From for LanguageServerKind`
- `impl TryInto for LanguageServerKind`
- `impl TryFrom for LanguageServerKind`
- `impl Any for LanguageServerKind`
- `impl ToOwned for LanguageServerKind`
- `impl Equivalent for LanguageServerKind`
- `impl DynClone for LanguageServerKind`
- `impl VZip for LanguageServerKind`
- `impl CastableFrom for LanguageServerKind`
- `impl CastableFrom for LanguageServerKind`
- `impl Read for LanguageServerKind`
- `impl IntoEither for LanguageServerKind`
- `impl ErasedDestructor for LanguageServerKind`
- `impl Same for LanguageServerKind`
- `impl Pointable for LanguageServerKind`
- `impl Instrument for LanguageServerKind`
- `impl WithSubscriber for LanguageServerKind`
- `impl FromAngle for LanguageServerKind`
- `impl IntoAngle for LanguageServerKind`
- `impl IntoCam16Unclamped for LanguageServerKind`
- `impl Cam16IntoUnclamped for LanguageServerKind`
- `impl ArraysFrom for LanguageServerKind`
- `impl ArraysInto for LanguageServerKind`
- `impl ComponentsFrom for LanguageServerKind`
- `impl TryComponentsInto for LanguageServerKind`
- `impl UintsFrom for LanguageServerKind`
- `impl UintsInto for LanguageServerKind`
- `impl AdaptIntoUnclamped for LanguageServerKind`
- `impl AdaptInto for LanguageServerKind`
- `impl IntoColor for LanguageServerKind`
- `impl IntoColorUnclamped for LanguageServerKind`
- `impl TryIntoColor for LanguageServerKind`
- `impl FromStimulus for LanguageServerKind`
- `impl IntoStimulus for LanguageServerKind`
- `impl Equivalent for LanguageServerKind`
- `impl StructuralPartialEq for LanguageServerKind`
- `impl PartialEq for LanguageServerKind`
- `impl Eq for LanguageServerKind`
- `impl Hash for LanguageServerKind`
- `impl Clone for LanguageServerKind`
- `impl Debug for LanguageServerKind`

## Auto Trait Implementations

`Freeze` `Send` `Sync` `Unpin` `UnsafeUnpin`

