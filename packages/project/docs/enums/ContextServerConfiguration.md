---
id: ContextServerConfiguration
title: ContextServerConfiguration
---

# Enum: ContextServerConfiguration

Defined in: [`packages/project/src/context_server_store/mod.rs:158`](../../../../packages/project/src/context_server_store/mod.rs#L158)

## Definition

```rust
pub enum ContextServerConfiguration {
    Custom{ .. },
    Extension{ .. },
    Http{ .. },
}
```

## Implementations

### command()

```rust
pub fn command(&self) -> Option<&ContextServerCommand>
```

Defined in: [`packages/project/src/context_server_store/mod.rs:177`](../../../../packages/project/src/context_server_store/mod.rs#L177)

#### Returns

`Option<&ContextServerCommand>`


***

### has_static_auth_header()

```rust
pub fn has_static_auth_header(&self) -> bool
```

Defined in: [`packages/project/src/context_server_store/mod.rs:185`](../../../../packages/project/src/context_server_store/mod.rs#L185)

#### Returns

`bool`


***

### remote()

```rust
pub fn remote(&self) -> bool
```

Defined in: [`packages/project/src/context_server_store/mod.rs:194`](../../../../packages/project/src/context_server_store/mod.rs#L194)

#### Returns

`bool`


***

### from_settings()

```rust
pub async fn from_settings(settings: ContextServerSettings, id: ContextServerId, registry: Entity<ContextServerDescriptorRegistry>, worktree_store: Entity<WorktreeStore>, cx: &AsyncApp) -> Option<Self>
```

Defined in: [`packages/project/src/context_server_store/mod.rs:202`](../../../../packages/project/src/context_server_store/mod.rs#L202)

#### Parameters

##### settings

[`ContextServerSettings`](ContextServerSettings.md)

##### id

`ContextServerId`

##### registry

`Entity<ContextServerDescriptorRegistry>`

##### worktree_store

`Entity<WorktreeStore>`

##### cx

`&AsyncApp`

#### Returns

`Option<Self>`

## Trait Implementations

- `impl Borrow for ContextServerConfiguration`
- `impl BorrowMut for ContextServerConfiguration`
- `impl Into for ContextServerConfiguration`
- `impl From for ContextServerConfiguration`
- `impl TryInto for ContextServerConfiguration`
- `impl TryFrom for ContextServerConfiguration`
- `impl Any for ContextServerConfiguration`
- `impl Equivalent for ContextServerConfiguration`
- `impl VZip for ContextServerConfiguration`
- `impl CastableFrom for ContextServerConfiguration`
- `impl CastableFrom for ContextServerConfiguration`
- `impl Read for ContextServerConfiguration`
- `impl IntoEither for ContextServerConfiguration`
- `impl ErasedDestructor for ContextServerConfiguration`
- `impl Same for ContextServerConfiguration`
- `impl Pointable for ContextServerConfiguration`
- `impl Instrument for ContextServerConfiguration`
- `impl WithSubscriber for ContextServerConfiguration`
- `impl FromAngle for ContextServerConfiguration`
- `impl IntoAngle for ContextServerConfiguration`
- `impl IntoCam16Unclamped for ContextServerConfiguration`
- `impl Cam16IntoUnclamped for ContextServerConfiguration`
- `impl ArraysFrom for ContextServerConfiguration`
- `impl ArraysInto for ContextServerConfiguration`
- `impl ComponentsFrom for ContextServerConfiguration`
- `impl TryComponentsInto for ContextServerConfiguration`
- `impl UintsFrom for ContextServerConfiguration`
- `impl UintsInto for ContextServerConfiguration`
- `impl AdaptIntoUnclamped for ContextServerConfiguration`
- `impl AdaptInto for ContextServerConfiguration`
- `impl IntoColor for ContextServerConfiguration`
- `impl IntoColorUnclamped for ContextServerConfiguration`
- `impl TryIntoColor for ContextServerConfiguration`
- `impl FromStimulus for ContextServerConfiguration`
- `impl IntoStimulus for ContextServerConfiguration`
- `impl Equivalent for ContextServerConfiguration`
- `impl Debug for ContextServerConfiguration`
- `impl StructuralPartialEq for ContextServerConfiguration`
- `impl PartialEq for ContextServerConfiguration`
- `impl Eq for ContextServerConfiguration`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

