---
id: ResolvedPath
title: ResolvedPath
---

# Enum: ResolvedPath

Defined in: [`packages/project/src/path.rs:53`](../../../../packages/project/src/path.rs#L53)

ResolvedPath is a path that has been resolved to either a ProjectPath
or an AbsPath and that *exists*.

## Definition

```rust
pub enum ResolvedPath
{
    ProjectPath{ .. },
    AbsPath{ .. },
}
```

## Implementations

### abs_path()

```rust
pub fn abs_path(&self) -> Option<&str>
```

Defined in: [`packages/project/src/path.rs:64`](../../../../packages/project/src/path.rs#L64)

#### Returns

`Option<&str>`


***

### into_abs_path()

```rust
pub fn into_abs_path(self) -> Option<String>
```

Defined in: [`packages/project/src/path.rs:71`](../../../../packages/project/src/path.rs#L71)

#### Returns

`Option<String>`


***

### project_path()

```rust
pub fn project_path(&self) -> Option<&ProjectPath>
```

Defined in: [`packages/project/src/path.rs:78`](../../../../packages/project/src/path.rs#L78)

#### Returns

`Option<&ProjectPath>`


***

### is_file()

```rust
pub fn is_file(&self) -> bool
```

Defined in: [`packages/project/src/path.rs:85`](../../../../packages/project/src/path.rs#L85)

#### Returns

`bool`


***

### is_dir()

```rust
pub fn is_dir(&self) -> bool
```

Defined in: [`packages/project/src/path.rs:87`](../../../../packages/project/src/path.rs#L87)

#### Returns

`bool`

## Trait Implementations

- `impl Borrow for ResolvedPath`
- `impl BorrowMut for ResolvedPath`
- `impl CloneToUninit for ResolvedPath`
- `impl Into for ResolvedPath`
- `impl From for ResolvedPath`
- `impl TryInto for ResolvedPath`
- `impl TryFrom for ResolvedPath`
- `impl Any for ResolvedPath`
- `impl ToOwned for ResolvedPath`
- `impl DynClone for ResolvedPath`
- `impl VZip for ResolvedPath`
- `impl CastableFrom for ResolvedPath`
- `impl CastableFrom for ResolvedPath`
- `impl Read for ResolvedPath`
- `impl IntoEither for ResolvedPath`
- `impl ErasedDestructor for ResolvedPath`
- `impl Same for ResolvedPath`
- `impl Pointable for ResolvedPath`
- `impl Instrument for ResolvedPath`
- `impl WithSubscriber for ResolvedPath`
- `impl FromAngle for ResolvedPath`
- `impl IntoAngle for ResolvedPath`
- `impl IntoCam16Unclamped for ResolvedPath`
- `impl Cam16IntoUnclamped for ResolvedPath`
- `impl ArraysFrom for ResolvedPath`
- `impl ArraysInto for ResolvedPath`
- `impl ComponentsFrom for ResolvedPath`
- `impl TryComponentsInto for ResolvedPath`
- `impl UintsFrom for ResolvedPath`
- `impl UintsInto for ResolvedPath`
- `impl AdaptIntoUnclamped for ResolvedPath`
- `impl AdaptInto for ResolvedPath`
- `impl IntoColor for ResolvedPath`
- `impl IntoColorUnclamped for ResolvedPath`
- `impl TryIntoColor for ResolvedPath`
- `impl FromStimulus for ResolvedPath`
- `impl IntoStimulus for ResolvedPath`
- `impl Debug for ResolvedPath`
- `impl Clone for ResolvedPath`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

