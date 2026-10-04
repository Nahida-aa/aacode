---
id: RepositoryId
title: RepositoryId
---

# Struct: RepositoryId

Defined in: [`packages/project/src/git_store/mod.rs:585`](../../../../packages/project/src/git_store/mod.rs#L585)

## Definition

```rust
pub struct RepositoryId
```

## Implementations

### to_proto()

```rust
pub fn to_proto(self) -> u64
```

Defined in: [`packages/project/src/git_store/mod.rs:6090`](../../../../packages/project/src/git_store/mod.rs#L6090)

#### Returns

`u64`


***

### from_proto()

```rust
pub fn from_proto(id: u64) -> Self
```

Defined in: [`packages/project/src/git_store/mod.rs:6094`](../../../../packages/project/src/git_store/mod.rs#L6094)

#### Parameters

##### id

`u64`

#### Returns

`Self`

## Trait Implementations

- `impl Borrow for RepositoryId`
- `impl BorrowMut for RepositoryId`
- `impl CloneToUninit for RepositoryId`
- `impl Into for RepositoryId`
- `impl From for RepositoryId`
- `impl TryInto for RepositoryId`
- `impl TryFrom for RepositoryId`
- `impl Any for RepositoryId`
- `impl ToOwned for RepositoryId`
- `impl Equivalent for RepositoryId`
- `impl Comparable for RepositoryId`
- `impl DynClone for RepositoryId`
- `impl VZip for RepositoryId`
- `impl CastableFrom for RepositoryId`
- `impl CastableFrom for RepositoryId`
- `impl Read for RepositoryId`
- `impl IntoEither for RepositoryId`
- `impl ErasedDestructor for RepositoryId`
- `impl Same for RepositoryId`
- `impl Pointable for RepositoryId`
- `impl MapSeekTarget for RepositoryId`
- `impl Instrument for RepositoryId`
- `impl WithSubscriber for RepositoryId`
- `impl FromAngle for RepositoryId`
- `impl IntoAngle for RepositoryId`
- `impl IntoCam16Unclamped for RepositoryId`
- `impl Cam16IntoUnclamped for RepositoryId`
- `impl ArraysFrom for RepositoryId`
- `impl ArraysInto for RepositoryId`
- `impl ComponentsFrom for RepositoryId`
- `impl TryComponentsInto for RepositoryId`
- `impl UintsFrom for RepositoryId`
- `impl UintsInto for RepositoryId`
- `impl AdaptIntoUnclamped for RepositoryId`
- `impl AdaptInto for RepositoryId`
- `impl IntoColor for RepositoryId`
- `impl IntoColorUnclamped for RepositoryId`
- `impl TryIntoColor for RepositoryId`
- `impl FromStimulus for RepositoryId`
- `impl IntoStimulus for RepositoryId`
- `impl Equivalent for RepositoryId`
- `impl ResetDiscriminant for RepositoryId`
- `impl Clone for RepositoryId`
- `impl Copy for RepositoryId`
- `impl Debug for RepositoryId`
- `impl StructuralPartialEq for RepositoryId`
- `impl PartialEq for RepositoryId`
- `impl Eq for RepositoryId`
- `impl PartialOrd for RepositoryId`
- `impl Ord for RepositoryId`
- `impl Hash for RepositoryId`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

