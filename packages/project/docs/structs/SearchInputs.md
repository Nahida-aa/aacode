---
id: SearchInputs
title: SearchInputs
---

# Struct: SearchInputs

Defined in: [`packages/project/src/search/mod.rs:41`](../../../../packages/project/src/search/mod.rs#L41)

## Definition

```rust
pub struct SearchInputs
```

_（存在非公开字段）_

## Implementations

### as_str()

```rust
pub fn as_str(&self) -> &str
```

Defined in: [`packages/project/src/search/mod.rs:62`](../../../../packages/project/src/search/mod.rs#L62)

#### Returns

`&str`


***

### files_to_include()

```rust
pub fn files_to_include(&self) -> &PathMatcher
```

Defined in: [`packages/project/src/search/mod.rs:65`](../../../../packages/project/src/search/mod.rs#L65)

#### Returns

`&PathMatcher`


***

### files_to_exclude()

```rust
pub fn files_to_exclude(&self) -> &PathMatcher
```

Defined in: [`packages/project/src/search/mod.rs:68`](../../../../packages/project/src/search/mod.rs#L68)

#### Returns

`&PathMatcher`


***

### buffers()

```rust
pub fn buffers(&self) -> &Option<Vec<Entity<Buffer>>>
```

Defined in: [`packages/project/src/search/mod.rs:71`](../../../../packages/project/src/search/mod.rs#L71)

#### Returns

`&Option<Vec<Entity<Buffer>>>`

## Trait Implementations

- `impl Borrow for SearchInputs`
- `impl BorrowMut for SearchInputs`
- `impl CloneToUninit for SearchInputs`
- `impl Into for SearchInputs`
- `impl From for SearchInputs`
- `impl TryInto for SearchInputs`
- `impl TryFrom for SearchInputs`
- `impl Any for SearchInputs`
- `impl ToOwned for SearchInputs`
- `impl DynClone for SearchInputs`
- `impl VZip for SearchInputs`
- `impl CastableFrom for SearchInputs`
- `impl CastableFrom for SearchInputs`
- `impl Read for SearchInputs`
- `impl IntoEither for SearchInputs`
- `impl ErasedDestructor for SearchInputs`
- `impl Same for SearchInputs`
- `impl Pointable for SearchInputs`
- `impl Instrument for SearchInputs`
- `impl WithSubscriber for SearchInputs`
- `impl FromAngle for SearchInputs`
- `impl IntoAngle for SearchInputs`
- `impl IntoCam16Unclamped for SearchInputs`
- `impl Cam16IntoUnclamped for SearchInputs`
- `impl ArraysFrom for SearchInputs`
- `impl ArraysInto for SearchInputs`
- `impl ComponentsFrom for SearchInputs`
- `impl TryComponentsInto for SearchInputs`
- `impl UintsFrom for SearchInputs`
- `impl UintsInto for SearchInputs`
- `impl AdaptIntoUnclamped for SearchInputs`
- `impl AdaptInto for SearchInputs`
- `impl IntoColor for SearchInputs`
- `impl IntoColorUnclamped for SearchInputs`
- `impl TryIntoColor for SearchInputs`
- `impl FromStimulus for SearchInputs`
- `impl IntoStimulus for SearchInputs`
- `impl Clone for SearchInputs`
- `impl Debug for SearchInputs`

## Auto Trait Implementations

`Freeze` `Send` `Sync` `Unpin` `UnsafeUnpin`

