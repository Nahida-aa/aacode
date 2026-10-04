---
id: PrettierStore
title: PrettierStore
---

# Struct: PrettierStore

Defined in: [`packages/project/src/prettier_store.rs:34`](../../../../packages/project/src/prettier_store.rs#L34)

## Definition

```rust
pub struct PrettierStore
```

_（存在非公开字段）_

## Implementations

### new()

```rust
pub fn new(node: NodeRuntime, fs: Arc<dyn Fs>, languages: Arc<LanguageRegistry>, worktree_store: Entity<WorktreeStore>, _: &Context<'_, Self>) -> Self
```

Defined in: [`packages/project/src/prettier_store.rs:57`](../../../../packages/project/src/prettier_store.rs#L57)

#### Parameters

##### node

`NodeRuntime`

##### fs

`Arc<dyn Fs>`

##### languages

`Arc<LanguageRegistry>`

##### worktree_store

`Entity<WorktreeStore>`

##### _

`&Context<'_, Self>`

#### Returns

`Self`


***

### remove_worktree()

```rust
pub fn remove_worktree(&self, id_to_remove: WorktreeId, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/prettier_store.rs:76`](../../../../packages/project/src/prettier_store.rs#L76)

#### Parameters

##### id_to_remove

`WorktreeId`

##### cx

`&Context<'_, Self>`


***

### update_prettier_settings()

```rust
pub fn update_prettier_settings(&self, worktree: &Entity<Worktree>, changes: &[(Arc<RelPath>, ProjectEntryId, PathChange)], cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/prettier_store.rs:455`](../../../../packages/project/src/prettier_store.rs#L455)

#### Parameters

##### worktree

`&Entity<Worktree>`

##### changes

`&[(Arc<RelPath>, ProjectEntryId, PathChange)]`

##### cx

`&Context<'_, Self>`


***

### install_default_prettier()

```rust
pub fn install_default_prettier<impl Iterator<Item = Arc<str>>: Iterator>(&self, worktree: Option<WorktreeId>, plugins: impl ?, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/prettier_store.rs:539`](../../../../packages/project/src/prettier_store.rs#L539)

#### Parameters

##### worktree

`Option<WorktreeId>`

##### plugins

`impl ?`

##### cx

`&Context<'_, Self>`

## Trait Implementations

- `impl Borrow for PrettierStore`
- `impl BorrowMut for PrettierStore`
- `impl Into for PrettierStore`
- `impl From for PrettierStore`
- `impl TryInto for PrettierStore`
- `impl TryFrom for PrettierStore`
- `impl Any for PrettierStore`
- `impl VZip for PrettierStore`
- `impl CastableFrom for PrettierStore`
- `impl CastableFrom for PrettierStore`
- `impl Read for PrettierStore`
- `impl IntoEither for PrettierStore`
- `impl ErasedDestructor for PrettierStore`
- `impl Same for PrettierStore`
- `impl Pointable for PrettierStore`
- `impl Instrument for PrettierStore`
- `impl WithSubscriber for PrettierStore`
- `impl FromAngle for PrettierStore`
- `impl IntoAngle for PrettierStore`
- `impl IntoCam16Unclamped for PrettierStore`
- `impl Cam16IntoUnclamped for PrettierStore`
- `impl ArraysFrom for PrettierStore`
- `impl ArraysInto for PrettierStore`
- `impl ComponentsFrom for PrettierStore`
- `impl TryComponentsInto for PrettierStore`
- `impl UintsFrom for PrettierStore`
- `impl UintsInto for PrettierStore`
- `impl AdaptIntoUnclamped for PrettierStore`
- `impl AdaptInto for PrettierStore`
- `impl IntoColor for PrettierStore`
- `impl IntoColorUnclamped for PrettierStore`
- `impl TryIntoColor for PrettierStore`
- `impl FromStimulus for PrettierStore`
- `impl IntoStimulus for PrettierStore`

## Auto Trait Implementations

`Freeze` `Send` `Sync` `Unpin` `UnsafeUnpin`

