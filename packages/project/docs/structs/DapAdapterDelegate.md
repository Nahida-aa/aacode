---
id: DapAdapterDelegate
title: DapAdapterDelegate
---

# Struct: DapAdapterDelegate

Defined in: [`packages/project/src/debugger/dap_store.rs:939`](../../../../packages/project/src/debugger/dap_store.rs#L939)

## Definition

```rust
pub struct DapAdapterDelegate
```

_（存在非公开字段）_

## Implementations

### new()

```rust
pub fn new(fs: Arc<dyn Fs>, worktree: Snapshot, status: UnboundedSender<String>, node_runtime: NodeRuntime, http_client: Arc<dyn HttpClient>, toolchain_store: Arc<dyn LanguageToolchainStore>, load_shell_env_task: Shared<Task<Option<HashMap<String, String>>>>, is_headless: bool) -> Self
```

Defined in: [`packages/project/src/debugger/dap_store.rs:951`](../../../../packages/project/src/debugger/dap_store.rs#L951)

#### Parameters

##### fs

`Arc<dyn Fs>`

##### worktree

`Snapshot`

##### status

`UnboundedSender<String>`

##### node_runtime

`NodeRuntime`

##### http_client

`Arc<dyn HttpClient>`

##### toolchain_store

`Arc<dyn LanguageToolchainStore>`

##### load_shell_env_task

`Shared<Task<Option<HashMap<String, String>>>>`

##### is_headless

`bool`

#### Returns

`Self`

## Trait Implementations

- `impl Borrow for DapAdapterDelegate`
- `impl BorrowMut for DapAdapterDelegate`
- `impl CloneToUninit for DapAdapterDelegate`
- `impl Into for DapAdapterDelegate`
- `impl From for DapAdapterDelegate`
- `impl TryInto for DapAdapterDelegate`
- `impl TryFrom for DapAdapterDelegate`
- `impl Any for DapAdapterDelegate`
- `impl ToOwned for DapAdapterDelegate`
- `impl DynClone for DapAdapterDelegate`
- `impl VZip for DapAdapterDelegate`
- `impl CastableFrom for DapAdapterDelegate`
- `impl CastableFrom for DapAdapterDelegate`
- `impl Read for DapAdapterDelegate`
- `impl IntoEither for DapAdapterDelegate`
- `impl ErasedDestructor for DapAdapterDelegate`
- `impl Same for DapAdapterDelegate`
- `impl Pointable for DapAdapterDelegate`
- `impl Instrument for DapAdapterDelegate`
- `impl WithSubscriber for DapAdapterDelegate`
- `impl FromAngle for DapAdapterDelegate`
- `impl IntoAngle for DapAdapterDelegate`
- `impl IntoCam16Unclamped for DapAdapterDelegate`
- `impl Cam16IntoUnclamped for DapAdapterDelegate`
- `impl ArraysFrom for DapAdapterDelegate`
- `impl ArraysInto for DapAdapterDelegate`
- `impl ComponentsFrom for DapAdapterDelegate`
- `impl TryComponentsInto for DapAdapterDelegate`
- `impl UintsFrom for DapAdapterDelegate`
- `impl UintsInto for DapAdapterDelegate`
- `impl AdaptIntoUnclamped for DapAdapterDelegate`
- `impl AdaptInto for DapAdapterDelegate`
- `impl IntoColor for DapAdapterDelegate`
- `impl IntoColorUnclamped for DapAdapterDelegate`
- `impl TryIntoColor for DapAdapterDelegate`
- `impl FromStimulus for DapAdapterDelegate`
- `impl IntoStimulus for DapAdapterDelegate`
- `impl Clone for DapAdapterDelegate`
- `impl DapDelegate for DapAdapterDelegate`

## Auto Trait Implementations

`Freeze` `Send` `Sync` `Unpin` `UnsafeUnpin`

