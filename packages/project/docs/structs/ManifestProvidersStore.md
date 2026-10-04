---
id: ManifestProvidersStore
title: ManifestProvidersStore
---

# Struct: ManifestProvidersStore

Defined in: [`packages/project/src/manifest_tree/manifest_store.rs:14`](../../../../packages/project/src/manifest_tree/manifest_store.rs#L14)

## Definition

```rust
pub struct ManifestProvidersStore
```

## Implementations

### global()

```rust
pub fn global(cx: &App) -> Self
```

Defined in: [`packages/project/src/manifest_tree/manifest_store.rs:33`](../../../../packages/project/src/manifest_tree/manifest_store.rs#L33)

Returns the global [`ManifestStore`].

Inserts a default [`ManifestStore`] if one does not yet exist.

#### Parameters

##### cx

`&App`

#### Returns

`Self`


***

### register()

```rust
pub fn register(&self, provider: Arc<dyn ManifestProvider>)
```

Defined in: [`packages/project/src/manifest_tree/manifest_store.rs:37`](../../../../packages/project/src/manifest_tree/manifest_store.rs#L37)

#### Parameters

##### provider

`Arc<dyn ManifestProvider>`


***

### unregister()

```rust
pub fn unregister(&self, name: &SharedString)
```

Defined in: [`packages/project/src/manifest_tree/manifest_store.rs:41`](../../../../packages/project/src/manifest_tree/manifest_store.rs#L41)

#### Parameters

##### name

`&SharedString`

## Trait Implementations

- `impl Borrow for ManifestProvidersStore`
- `impl BorrowMut for ManifestProvidersStore`
- `impl CloneToUninit for ManifestProvidersStore`
- `impl Into for ManifestProvidersStore`
- `impl From for ManifestProvidersStore`
- `impl TryInto for ManifestProvidersStore`
- `impl TryFrom for ManifestProvidersStore`
- `impl Any for ManifestProvidersStore`
- `impl ToOwned for ManifestProvidersStore`
- `impl DynClone for ManifestProvidersStore`
- `impl VZip for ManifestProvidersStore`
- `impl CastableFrom for ManifestProvidersStore`
- `impl CastableFrom for ManifestProvidersStore`
- `impl Read for ManifestProvidersStore`
- `impl IntoEither for ManifestProvidersStore`
- `impl ErasedDestructor for ManifestProvidersStore`
- `impl Same for ManifestProvidersStore`
- `impl ReadPrimitive for ManifestProvidersStore`
- `impl Pointable for ManifestProvidersStore`
- `impl Instrument for ManifestProvidersStore`
- `impl WithSubscriber for ManifestProvidersStore`
- `impl FromAngle for ManifestProvidersStore`
- `impl IntoAngle for ManifestProvidersStore`
- `impl IntoCam16Unclamped for ManifestProvidersStore`
- `impl Cam16IntoUnclamped for ManifestProvidersStore`
- `impl ArraysFrom for ManifestProvidersStore`
- `impl ArraysInto for ManifestProvidersStore`
- `impl ComponentsFrom for ManifestProvidersStore`
- `impl TryComponentsInto for ManifestProvidersStore`
- `impl UintsFrom for ManifestProvidersStore`
- `impl UintsInto for ManifestProvidersStore`
- `impl AdaptIntoUnclamped for ManifestProvidersStore`
- `impl AdaptInto for ManifestProvidersStore`
- `impl IntoColor for ManifestProvidersStore`
- `impl IntoColorUnclamped for ManifestProvidersStore`
- `impl TryIntoColor for ManifestProvidersStore`
- `impl FromStimulus for ManifestProvidersStore`
- `impl IntoStimulus for ManifestProvidersStore`
- `impl Clone for ManifestProvidersStore`
- `impl Default for ManifestProvidersStore`

## Auto Trait Implementations

`Freeze` `Unpin` `UnsafeUnpin`

