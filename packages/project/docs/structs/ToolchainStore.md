---
id: ToolchainStore
title: ToolchainStore
---

# Struct: ToolchainStore

Defined in: [`packages/project/src/toolchain_store.rs:32`](../../../../packages/project/src/toolchain_store.rs#L32)

## Definition

```rust
pub struct ToolchainStore
```

_（存在非公开字段）_

## Implementations

### init()

```rust
pub fn init(client: &AnyProtoClient)
```

Defined in: [`packages/project/src/toolchain_store.rs:53`](../../../../packages/project/src/toolchain_store.rs#L53)

#### Parameters

##### client

`&AnyProtoClient`


***

### local()

```rust
pub fn local(languages: Arc<LanguageRegistry>, worktree_store: Entity<WorktreeStore>, project_environment: Entity<ProjectEnvironment>, manifest_tree: Entity<ManifestTree>, cx: &Context<'_, Self>) -> Self
```

Defined in: [`packages/project/src/toolchain_store.rs:60`](../../../../packages/project/src/toolchain_store.rs#L60)

#### Parameters

##### languages

`Arc<LanguageRegistry>`

##### worktree_store

`Entity<WorktreeStore>`

##### project_environment

`Entity<ProjectEnvironment>`

##### manifest_tree

`Entity<ManifestTree>`

##### cx

`&Context<'_, Self>`

#### Returns

`Self`


***

### as_language_toolchain_store()

```rust
pub fn as_language_toolchain_store(&self) -> Arc<dyn LanguageToolchainStore>
```

Defined in: [`packages/project/src/toolchain_store.rs:400`](../../../../packages/project/src/toolchain_store.rs#L400)

#### Returns

`Arc<dyn LanguageToolchainStore>`


***

### as_local_store()

```rust
pub fn as_local_store(&self) -> Option<&Entity<LocalToolchainStore>>
```

Defined in: [`packages/project/src/toolchain_store.rs:406`](../../../../packages/project/src/toolchain_store.rs#L406)

#### Returns

`Option<&Entity<LocalToolchainStore>>`

## Trait Implementations

- `impl Borrow for ToolchainStore`
- `impl BorrowMut for ToolchainStore`
- `impl Into for ToolchainStore`
- `impl From for ToolchainStore`
- `impl TryInto for ToolchainStore`
- `impl TryFrom for ToolchainStore`
- `impl Any for ToolchainStore`
- `impl VZip for ToolchainStore`
- `impl CastableFrom for ToolchainStore`
- `impl CastableFrom for ToolchainStore`
- `impl Read for ToolchainStore`
- `impl IntoEither for ToolchainStore`
- `impl ErasedDestructor for ToolchainStore`
- `impl Same for ToolchainStore`
- `impl Pointable for ToolchainStore`
- `impl Instrument for ToolchainStore`
- `impl WithSubscriber for ToolchainStore`
- `impl FromAngle for ToolchainStore`
- `impl IntoAngle for ToolchainStore`
- `impl IntoCam16Unclamped for ToolchainStore`
- `impl Cam16IntoUnclamped for ToolchainStore`
- `impl ArraysFrom for ToolchainStore`
- `impl ArraysInto for ToolchainStore`
- `impl ComponentsFrom for ToolchainStore`
- `impl TryComponentsInto for ToolchainStore`
- `impl UintsFrom for ToolchainStore`
- `impl UintsInto for ToolchainStore`
- `impl AdaptIntoUnclamped for ToolchainStore`
- `impl AdaptInto for ToolchainStore`
- `impl IntoColor for ToolchainStore`
- `impl IntoColorUnclamped for ToolchainStore`
- `impl TryIntoColor for ToolchainStore`
- `impl FromStimulus for ToolchainStore`
- `impl IntoStimulus for ToolchainStore`
- `impl EventEmitter for ToolchainStore`

## Auto Trait Implementations

`Freeze` `Unpin` `UnsafeUnpin`

