---
id: GitJobDebugQueue
title: GitJobDebugQueue
---

# Struct: GitJobDebugQueue

Defined in: [`packages/project/src/git_store/job_debug_queue.rs:7`](../../../../packages/project/src/git_store/job_debug_queue.rs#L7)

## Definition

```rust
pub struct GitJobDebugQueue
```

_（存在非公开字段）_

## Implementations

### new()

```rust
pub fn new() -> Self
```

Defined in: [`packages/project/src/git_store/job_debug_queue.rs:50`](../../../../packages/project/src/git_store/job_debug_queue.rs#L50)

#### Returns

`Self`


***

### add()

```rust
pub fn add(&self, id: u64, description: &'static str, key: Option<SharedString>)
```

Defined in: [`packages/project/src/git_store/job_debug_queue.rs:58`](../../../../packages/project/src/git_store/job_debug_queue.rs#L58)

#### Parameters

##### id

`u64`

##### description

`&'static str`

##### key

`Option<SharedString>`


***

### mark_running()

```rust
pub fn mark_running(&self, id: u64)
```

Defined in: [`packages/project/src/git_store/job_debug_queue.rs:67`](../../../../packages/project/src/git_store/job_debug_queue.rs#L67)

#### Parameters

##### id

`u64`


***

### mark_unfinished_complete()

```rust
pub fn mark_unfinished_complete(&self, status: CompletedJobStatus)
```

Defined in: [`packages/project/src/git_store/job_debug_queue.rs:83`](../../../../packages/project/src/git_store/job_debug_queue.rs#L83)

#### Parameters

##### status

[`CompletedJobStatus`](../enums/CompletedJobStatus.md)


***

### mark_complete()

```rust
pub fn mark_complete(&self, id: u64, status: CompletedJobStatus)
```

Defined in: [`packages/project/src/git_store/job_debug_queue.rs:95`](../../../../packages/project/src/git_store/job_debug_queue.rs#L95)

#### Parameters

##### id

`u64`

##### status

[`CompletedJobStatus`](../enums/CompletedJobStatus.md)


***

### to_debug_string()

```rust
pub fn to_debug_string(&self) -> String
```

Defined in: [`packages/project/src/git_store/job_debug_queue.rs:127`](../../../../packages/project/src/git_store/job_debug_queue.rs#L127)

#### Returns

`String`


***

### to_debug_value()

```rust
pub fn to_debug_value(&self) -> Value
```

Defined in: [`packages/project/src/git_store/job_debug_queue.rs:131`](../../../../packages/project/src/git_store/job_debug_queue.rs#L131)

#### Returns

`Value`

## Trait Implementations

- `impl Borrow for GitJobDebugQueue`
- `impl BorrowMut for GitJobDebugQueue`
- `impl Into for GitJobDebugQueue`
- `impl From for GitJobDebugQueue`
- `impl TryInto for GitJobDebugQueue`
- `impl TryFrom for GitJobDebugQueue`
- `impl Any for GitJobDebugQueue`
- `impl VZip for GitJobDebugQueue`
- `impl CastableFrom for GitJobDebugQueue`
- `impl CastableFrom for GitJobDebugQueue`
- `impl Read for GitJobDebugQueue`
- `impl IntoEither for GitJobDebugQueue`
- `impl ErasedDestructor for GitJobDebugQueue`
- `impl Same for GitJobDebugQueue`
- `impl Pointable for GitJobDebugQueue`
- `impl Instrument for GitJobDebugQueue`
- `impl WithSubscriber for GitJobDebugQueue`
- `impl FromAngle for GitJobDebugQueue`
- `impl IntoAngle for GitJobDebugQueue`
- `impl IntoCam16Unclamped for GitJobDebugQueue`
- `impl Cam16IntoUnclamped for GitJobDebugQueue`
- `impl ArraysFrom for GitJobDebugQueue`
- `impl ArraysInto for GitJobDebugQueue`
- `impl ComponentsFrom for GitJobDebugQueue`
- `impl TryComponentsInto for GitJobDebugQueue`
- `impl UintsFrom for GitJobDebugQueue`
- `impl UintsInto for GitJobDebugQueue`
- `impl AdaptIntoUnclamped for GitJobDebugQueue`
- `impl AdaptInto for GitJobDebugQueue`
- `impl IntoColor for GitJobDebugQueue`
- `impl IntoColorUnclamped for GitJobDebugQueue`
- `impl TryIntoColor for GitJobDebugQueue`
- `impl FromStimulus for GitJobDebugQueue`
- `impl IntoStimulus for GitJobDebugQueue`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

