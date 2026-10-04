---
id: DiagnosticSummary
title: DiagnosticSummary
---

# Struct: DiagnosticSummary

Defined in: [`packages/project/src/lsp_store/mod.rs:16176`](../../../../packages/project/src/lsp_store/mod.rs#L16176)

## Definition

```rust
pub struct DiagnosticSummary {
    pub error_count: usize,
    pub warning_count: usize,
}
```

## Implementations

### new()

```rust
pub fn new<'a, T: 'a, impl IntoIterator<Item = &'a DiagnosticEntry<T>>: IntoIterator>(diagnostics: impl ?) -> Self
```

Defined in: [`packages/project/src/lsp_store/mod.rs:16182`](../../../../packages/project/src/lsp_store/mod.rs#L16182)

#### Parameters

##### diagnostics

`impl ?`

#### Returns

`Self`


***

### is_empty()

```rust
pub fn is_empty(&self) -> bool
```

Defined in: [`packages/project/src/lsp_store/mod.rs:16201`](../../../../packages/project/src/lsp_store/mod.rs#L16201)

#### Returns

`bool`


***

### to_proto()

```rust
pub fn to_proto(self, language_server_id: LanguageServerId, path: &RelPath) -> DiagnosticSummary
```

Defined in: [`packages/project/src/lsp_store/mod.rs:16205`](../../../../packages/project/src/lsp_store/mod.rs#L16205)

#### Parameters

##### language_server_id

`LanguageServerId`

##### path

`&RelPath`

#### Returns

`DiagnosticSummary`

## Trait Implementations

- `impl Borrow for DiagnosticSummary`
- `impl BorrowMut for DiagnosticSummary`
- `impl CloneToUninit for DiagnosticSummary`
- `impl Into for DiagnosticSummary`
- `impl From for DiagnosticSummary`
- `impl TryInto for DiagnosticSummary`
- `impl TryFrom for DiagnosticSummary`
- `impl Any for DiagnosticSummary`
- `impl ToOwned for DiagnosticSummary`
- `impl Serialize for DiagnosticSummary`
- `impl DynClone for DiagnosticSummary`
- `impl VZip for DiagnosticSummary`
- `impl CastableFrom for DiagnosticSummary`
- `impl CastableFrom for DiagnosticSummary`
- `impl Read for DiagnosticSummary`
- `impl IntoEither for DiagnosticSummary`
- `impl ErasedDestructor for DiagnosticSummary`
- `impl Same for DiagnosticSummary`
- `impl ReadPrimitive for DiagnosticSummary`
- `impl Pointable for DiagnosticSummary`
- `impl Instrument for DiagnosticSummary`
- `impl WithSubscriber for DiagnosticSummary`
- `impl FromAngle for DiagnosticSummary`
- `impl IntoAngle for DiagnosticSummary`
- `impl IntoCam16Unclamped for DiagnosticSummary`
- `impl Cam16IntoUnclamped for DiagnosticSummary`
- `impl ArraysFrom for DiagnosticSummary`
- `impl ArraysInto for DiagnosticSummary`
- `impl ComponentsFrom for DiagnosticSummary`
- `impl TryComponentsInto for DiagnosticSummary`
- `impl UintsFrom for DiagnosticSummary`
- `impl UintsInto for DiagnosticSummary`
- `impl AdaptIntoUnclamped for DiagnosticSummary`
- `impl AdaptInto for DiagnosticSummary`
- `impl IntoColor for DiagnosticSummary`
- `impl IntoColorUnclamped for DiagnosticSummary`
- `impl TryIntoColor for DiagnosticSummary`
- `impl FromStimulus for DiagnosticSummary`
- `impl IntoStimulus for DiagnosticSummary`
- `impl ResetDiscriminant for DiagnosticSummary`
- `impl Copy for DiagnosticSummary`
- `impl Clone for DiagnosticSummary`
- `impl Debug for DiagnosticSummary`
- `impl Default for DiagnosticSummary`
- `impl StructuralPartialEq for DiagnosticSummary`
- `impl PartialEq for DiagnosticSummary`
- `impl Serialize for DiagnosticSummary`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

