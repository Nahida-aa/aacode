---
id: Session
title: Session
---

# Struct: Session

Defined in: [`packages/project/src/debugger/session.rs:691`](../../../../packages/project/src/debugger/session.rs#L691)

Represents a current state of a single debug adapter and provides ways to mutate it.

## Definition

```rust
pub struct Session
{
    pub state: SessionState,
}
```

_（存在非公开字段）_

## Implementations

### task_context()

```rust
pub fn task_context(&self) -> &SharedTaskContext
```

Defined in: [`packages/project/src/debugger/session.rs:899`](../../../../packages/project/src/debugger/session.rs#L899)

#### Returns

`&SharedTaskContext`


***

### worktree()

```rust
pub fn worktree(&self) -> Option<Entity<Worktree>>
```

Defined in: [`packages/project/src/debugger/session.rs:903`](../../../../packages/project/src/debugger/session.rs#L903)

#### Returns

`Option<Entity<Worktree>>`


***

### boot()

```rust
pub fn boot(&self, binary: DebugAdapterBinary, worktree: Entity<Worktree>, dap_store: WeakEntity<DapStore>, cx: &Context<'_, Self>) -> Task<Result<()>>
```

Defined in: [`packages/project/src/debugger/session.rs:910`](../../../../packages/project/src/debugger/session.rs#L910)

#### Parameters

##### binary

`DebugAdapterBinary`

##### worktree

`Entity<Worktree>`

##### dap_store

`WeakEntity<DapStore>`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<()>>`


***

### session_id()

```rust
pub fn session_id(&self) -> SessionId
```

Defined in: [`packages/project/src/debugger/session.rs:1004`](../../../../packages/project/src/debugger/session.rs#L1004)

#### Returns

`SessionId`


***

### child_session_ids()

```rust
pub fn child_session_ids(&self) -> HashSet<SessionId>
```

Defined in: [`packages/project/src/debugger/session.rs:1008`](../../../../packages/project/src/debugger/session.rs#L1008)

#### Returns

`HashSet<SessionId>`


***

### add_child_session_id()

```rust
pub fn add_child_session_id(&self, session_id: SessionId)
```

Defined in: [`packages/project/src/debugger/session.rs:1012`](../../../../packages/project/src/debugger/session.rs#L1012)

#### Parameters

##### session_id

`SessionId`


***

### remove_child_session_id()

```rust
pub fn remove_child_session_id(&self, session_id: SessionId)
```

Defined in: [`packages/project/src/debugger/session.rs:1016`](../../../../packages/project/src/debugger/session.rs#L1016)

#### Parameters

##### session_id

`SessionId`


***

### parent_id()

```rust
pub fn parent_id(&self, cx: &App) -> Option<SessionId>
```

Defined in: [`packages/project/src/debugger/session.rs:1020`](../../../../packages/project/src/debugger/session.rs#L1020)

#### Parameters

##### cx

`&App`

#### Returns

`Option<SessionId>`


***

### parent_session()

```rust
pub fn parent_session(&self) -> Option<&Entity<Self>>
```

Defined in: [`packages/project/src/debugger/session.rs:1026`](../../../../packages/project/src/debugger/session.rs#L1026)

#### Returns

`Option<&Entity<Self>>`


***

### on_app_quit()

```rust
pub fn on_app_quit(&self, cx: &Context<'_, Self>) -> Task<()>
```

Defined in: [`packages/project/src/debugger/session.rs:1030`](../../../../packages/project/src/debugger/session.rs#L1030)

#### Parameters

##### cx

`&Context<'_, Self>`

#### Returns

`Task<()>`


***

### capabilities()

```rust
pub fn capabilities(&self) -> &Capabilities
```

Defined in: [`packages/project/src/debugger/session.rs:1061`](../../../../packages/project/src/debugger/session.rs#L1061)

#### Returns

`&Capabilities`


***

### binary()

```rust
pub fn binary(&self) -> Option<&DebugAdapterBinary>
```

Defined in: [`packages/project/src/debugger/session.rs:1065`](../../../../packages/project/src/debugger/session.rs#L1065)

#### Returns

`Option<&DebugAdapterBinary>`


***

### adapter()

```rust
pub fn adapter(&self) -> DebugAdapterName
```

Defined in: [`packages/project/src/debugger/session.rs:1072`](../../../../packages/project/src/debugger/session.rs#L1072)

#### Returns

`DebugAdapterName`


***

### label()

```rust
pub fn label(&self) -> Option<SharedString>
```

Defined in: [`packages/project/src/debugger/session.rs:1076`](../../../../packages/project/src/debugger/session.rs#L1076)

#### Returns

`Option<SharedString>`


***

### is_terminated()

```rust
pub fn is_terminated(&self) -> bool
```

Defined in: [`packages/project/src/debugger/session.rs:1080`](../../../../packages/project/src/debugger/session.rs#L1080)

#### Returns

`bool`


***

### console_output()

```rust
pub fn console_output(&self, cx: &Context<'_, Self>) -> UnboundedSender<String>
```

Defined in: [`packages/project/src/debugger/session.rs:1084`](../../../../packages/project/src/debugger/session.rs#L1084)

#### Parameters

##### cx

`&Context<'_, Self>`

#### Returns

`UnboundedSender<String>`


***

### is_started()

```rust
pub fn is_started(&self) -> bool
```

Defined in: [`packages/project/src/debugger/session.rs:1111`](../../../../packages/project/src/debugger/session.rs#L1111)

#### Returns

`bool`


***

### is_building()

```rust
pub fn is_building(&self) -> bool
```

Defined in: [`packages/project/src/debugger/session.rs:1118`](../../../../packages/project/src/debugger/session.rs#L1118)

#### Returns

`bool`


***

### as_running_mut()

```rust
pub fn as_running_mut(&self) -> Option<&RunningMode>
```

Defined in: [`packages/project/src/debugger/session.rs:1122`](../../../../packages/project/src/debugger/session.rs#L1122)

#### Returns

`Option<&RunningMode>`


***

### as_running()

```rust
pub fn as_running(&self) -> Option<&RunningMode>
```

Defined in: [`packages/project/src/debugger/session.rs:1129`](../../../../packages/project/src/debugger/session.rs#L1129)

#### Returns

`Option<&RunningMode>`


***

### run_to_position()

```rust
pub fn run_to_position(&self, breakpoint: SourceBreakpoint, active_thread_id: ThreadId, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/debugger/session.rs:1332`](../../../../packages/project/src/debugger/session.rs#L1332)

#### Parameters

##### breakpoint

[`SourceBreakpoint`](SourceBreakpoint.md)

##### active_thread_id

[`ThreadId`](ThreadId.md)

##### cx

`&Context<'_, Self>`


***

### has_new_output()

```rust
pub fn has_new_output(&self, last_update: OutputToken) -> bool
```

Defined in: [`packages/project/src/debugger/session.rs:1369`](../../../../packages/project/src/debugger/session.rs#L1369)

#### Parameters

##### last_update

[`OutputToken`](OutputToken.md)

#### Returns

`bool`


***

### output()

```rust
pub fn output(&self, since: OutputToken) -> (impl ?, OutputToken)
```

Defined in: [`packages/project/src/debugger/session.rs:1373`](../../../../packages/project/src/debugger/session.rs#L1373)

#### Parameters

##### since

[`OutputToken`](OutputToken.md)

#### Returns

`(impl ?, OutputToken)`


***

### respond_to_client()

```rust
pub fn respond_to_client(&self, request_seq: u64, success: bool, command: String, body: Option<Value>, cx: &Context<'_, Self>) -> Task<Result<()>>
```

Defined in: [`packages/project/src/debugger/session.rs:1391`](../../../../packages/project/src/debugger/session.rs#L1391)

#### Parameters

##### request_seq

`u64`

##### success

`bool`

##### command

`String`

##### body

`Option<Value>`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<()>>`


***

### historic_snapshots()

```rust
pub fn historic_snapshots(&self) -> &VecDeque<SessionSnapshot>
```

Defined in: [`packages/project/src/debugger/session.rs:1437`](../../../../packages/project/src/debugger/session.rs#L1437)

#### Returns

`&VecDeque<SessionSnapshot>`


***

### select_historic_snapshot()

```rust
pub fn select_historic_snapshot(&self, ix: Option<usize>, cx: &Context<'_, Session>)
```

Defined in: [`packages/project/src/debugger/session.rs:1441`](../../../../packages/project/src/debugger/session.rs#L1441)

#### Parameters

##### ix

`Option<usize>`

##### cx

`&Context<'_, Session>`


***

### active_snapshot_index()

```rust
pub fn active_snapshot_index(&self) -> Option<usize>
```

Defined in: [`packages/project/src/debugger/session.rs:1459`](../../../../packages/project/src/debugger/session.rs#L1459)

#### Returns

`Option<usize>`


***

### any_stopped_thread()

```rust
pub fn any_stopped_thread(&self) -> bool
```

Defined in: [`packages/project/src/debugger/session.rs:1793`](../../../../packages/project/src/debugger/session.rs#L1793)

#### Returns

`bool`


***

### thread_status()

```rust
pub fn thread_status(&self, thread_id: ThreadId) -> ThreadStatus
```

Defined in: [`packages/project/src/debugger/session.rs:1797`](../../../../packages/project/src/debugger/session.rs#L1797)

#### Parameters

##### thread_id

[`ThreadId`](ThreadId.md)

#### Returns

[`ThreadStatus`](../enums/ThreadStatus.md)


***

### threads()

```rust
pub fn threads(&self, cx: &Context<'_, Self>) -> Vec<(Thread, ThreadStatus)>
```

Defined in: [`packages/project/src/debugger/session.rs:1801`](../../../../packages/project/src/debugger/session.rs#L1801)

#### Parameters

##### cx

`&Context<'_, Self>`

#### Returns

`Vec<(Thread, ThreadStatus)>`


***

### modules()

```rust
pub fn modules(&self, cx: &Context<'_, Self>) -> &[Module]
```

Defined in: [`packages/project/src/debugger/session.rs:1834`](../../../../packages/project/src/debugger/session.rs#L1834)

#### Parameters

##### cx

`&Context<'_, Self>`

#### Returns

`&[Module]`


***

### data_access_size()

```rust
pub fn data_access_size(&self, frame_id: Option<u64>, evaluate_name: &str, cx: &Context<'_, Self>) -> Task<Option<u64>>
```

Defined in: [`packages/project/src/debugger/session.rs:1853`](../../../../packages/project/src/debugger/session.rs#L1853)

#### Parameters

##### frame_id

`Option<u64>`

##### evaluate_name

`&str`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Option<u64>>`


***

### memory_reference_of_expr()

```rust
pub fn memory_reference_of_expr(&self, frame_id: Option<u64>, expression: String, cx: &Context<'_, Self>) -> Task<Option<(String, Option<String>)>>
```

Defined in: [`packages/project/src/debugger/session.rs:1876`](../../../../packages/project/src/debugger/session.rs#L1876)

#### Parameters

##### frame_id

`Option<u64>`

##### expression

`String`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Option<(String, Option<String>)>>`


***

### write_memory()

```rust
pub fn write_memory(&self, address: u64, data: &[u8], cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/debugger/session.rs:1901`](../../../../packages/project/src/debugger/session.rs#L1901)

#### Parameters

##### address

`u64`

##### data

`&[u8]`

##### cx

`&Context<'_, Self>`


***

### read_memory()

```rust
pub fn read_memory(&self, range: RangeInclusive<u64>, cx: &Context<'_, Self>) -> MemoryIterator
```

Defined in: [`packages/project/src/debugger/session.rs:1921`](../../../../packages/project/src/debugger/session.rs#L1921)

#### Parameters

##### range

`RangeInclusive<u64>`

##### cx

`&Context<'_, Self>`

#### Returns

[`MemoryIterator`](MemoryIterator.md)


***

### ignore_breakpoints()

```rust
pub fn ignore_breakpoints(&self) -> bool
```

Defined in: [`packages/project/src/debugger/session.rs:1983`](../../../../packages/project/src/debugger/session.rs#L1983)

#### Returns

`bool`


***

### toggle_ignore_breakpoints()

```rust
pub fn toggle_ignore_breakpoints(&self, cx: &App) -> Task<HashMap<Arc<Path>, Error>>
```

Defined in: [`packages/project/src/debugger/session.rs:1987`](../../../../packages/project/src/debugger/session.rs#L1987)

#### Parameters

##### cx

`&App`

#### Returns

`Task<HashMap<Arc<Path>, Error>>`


***

### data_breakpoints()

```rust
pub fn data_breakpoints(&self) -> impl ?
```

Defined in: [`packages/project/src/debugger/session.rs:2013`](../../../../packages/project/src/debugger/session.rs#L2013)

#### Returns

`impl ?`


***

### exception_breakpoints()

```rust
pub fn exception_breakpoints(&self) -> impl ?
```

Defined in: [`packages/project/src/debugger/session.rs:2017`](../../../../packages/project/src/debugger/session.rs#L2017)

#### Returns

`impl ?`


***

### toggle_exception_breakpoint()

```rust
pub fn toggle_exception_breakpoint(&self, id: &str, cx: &App)
```

Defined in: [`packages/project/src/debugger/session.rs:2023`](../../../../packages/project/src/debugger/session.rs#L2023)

#### Parameters

##### id

`&str`

##### cx

`&App`


***

### toggle_data_breakpoint()

```rust
pub fn toggle_data_breakpoint(&self, id: &str, cx: &Context<'_, Session>)
```

Defined in: [`packages/project/src/debugger/session.rs:2050`](../../../../packages/project/src/debugger/session.rs#L2050)

#### Parameters

##### id

`&str`

##### cx

`&Context<'_, Session>`


***

### create_data_breakpoint()

```rust
pub fn create_data_breakpoint(&self, context: Arc<DataBreakpointContext>, data_id: String, dap: DataBreakpoint, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/debugger/session.rs:2069`](../../../../packages/project/src/debugger/session.rs#L2069)

#### Parameters

##### context

`Arc<DataBreakpointContext>`

##### data_id

`String`

##### dap

`DataBreakpoint`

##### cx

`&Context<'_, Self>`


***

### breakpoints_enabled()

```rust
pub fn breakpoints_enabled(&self) -> bool
```

Defined in: [`packages/project/src/debugger/session.rs:2089`](../../../../packages/project/src/debugger/session.rs#L2089)

#### Returns

`bool`


***

### loaded_sources()

```rust
pub fn loaded_sources(&self, cx: &Context<'_, Self>) -> &[Source]
```

Defined in: [`packages/project/src/debugger/session.rs:2093`](../../../../packages/project/src/debugger/session.rs#L2093)

#### Parameters

##### cx

`&Context<'_, Self>`

#### Returns

`&[Source]`


***

### pause_thread()

```rust
pub fn pause_thread(&self, thread_id: ThreadId, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/debugger/session.rs:2193`](../../../../packages/project/src/debugger/session.rs#L2193)

#### Parameters

##### thread_id

[`ThreadId`](ThreadId.md)

##### cx

`&Context<'_, Self>`


***

### restart_stack_frame()

```rust
pub fn restart_stack_frame(&self, stack_frame_id: u64, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/debugger/session.rs:2204`](../../../../packages/project/src/debugger/session.rs#L2204)

#### Parameters

##### stack_frame_id

`u64`

##### cx

`&Context<'_, Self>`


***

### restart()

```rust
pub fn restart(&self, args: Option<Value>, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/debugger/session.rs:2213`](../../../../packages/project/src/debugger/session.rs#L2213)

#### Parameters

##### args

`Option<Value>`

##### cx

`&Context<'_, Self>`


***

### shutdown()

```rust
pub fn shutdown(&self, cx: &Context<'_, Self>) -> Task<()>
```

Defined in: [`packages/project/src/debugger/session.rs:2246`](../../../../packages/project/src/debugger/session.rs#L2246)

#### Parameters

##### cx

`&Context<'_, Self>`

#### Returns

`Task<()>`


***

### completions()

```rust
pub fn completions(&self, query: CompletionsQuery, cx: &Context<'_, Self>) -> Task<Result<Vec<CompletionItem>>>
```

Defined in: [`packages/project/src/debugger/session.rs:2299`](../../../../packages/project/src/debugger/session.rs#L2299)

#### Parameters

##### query

[`CompletionsQuery`](CompletionsQuery.md)

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<Vec<CompletionItem>>>`


***

### continue_program()

```rust
pub fn continue_program(&self, thread_id: ThreadId, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/debugger/session.rs:2315`](../../../../packages/project/src/debugger/session.rs#L2315)

#### Parameters

##### thread_id

[`ThreadId`](ThreadId.md)

##### cx

`&Context<'_, Self>`


***

### continue_thread()

```rust
pub fn continue_thread(&self, thread_id: ThreadId, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/debugger/session.rs:2319`](../../../../packages/project/src/debugger/session.rs#L2319)

#### Parameters

##### thread_id

[`ThreadId`](ThreadId.md)

##### cx

`&Context<'_, Self>`


***

### adapter_client()

```rust
pub fn adapter_client(&self) -> Option<Arc<DebugAdapterClient>>
```

Defined in: [`packages/project/src/debugger/session.rs:2355`](../../../../packages/project/src/debugger/session.rs#L2355)

#### Returns

`Option<Arc<DebugAdapterClient>>`


***

### has_ever_stopped()

```rust
pub fn has_ever_stopped(&self) -> bool
```

Defined in: [`packages/project/src/debugger/session.rs:2362`](../../../../packages/project/src/debugger/session.rs#L2362)

#### Returns

`bool`


***

### step_over()

```rust
pub fn step_over(&self, thread_id: ThreadId, granularity: SteppingGranularity, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/debugger/session.rs:2366`](../../../../packages/project/src/debugger/session.rs#L2366)

#### Parameters

##### thread_id

[`ThreadId`](ThreadId.md)

##### granularity

`SteppingGranularity`

##### cx

`&Context<'_, Self>`


***

### step_in()

```rust
pub fn step_in(&self, thread_id: ThreadId, granularity: SteppingGranularity, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/debugger/session.rs:2398`](../../../../packages/project/src/debugger/session.rs#L2398)

#### Parameters

##### thread_id

[`ThreadId`](ThreadId.md)

##### granularity

`SteppingGranularity`

##### cx

`&Context<'_, Self>`


***

### step_out()

```rust
pub fn step_out(&self, thread_id: ThreadId, granularity: SteppingGranularity, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/debugger/session.rs:2430`](../../../../packages/project/src/debugger/session.rs#L2430)

#### Parameters

##### thread_id

[`ThreadId`](ThreadId.md)

##### granularity

`SteppingGranularity`

##### cx

`&Context<'_, Self>`


***

### step_back()

```rust
pub fn step_back(&self, thread_id: ThreadId, granularity: SteppingGranularity, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/debugger/session.rs:2462`](../../../../packages/project/src/debugger/session.rs#L2462)

#### Parameters

##### thread_id

[`ThreadId`](ThreadId.md)

##### granularity

`SteppingGranularity`

##### cx

`&Context<'_, Self>`


***

### stack_frames()

```rust
pub fn stack_frames(&self, thread_id: ThreadId, cx: &Context<'_, Self>) -> Result<Vec<StackFrame>>
```

Defined in: [`packages/project/src/debugger/session.rs:2495`](../../../../packages/project/src/debugger/session.rs#L2495)

#### Parameters

##### thread_id

[`ThreadId`](ThreadId.md)

##### cx

`&Context<'_, Self>`

#### Returns

`Result<Vec<StackFrame>>`


***

### scopes()

```rust
pub fn scopes(&self, stack_frame_id: u64, cx: &Context<'_, Self>) -> &[Scope]
```

Defined in: [`packages/project/src/debugger/session.rs:2575`](../../../../packages/project/src/debugger/session.rs#L2575)

#### Parameters

##### stack_frame_id

`u64`

##### cx

`&Context<'_, Self>`

#### Returns

`&[Scope]`


***

### variables_by_stack_frame_id()

```rust
pub fn variables_by_stack_frame_id(&self, stack_frame_id: StackFrameId, globals: bool, locals: bool) -> Vec<Variable>
```

Defined in: [`packages/project/src/debugger/session.rs:2618`](../../../../packages/project/src/debugger/session.rs#L2618)

#### Parameters

##### stack_frame_id

`StackFrameId`

##### globals

`bool`

##### locals

`bool`

#### Returns

`Vec<Variable>`


***

### watchers()

```rust
pub fn watchers(&self) -> &HashMap<SharedString, Watcher>
```

Defined in: [`packages/project/src/debugger/session.rs:2642`](../../../../packages/project/src/debugger/session.rs#L2642)

#### Returns

`&HashMap<SharedString, Watcher>`


***

### add_watcher()

```rust
pub fn add_watcher(&self, expression: SharedString, frame_id: u64, cx: &Context<'_, Self>) -> Task<Result<()>>
```

Defined in: [`packages/project/src/debugger/session.rs:2646`](../../../../packages/project/src/debugger/session.rs#L2646)

#### Parameters

##### expression

`SharedString`

##### frame_id

`u64`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<()>>`


***

### refresh_watchers()

```rust
pub fn refresh_watchers(&self, frame_id: u64, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/debugger/session.rs:2677`](../../../../packages/project/src/debugger/session.rs#L2677)

#### Parameters

##### frame_id

`u64`

##### cx

`&Context<'_, Self>`


***

### remove_watcher()

```rust
pub fn remove_watcher(&self, expression: SharedString)
```

Defined in: [`packages/project/src/debugger/session.rs:2685`](../../../../packages/project/src/debugger/session.rs#L2685)

#### Parameters

##### expression

`SharedString`


***

### variables()

```rust
pub fn variables(&self, variables_reference: VariableReference, cx: &Context<'_, Self>) -> Vec<Variable>
```

Defined in: [`packages/project/src/debugger/session.rs:2689`](../../../../packages/project/src/debugger/session.rs#L2689)

#### Parameters

##### variables_reference

`VariableReference`

##### cx

`&Context<'_, Self>`

#### Returns

`Vec<Variable>`


***

### data_breakpoint_info()

```rust
pub fn data_breakpoint_info(&self, context: Arc<DataBreakpointContext>, mode: Option<String>, cx: &Context<'_, Self>) -> Task<Option<DataBreakpointInfoResponse>>
```

Defined in: [`packages/project/src/debugger/session.rs:2756`](../../../../packages/project/src/debugger/session.rs#L2756)

#### Parameters

##### context

`Arc<DataBreakpointContext>`

##### mode

`Option<String>`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Option<DataBreakpointInfoResponse>>`


***

### set_variable_value()

```rust
pub fn set_variable_value(&self, stack_frame_id: u64, variables_reference: u64, name: String, value: String, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/debugger/session.rs:2767`](../../../../packages/project/src/debugger/session.rs#L2767)

#### Parameters

##### stack_frame_id

`u64`

##### variables_reference

`u64`

##### name

`String`

##### value

`String`

##### cx

`&Context<'_, Self>`


***

### evaluate()

```rust
pub fn evaluate(&self, expression: String, context: Option<EvaluateArgumentsContext>, frame_id: Option<u64>, source: Option<Source>, cx: &Context<'_, Self>) -> Task<()>
```

Defined in: [`packages/project/src/debugger/session.rs:2797`](../../../../packages/project/src/debugger/session.rs#L2797)

#### Parameters

##### expression

`String`

##### context

`Option<EvaluateArgumentsContext>`

##### frame_id

`Option<u64>`

##### source

`Option<Source>`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<()>`


***

### evaluate_variable_value()

```rust
pub fn evaluate_variable_value(&self, expression: String, frame_id: Option<u64>, cx: &Context<'_, Self>) -> Task<Option<String>>
```

Defined in: [`packages/project/src/debugger/session.rs:2867`](../../../../packages/project/src/debugger/session.rs#L2867)

Evaluates an expression to obtain its full value for copying.

#### Parameters

##### expression

`String`

##### frame_id

`Option<u64>`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Option<String>>`


***

### location()

```rust
pub fn location(&self, reference: u64, cx: &Context<'_, Self>) -> Option<LocationsResponse>
```

Defined in: [`packages/project/src/debugger/session.rs:2898`](../../../../packages/project/src/debugger/session.rs#L2898)

#### Parameters

##### reference

`u64`

##### cx

`&Context<'_, Self>`

#### Returns

`Option<LocationsResponse>`


***

### is_attached()

```rust
pub fn is_attached(&self) -> bool
```

Defined in: [`packages/project/src/debugger/session.rs:2916`](../../../../packages/project/src/debugger/session.rs#L2916)

#### Returns

`bool`


***

### disconnect_client()

```rust
pub fn disconnect_client(&self, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/debugger/session.rs:2923`](../../../../packages/project/src/debugger/session.rs#L2923)

#### Parameters

##### cx

`&Context<'_, Self>`


***

### terminate_threads()

```rust
pub fn terminate_threads(&self, thread_ids: Option<Vec<ThreadId>>, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/debugger/session.rs:2933`](../../../../packages/project/src/debugger/session.rs#L2933)

#### Parameters

##### thread_ids

`Option<Vec<ThreadId>>`

##### cx

`&Context<'_, Self>`


***

### thread_state()

```rust
pub fn thread_state(&self, thread_id: ThreadId) -> Option<ThreadStatus>
```

Defined in: [`packages/project/src/debugger/session.rs:2952`](../../../../packages/project/src/debugger/session.rs#L2952)

#### Parameters

##### thread_id

[`ThreadId`](ThreadId.md)

#### Returns

`Option<ThreadStatus>`


***

### quirks()

```rust
pub fn quirks(&self) -> SessionQuirks
```

Defined in: [`packages/project/src/debugger/session.rs:2956`](../../../../packages/project/src/debugger/session.rs#L2956)

#### Returns

[`SessionQuirks`](SessionQuirks.md)

## Trait Implementations

- `impl Borrow for Session`
- `impl BorrowMut for Session`
- `impl Into for Session`
- `impl From for Session`
- `impl TryInto for Session`
- `impl TryFrom for Session`
- `impl Any for Session`
- `impl VZip for Session`
- `impl CastableFrom for Session`
- `impl CastableFrom for Session`
- `impl Read for Session`
- `impl IntoEither for Session`
- `impl ErasedDestructor for Session`
- `impl Same for Session`
- `impl Pointable for Session`
- `impl Instrument for Session`
- `impl WithSubscriber for Session`
- `impl FromAngle for Session`
- `impl IntoAngle for Session`
- `impl IntoCam16Unclamped for Session`
- `impl Cam16IntoUnclamped for Session`
- `impl ArraysFrom for Session`
- `impl ArraysInto for Session`
- `impl ComponentsFrom for Session`
- `impl TryComponentsInto for Session`
- `impl UintsFrom for Session`
- `impl UintsInto for Session`
- `impl AdaptIntoUnclamped for Session`
- `impl AdaptInto for Session`
- `impl IntoColor for Session`
- `impl IntoColorUnclamped for Session`
- `impl TryIntoColor for Session`
- `impl FromStimulus for Session`
- `impl IntoStimulus for Session`
- `impl EventEmitter for Session`
- `impl EventEmitter for Session`

## Auto Trait Implementations

`Freeze` `Send` `Sync` `Unpin` `UnsafeUnpin`

