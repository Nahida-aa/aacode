---
id: LanguageServerLogKey
title: LanguageServerLogKey
---

# Struct: LanguageServerLogKey

Defined in: [`packages/project/src/lsp_store/log_store.rs:223`](../../../../packages/project/src/lsp_store/log_store.rs#L223)

## Definition

```rust
pub struct LanguageServerLogKey {
    pub kind: LanguageServerKind,
    pub server_id: LanguageServerId,
}
```

## Implementations

### new()

```rust
pub fn new(kind: LanguageServerKind, server_id: LanguageServerId) -> Self
```

Defined in: [`packages/project/src/lsp_store/log_store.rs:229`](../../../../packages/project/src/lsp_store/log_store.rs#L229)

#### Parameters

##### kind

[`LanguageServerKind`](../enums/LanguageServerKind.md)

##### server_id

`LanguageServerId`

#### Returns

`Self`


***

### is_for_project()

```rust
pub fn is_for_project(&self, project: &WeakEntity<Project>, lsp_store: &WeakEntity<LspStore>) -> bool
```

Defined in: [`packages/project/src/lsp_store/log_store.rs:233`](../../../../packages/project/src/lsp_store/log_store.rs#L233)

#### Parameters

##### project

`&WeakEntity<Project>`

##### lsp_store

`&WeakEntity<LspStore>`

#### Returns

`bool`

## Trait Implementations

- `impl Borrow for LanguageServerLogKey`
- `impl BorrowMut for LanguageServerLogKey`
- `impl CloneToUninit for LanguageServerLogKey`
- `impl Into for LanguageServerLogKey`
- `impl From for LanguageServerLogKey`
- `impl TryInto for LanguageServerLogKey`
- `impl TryFrom for LanguageServerLogKey`
- `impl Any for LanguageServerLogKey`
- `impl ToOwned for LanguageServerLogKey`
- `impl Equivalent for LanguageServerLogKey`
- `impl DynClone for LanguageServerLogKey`
- `impl VZip for LanguageServerLogKey`
- `impl CastableFrom for LanguageServerLogKey`
- `impl CastableFrom for LanguageServerLogKey`
- `impl Read for LanguageServerLogKey`
- `impl IntoEither for LanguageServerLogKey`
- `impl ErasedDestructor for LanguageServerLogKey`
- `impl Same for LanguageServerLogKey`
- `impl Pointable for LanguageServerLogKey`
- `impl Instrument for LanguageServerLogKey`
- `impl WithSubscriber for LanguageServerLogKey`
- `impl FromAngle for LanguageServerLogKey`
- `impl IntoAngle for LanguageServerLogKey`
- `impl IntoCam16Unclamped for LanguageServerLogKey`
- `impl Cam16IntoUnclamped for LanguageServerLogKey`
- `impl ArraysFrom for LanguageServerLogKey`
- `impl ArraysInto for LanguageServerLogKey`
- `impl ComponentsFrom for LanguageServerLogKey`
- `impl TryComponentsInto for LanguageServerLogKey`
- `impl UintsFrom for LanguageServerLogKey`
- `impl UintsInto for LanguageServerLogKey`
- `impl AdaptIntoUnclamped for LanguageServerLogKey`
- `impl AdaptInto for LanguageServerLogKey`
- `impl IntoColor for LanguageServerLogKey`
- `impl IntoColorUnclamped for LanguageServerLogKey`
- `impl TryIntoColor for LanguageServerLogKey`
- `impl FromStimulus for LanguageServerLogKey`
- `impl IntoStimulus for LanguageServerLogKey`
- `impl Equivalent for LanguageServerLogKey`
- `impl Clone for LanguageServerLogKey`
- `impl Debug for LanguageServerLogKey`
- `impl StructuralPartialEq for LanguageServerLogKey`
- `impl PartialEq for LanguageServerLogKey`
- `impl Eq for LanguageServerLogKey`
- `impl Hash for LanguageServerLogKey`

## Auto Trait Implementations

`Freeze` `Send` `Sync` `Unpin` `UnsafeUnpin`

