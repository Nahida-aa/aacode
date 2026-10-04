---
id: DirectoryLister
title: DirectoryLister
---

# Enum: DirectoryLister

Defined in: [`packages/project/src/directory.rs:21`](../../../../packages/project/src/directory.rs#L21)

## Definition

```rust
pub enum DirectoryLister {
    Project(Entity<Project>),
    Local(Entity<Project>, Arc<dyn Fs>),
}
```

## Implementations

### is_local()

```rust
pub fn is_local(&self, cx: &App) -> bool
```

Defined in: [`packages/project/src/directory.rs:40`](../../../../packages/project/src/directory.rs#L40)

#### Parameters

##### cx

`&App`

#### Returns

`bool`


***

### resolve_tilde()

```rust
pub fn resolve_tilde<'a>(&self, path: &'a String, cx: &App) -> Cow<'a, str>
```

Defined in: [`packages/project/src/directory.rs:47`](../../../../packages/project/src/directory.rs#L47)

#### Parameters

##### path

`&'a String`

##### cx

`&App`

#### Returns

`Cow<'a, str>`


***

### default_query()

```rust
pub fn default_query(&self, cx: &App) -> String
```

Defined in: [`packages/project/src/directory.rs:55`](../../../../packages/project/src/directory.rs#L55)

#### Parameters

##### cx

`&App`

#### Returns

`String`


***

### list_directory()

```rust
pub fn list_directory(&self, path: String, cx: &App) -> Task<Result<Vec<DirectoryItem>>>
```

Defined in: [`packages/project/src/directory.rs:81`](../../../../packages/project/src/directory.rs#L81)

#### Parameters

##### path

`String`

##### cx

`&App`

#### Returns

`Task<Result<Vec<DirectoryItem>>>`


***

### path_style()

```rust
pub fn path_style(&self, cx: &App) -> PathStyle
```

Defined in: [`packages/project/src/directory.rs:108`](../../../../packages/project/src/directory.rs#L108)

#### Parameters

##### cx

`&App`

#### Returns

`PathStyle`

## Trait Implementations

- `impl Borrow for DirectoryLister`
- `impl BorrowMut for DirectoryLister`
- `impl CloneToUninit for DirectoryLister`
- `impl Into for DirectoryLister`
- `impl From for DirectoryLister`
- `impl TryInto for DirectoryLister`
- `impl TryFrom for DirectoryLister`
- `impl Any for DirectoryLister`
- `impl ToOwned for DirectoryLister`
- `impl DynClone for DirectoryLister`
- `impl VZip for DirectoryLister`
- `impl CastableFrom for DirectoryLister`
- `impl CastableFrom for DirectoryLister`
- `impl Read for DirectoryLister`
- `impl IntoEither for DirectoryLister`
- `impl ErasedDestructor for DirectoryLister`
- `impl Same for DirectoryLister`
- `impl Pointable for DirectoryLister`
- `impl Instrument for DirectoryLister`
- `impl WithSubscriber for DirectoryLister`
- `impl FromAngle for DirectoryLister`
- `impl IntoAngle for DirectoryLister`
- `impl IntoCam16Unclamped for DirectoryLister`
- `impl Cam16IntoUnclamped for DirectoryLister`
- `impl ArraysFrom for DirectoryLister`
- `impl ArraysInto for DirectoryLister`
- `impl ComponentsFrom for DirectoryLister`
- `impl TryComponentsInto for DirectoryLister`
- `impl UintsFrom for DirectoryLister`
- `impl UintsInto for DirectoryLister`
- `impl AdaptIntoUnclamped for DirectoryLister`
- `impl AdaptInto for DirectoryLister`
- `impl IntoColor for DirectoryLister`
- `impl IntoColorUnclamped for DirectoryLister`
- `impl TryIntoColor for DirectoryLister`
- `impl FromStimulus for DirectoryLister`
- `impl IntoStimulus for DirectoryLister`
- `impl Clone for DirectoryLister`
- `impl Debug for DirectoryLister`

## Auto Trait Implementations

`Freeze` `Send` `Sync` `Unpin` `UnsafeUnpin`

