---
id: Manager
title: Manager
---

# Struct: Manager

Defined in: [`packages/project/src/connection_manager.rs:17`](../../../../packages/project/src/connection_manager.rs#L17)

## Definition

```rust
pub struct Manager
```

_（存在非公开字段）_

## Implementations

### global()

```rust
pub fn global(cx: &App) -> Entity<Manager>
```

Defined in: [`packages/project/src/connection_manager.rs:33`](../../../../packages/project/src/connection_manager.rs#L33)

#### Parameters

##### cx

`&App`

#### Returns

`Entity<Manager>`


***

### maintain_project_connection()

```rust
pub fn maintain_project_connection(&self, project: &Entity<Project>, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/connection_manager.rs:37`](../../../../packages/project/src/connection_manager.rs#L37)

#### Parameters

##### project

`&Entity<Project>`

##### cx

`&Context<'_, Self>`

## Trait Implementations

- `impl Borrow for Manager`
- `impl BorrowMut for Manager`
- `impl Into for Manager`
- `impl From for Manager`
- `impl TryInto for Manager`
- `impl TryFrom for Manager`
- `impl Any for Manager`
- `impl VZip for Manager`
- `impl CastableFrom for Manager`
- `impl CastableFrom for Manager`
- `impl Read for Manager`
- `impl IntoEither for Manager`
- `impl ErasedDestructor for Manager`
- `impl Same for Manager`
- `impl Pointable for Manager`
- `impl Instrument for Manager`
- `impl WithSubscriber for Manager`
- `impl FromAngle for Manager`
- `impl IntoAngle for Manager`
- `impl IntoCam16Unclamped for Manager`
- `impl Cam16IntoUnclamped for Manager`
- `impl ArraysFrom for Manager`
- `impl ArraysInto for Manager`
- `impl ComponentsFrom for Manager`
- `impl TryComponentsInto for Manager`
- `impl UintsFrom for Manager`
- `impl UintsInto for Manager`
- `impl AdaptIntoUnclamped for Manager`
- `impl AdaptInto for Manager`
- `impl IntoColor for Manager`
- `impl IntoColorUnclamped for Manager`
- `impl TryIntoColor for Manager`
- `impl FromStimulus for Manager`
- `impl IntoStimulus for Manager`

## Auto Trait Implementations

`Freeze` `Send` `Sync` `Unpin` `UnsafeUnpin`

