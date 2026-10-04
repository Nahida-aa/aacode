---
id: PrettierInstance
title: PrettierInstance
---

# Struct: PrettierInstance

Defined in: [`packages/project/src/prettier_store.rs:808`](../../../../packages/project/src/prettier_store.rs#L808)

## Definition

```rust
pub struct PrettierInstance
```

_（存在非公开字段）_

## Implementations

### prettier_task()

```rust
pub fn prettier_task(&self, node: &NodeRuntime, prettier_dir: Option<&Path>, worktree_id: Option<WorktreeId>, cx: &Context<'_, PrettierStore>) -> Option<Task<Result<PrettierTask>>>
```

Defined in: [`packages/project/src/prettier_store.rs:853`](../../../../packages/project/src/prettier_store.rs#L853)

#### Parameters

##### node

`&NodeRuntime`

##### prettier_dir

`Option<&Path>`

##### worktree_id

`Option<WorktreeId>`

##### cx

`&Context<'_, PrettierStore>`

#### Returns

`Option<Task<Result<PrettierTask>>>`


***

### server()

```rust
pub async fn server(&self) -> Option<Arc<LanguageServer>>
```

Defined in: [`packages/project/src/prettier_store.rs:898`](../../../../packages/project/src/prettier_store.rs#L898)

#### Returns

`Option<Arc<LanguageServer>>`

## Trait Implementations

- `impl Borrow for PrettierInstance`
- `impl BorrowMut for PrettierInstance`
- `impl CloneToUninit for PrettierInstance`
- `impl Into for PrettierInstance`
- `impl From for PrettierInstance`
- `impl TryInto for PrettierInstance`
- `impl TryFrom for PrettierInstance`
- `impl Any for PrettierInstance`
- `impl ToOwned for PrettierInstance`
- `impl DynClone for PrettierInstance`
- `impl VZip for PrettierInstance`
- `impl CastableFrom for PrettierInstance`
- `impl CastableFrom for PrettierInstance`
- `impl Read for PrettierInstance`
- `impl IntoEither for PrettierInstance`
- `impl ErasedDestructor for PrettierInstance`
- `impl Same for PrettierInstance`
- `impl Pointable for PrettierInstance`
- `impl Instrument for PrettierInstance`
- `impl WithSubscriber for PrettierInstance`
- `impl FromAngle for PrettierInstance`
- `impl IntoAngle for PrettierInstance`
- `impl IntoCam16Unclamped for PrettierInstance`
- `impl Cam16IntoUnclamped for PrettierInstance`
- `impl ArraysFrom for PrettierInstance`
- `impl ArraysInto for PrettierInstance`
- `impl ComponentsFrom for PrettierInstance`
- `impl TryComponentsInto for PrettierInstance`
- `impl UintsFrom for PrettierInstance`
- `impl UintsInto for PrettierInstance`
- `impl AdaptIntoUnclamped for PrettierInstance`
- `impl AdaptInto for PrettierInstance`
- `impl IntoColor for PrettierInstance`
- `impl IntoColorUnclamped for PrettierInstance`
- `impl TryIntoColor for PrettierInstance`
- `impl FromStimulus for PrettierInstance`
- `impl IntoStimulus for PrettierInstance`
- `impl Debug for PrettierInstance`
- `impl Clone for PrettierInstance`

## Auto Trait Implementations

`Freeze` `Send` `Sync` `Unpin` `UnsafeUnpin`

