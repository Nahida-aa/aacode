---
id: ContextServerDescriptorRegistry
title: ContextServerDescriptorRegistry
---

# Struct: ContextServerDescriptorRegistry

Defined in: [`packages/project/src/context_server_store/registry.rs:29`](../../../../packages/project/src/context_server_store/registry.rs#L29)

## Definition

```rust
pub struct ContextServerDescriptorRegistry
```

_（存在非公开字段）_

## Implementations

### default_global()

```rust
pub fn default_global(cx: &App) -> Entity<Self>
```

Defined in: [`packages/project/src/context_server_store/registry.rs:37`](../../../../packages/project/src/context_server_store/registry.rs#L37)

Returns the global [`ContextServerDescriptorRegistry`](ContextServerDescriptorRegistry.md).

Inserts a default [`ContextServerDescriptorRegistry`](ContextServerDescriptorRegistry.md) if one does not yet exist.

#### Parameters

##### cx

`&App`

#### Returns

`Entity<Self>`


***

### new()

```rust
pub fn new() -> Self
```

Defined in: [`packages/project/src/context_server_store/registry.rs:47`](../../../../packages/project/src/context_server_store/registry.rs#L47)

#### Returns

`Self`


***

### context_server_descriptors()

```rust
pub fn context_server_descriptors(&self) -> Vec<(Arc<str>, Arc<dyn ContextServerDescriptor>)>
```

Defined in: [`packages/project/src/context_server_store/registry.rs:53`](../../../../packages/project/src/context_server_store/registry.rs#L53)

#### Returns

`Vec<(Arc<str>, Arc<dyn ContextServerDescriptor>)>`


***

### context_server_descriptor()

```rust
pub fn context_server_descriptor(&self, id: &str) -> Option<Arc<dyn ContextServerDescriptor>>
```

Defined in: [`packages/project/src/context_server_store/registry.rs:60`](../../../../packages/project/src/context_server_store/registry.rs#L60)

#### Parameters

##### id

`&str`

#### Returns

`Option<Arc<dyn ContextServerDescriptor>>`


***

### register_context_server_descriptor()

```rust
pub fn register_context_server_descriptor(&self, id: Arc<str>, descriptor: Arc<dyn ContextServerDescriptor>, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/context_server_store/registry.rs:65`](../../../../packages/project/src/context_server_store/registry.rs#L65)

Registers the provided [`ContextServerDescriptor`](../traits/ContextServerDescriptor.md).

#### Parameters

##### id

`Arc<str>`

##### descriptor

`Arc<dyn ContextServerDescriptor>`

##### cx

`&Context<'_, Self>`


***

### unregister_context_server_descriptor_by_id()

```rust
pub fn unregister_context_server_descriptor_by_id(&self, server_id: &str, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/context_server_store/registry.rs:76`](../../../../packages/project/src/context_server_store/registry.rs#L76)

Unregisters the [`ContextServerDescriptor`](../traits/ContextServerDescriptor.md) for the server with the given ID.

#### Parameters

##### server_id

`&str`

##### cx

`&Context<'_, Self>`

## Trait Implementations

- `impl Borrow for ContextServerDescriptorRegistry`
- `impl BorrowMut for ContextServerDescriptorRegistry`
- `impl Into for ContextServerDescriptorRegistry`
- `impl From for ContextServerDescriptorRegistry`
- `impl TryInto for ContextServerDescriptorRegistry`
- `impl TryFrom for ContextServerDescriptorRegistry`
- `impl Any for ContextServerDescriptorRegistry`
- `impl VZip for ContextServerDescriptorRegistry`
- `impl CastableFrom for ContextServerDescriptorRegistry`
- `impl CastableFrom for ContextServerDescriptorRegistry`
- `impl Read for ContextServerDescriptorRegistry`
- `impl IntoEither for ContextServerDescriptorRegistry`
- `impl ErasedDestructor for ContextServerDescriptorRegistry`
- `impl Same for ContextServerDescriptorRegistry`
- `impl ReadPrimitive for ContextServerDescriptorRegistry`
- `impl Pointable for ContextServerDescriptorRegistry`
- `impl Instrument for ContextServerDescriptorRegistry`
- `impl WithSubscriber for ContextServerDescriptorRegistry`
- `impl FromAngle for ContextServerDescriptorRegistry`
- `impl IntoAngle for ContextServerDescriptorRegistry`
- `impl IntoCam16Unclamped for ContextServerDescriptorRegistry`
- `impl Cam16IntoUnclamped for ContextServerDescriptorRegistry`
- `impl ArraysFrom for ContextServerDescriptorRegistry`
- `impl ArraysInto for ContextServerDescriptorRegistry`
- `impl ComponentsFrom for ContextServerDescriptorRegistry`
- `impl TryComponentsInto for ContextServerDescriptorRegistry`
- `impl UintsFrom for ContextServerDescriptorRegistry`
- `impl UintsInto for ContextServerDescriptorRegistry`
- `impl AdaptIntoUnclamped for ContextServerDescriptorRegistry`
- `impl AdaptInto for ContextServerDescriptorRegistry`
- `impl IntoColor for ContextServerDescriptorRegistry`
- `impl IntoColorUnclamped for ContextServerDescriptorRegistry`
- `impl TryIntoColor for ContextServerDescriptorRegistry`
- `impl FromStimulus for ContextServerDescriptorRegistry`
- `impl IntoStimulus for ContextServerDescriptorRegistry`
- `impl Default for ContextServerDescriptorRegistry`

## Auto Trait Implementations

`Freeze` `Unpin` `UnsafeUnpin`

