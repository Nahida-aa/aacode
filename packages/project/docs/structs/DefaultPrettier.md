---
id: DefaultPrettier
title: DefaultPrettier
---

# Struct: DefaultPrettier

Defined in: [`packages/project/src/prettier_store.rs:790`](../../../../packages/project/src/prettier_store.rs#L790)

## Definition

```rust
pub struct DefaultPrettier
```

_（存在非公开字段）_

## Implementations

### instance()

```rust
pub fn instance(&self) -> Option<&PrettierInstance>
```

Defined in: [`packages/project/src/prettier_store.rs:827`](../../../../packages/project/src/prettier_store.rs#L827)

#### Returns

`Option<&PrettierInstance>`


***

### prettier_task()

```rust
pub fn prettier_task(&self, node: &NodeRuntime, worktree_id: Option<WorktreeId>, cx: &Context<'_, PrettierStore>) -> Option<Task<Result<PrettierTask>>>
```

Defined in: [`packages/project/src/prettier_store.rs:835`](../../../../packages/project/src/prettier_store.rs#L835)

#### Parameters

##### node

`&NodeRuntime`

##### worktree_id

`Option<WorktreeId>`

##### cx

`&Context<'_, PrettierStore>`

#### Returns

`Option<Task<Result<PrettierTask>>>`

## Trait Implementations

- `impl Borrow for DefaultPrettier`
- `impl BorrowMut for DefaultPrettier`
- `impl Into for DefaultPrettier`
- `impl From for DefaultPrettier`
- `impl TryInto for DefaultPrettier`
- `impl TryFrom for DefaultPrettier`
- `impl Any for DefaultPrettier`
- `impl VZip for DefaultPrettier`
- `impl CastableFrom for DefaultPrettier`
- `impl CastableFrom for DefaultPrettier`
- `impl Read for DefaultPrettier`
- `impl IntoEither for DefaultPrettier`
- `impl ErasedDestructor for DefaultPrettier`
- `impl Same for DefaultPrettier`
- `impl ReadPrimitive for DefaultPrettier`
- `impl Pointable for DefaultPrettier`
- `impl Instrument for DefaultPrettier`
- `impl WithSubscriber for DefaultPrettier`
- `impl FromAngle for DefaultPrettier`
- `impl IntoAngle for DefaultPrettier`
- `impl IntoCam16Unclamped for DefaultPrettier`
- `impl Cam16IntoUnclamped for DefaultPrettier`
- `impl ArraysFrom for DefaultPrettier`
- `impl ArraysInto for DefaultPrettier`
- `impl ComponentsFrom for DefaultPrettier`
- `impl TryComponentsInto for DefaultPrettier`
- `impl UintsFrom for DefaultPrettier`
- `impl UintsInto for DefaultPrettier`
- `impl AdaptIntoUnclamped for DefaultPrettier`
- `impl AdaptInto for DefaultPrettier`
- `impl IntoColor for DefaultPrettier`
- `impl IntoColorUnclamped for DefaultPrettier`
- `impl TryIntoColor for DefaultPrettier`
- `impl FromStimulus for DefaultPrettier`
- `impl IntoStimulus for DefaultPrettier`
- `impl Debug for DefaultPrettier`
- `impl Default for DefaultPrettier`

## Auto Trait Implementations

`Freeze` `Send` `Sync` `Unpin` `UnsafeUnpin`

