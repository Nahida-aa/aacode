---
id: Project
title: Project
---

# Struct: Project

Defined in: [`packages/project/src/project/mod.rs:101`](../../../../packages/project/src/project/mod.rs#L101)

## Definition

```rust
pub struct Project
```

_（存在非公开字段）_

## Implementations

### code_lens_actions()

```rust
pub fn code_lens_actions(&self, buffer: &Entity<Buffer>, range: Range<Anchor>, cx: &Context<'_, Self>) -> Task<Result<Option<Vec<CodeAction>>>>
```

Defined in: [`packages/project/src/lsp_store/code_lens.rs:525`](../../../../packages/project/src/lsp_store/code_lens.rs#L525)

#### Parameters

##### buffer

`&Entity<Buffer>`

##### range

`Range<Anchor>`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<Option<Vec<CodeAction>>>>`


***

### active_entry_directory()

```rust
pub fn active_entry_directory(&self, cx: &App) -> Option<PathBuf>
```

Defined in: [`packages/project/src/terminals.rs:33`](../../../../packages/project/src/terminals.rs#L33)

#### Parameters

##### cx

`&App`

#### Returns

`Option<PathBuf>`


***

### active_project_directory()

```rust
pub fn active_project_directory(&self, cx: &App) -> Option<Arc<Path>>
```

Defined in: [`packages/project/src/terminals.rs:47`](../../../../packages/project/src/terminals.rs#L47)

#### Parameters

##### cx

`&App`

#### Returns

`Option<Arc<Path>>`


***

### first_project_directory()

```rust
pub fn first_project_directory(&self, cx: &App) -> Option<PathBuf>
```

Defined in: [`packages/project/src/terminals.rs:55`](../../../../packages/project/src/terminals.rs#L55)

#### Parameters

##### cx

`&App`

#### Returns

`Option<PathBuf>`


***

### create_terminal_task()

```rust
pub fn create_terminal_task(&self, spawn_task: SpawnInTerminal, cx: &Context<'_, Self>) -> Task<Result<Entity<Terminal>>>
```

Defined in: [`packages/project/src/terminals.rs:65`](../../../../packages/project/src/terminals.rs#L65)

#### Parameters

##### spawn_task

`SpawnInTerminal`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<Entity<Terminal>>>`


***

### create_terminal_shell()

```rust
pub fn create_terminal_shell(&self, cwd: Option<PathBuf>, cx: &Context<'_, Self>) -> Task<Result<Entity<Terminal>>>
```

Defined in: [`packages/project/src/terminals.rs:285`](../../../../packages/project/src/terminals.rs#L285)

#### Parameters

##### cwd

`Option<PathBuf>`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<Entity<Terminal>>>`


***

### create_local_terminal()

```rust
pub fn create_local_terminal(&self, cx: &Context<'_, Self>) -> Task<Result<Entity<Terminal>>>
```

Defined in: [`packages/project/src/terminals.rs:296`](../../../../packages/project/src/terminals.rs#L296)

Creates a local terminal even if the project is remote.
In remote projects: opens in Zed's launch directory (bypasses SSH).
In local projects: opens in the project directory (same as regular terminals).

#### Parameters

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<Entity<Terminal>>>`


***

### clone_terminal()

```rust
pub fn clone_terminal(&self, terminal: &Entity<Terminal>, cx: &Context<'_, Project>, cwd: Option<PathBuf>) -> Task<Result<Entity<Terminal>>>
```

Defined in: [`packages/project/src/terminals.rs:481`](../../../../packages/project/src/terminals.rs#L481)

#### Parameters

##### terminal

`&Entity<Terminal>`

##### cx

`&Context<'_, Project>`

##### cwd

`Option<PathBuf>`

#### Returns

`Task<Result<Entity<Terminal>>>`


***

### terminal_settings()

```rust
pub fn terminal_settings<'a>(&self, path: &'a Option<PathBuf>, cx: &'a App) -> &'a TerminalSettings
```

Defined in: [`packages/project/src/terminals.rs:528`](../../../../packages/project/src/terminals.rs#L528)

#### Parameters

##### path

`&'a Option<PathBuf>`

##### cx

`&'a App`

#### Returns

`&'a TerminalSettings`


***

### exec_in_shell()

```rust
pub fn exec_in_shell(&self, command: String, cx: &Context<'_, Self>) -> Task<Result<Command>>
```

Defined in: [`packages/project/src/terminals.rs:545`](../../../../packages/project/src/terminals.rs#L545)

#### Parameters

##### command

`String`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<Command>>`


***

### local_terminal_handles()

```rust
pub fn local_terminal_handles(&self) -> &Vec<WeakEntity<Terminal>>
```

Defined in: [`packages/project/src/terminals.rs:607`](../../../../packages/project/src/terminals.rs#L607)

#### Returns

`&Vec<WeakEntity<Terminal>>`


***

### set_agent_location()

```rust
pub fn set_agent_location(&self, new_location: Option<AgentLocation>, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/project/ai.rs:11`](../../../../packages/project/src/project/ai.rs#L11)

#### Parameters

##### new_location

`Option<AgentLocation>`

##### cx

`&Context<'_, Self>`


***

### agent_location()

```rust
pub fn agent_location(&self) -> Option<AgentLocation>
```

Defined in: [`packages/project/src/project/ai.rs:46`](../../../../packages/project/src/project/ai.rs#L46)

#### Returns

`Option<AgentLocation>`


***

### create_buffer()

```rust
pub fn create_buffer(&self, language: Option<Arc<Language>>, project_searchable: bool, cx: &Context<'_, Self>) -> Task<Result<Entity<Buffer>>>
```

Defined in: [`packages/project/src/project/buffers.rs:33`](../../../../packages/project/src/project/buffers.rs#L33)

#### Parameters

##### language

`Option<Arc<Language>>`

##### project_searchable

`bool`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<Entity<Buffer>>>`


***

### create_local_buffer()

```rust
pub fn create_local_buffer(&self, text: &str, language: Option<Arc<Language>>, project_searchable: bool, cx: &Context<'_, Self>) -> Entity<Buffer>
```

Defined in: [`packages/project/src/project/buffers.rs:44`](../../../../packages/project/src/project/buffers.rs#L44)

#### Parameters

##### text

`&str`

##### language

`Option<Arc<Language>>`

##### project_searchable

`bool`

##### cx

`&Context<'_, Self>`

#### Returns

`Entity<Buffer>`


***

### open_path()

```rust
pub fn open_path(&self, path: ProjectPath, cx: &Context<'_, Self>) -> Task<Result<(Option<ProjectEntryId>, Entity<Buffer>)>>
```

Defined in: [`packages/project/src/project/buffers.rs:59`](../../../../packages/project/src/project/buffers.rs#L59)

#### Parameters

##### path

[`ProjectPath`](ProjectPath.md)

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<(Option<ProjectEntryId>, Entity<Buffer>)>>`


***

### open_local_buffer()

```rust
pub fn open_local_buffer<impl AsRef<Path>: AsRef>(&self, abs_path: impl ?, cx: &Context<'_, Self>) -> Task<Result<Entity<Buffer>>>
```

Defined in: [`packages/project/src/project/buffers.rs:75`](../../../../packages/project/src/project/buffers.rs#L75)

#### Parameters

##### abs_path

`impl ?`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<Entity<Buffer>>>`


***

### open_buffer()

```rust
pub fn open_buffer<impl Into<ProjectPath>: Into>(&self, path: impl ?, cx: &App) -> Task<Result<Entity<Buffer>>>
```

Defined in: [`packages/project/src/project/buffers.rs:90`](../../../../packages/project/src/project/buffers.rs#L90)

#### Parameters

##### path

`impl ?`

##### cx

`&App`

#### Returns

`Task<Result<Entity<Buffer>>>`


***

### open_buffer_by_id()

```rust
pub fn open_buffer_by_id(&self, id: BufferId, cx: &Context<'_, Self>) -> Task<Result<Entity<Buffer>>>
```

Defined in: [`packages/project/src/project/buffers.rs:104`](../../../../packages/project/src/project/buffers.rs#L104)

#### Parameters

##### id

`BufferId`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<Entity<Buffer>>>`


***

### save_buffers()

```rust
pub fn save_buffers(&self, buffers: HashSet<Entity<Buffer>>, cx: &Context<'_, Self>) -> Task<Result<()>>
```

Defined in: [`packages/project/src/project/buffers.rs:133`](../../../../packages/project/src/project/buffers.rs#L133)

#### Parameters

##### buffers

`HashSet<Entity<Buffer>>`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<()>>`


***

### save_buffer()

```rust
pub fn save_buffer(&self, buffer: Entity<Buffer>, cx: &Context<'_, Self>) -> Task<Result<()>>
```

Defined in: [`packages/project/src/project/buffers.rs:148`](../../../../packages/project/src/project/buffers.rs#L148)

#### Parameters

##### buffer

`Entity<Buffer>`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<()>>`


***

### save_buffer_as()

```rust
pub fn save_buffer_as(&self, buffer: Entity<Buffer>, path: ProjectPath, cx: &Context<'_, Self>) -> Task<Result<()>>
```

Defined in: [`packages/project/src/project/buffers.rs:152`](../../../../packages/project/src/project/buffers.rs#L152)

#### Parameters

##### buffer

`Entity<Buffer>`

##### path

[`ProjectPath`](ProjectPath.md)

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<()>>`


***

### get_open_buffer()

```rust
pub fn get_open_buffer(&self, path: &ProjectPath, cx: &App) -> Option<Entity<Buffer>>
```

Defined in: [`packages/project/src/project/buffers.rs:162`](../../../../packages/project/src/project/buffers.rs#L162)

#### Parameters

##### path

`&ProjectPath`

##### cx

`&App`

#### Returns

`Option<Entity<Buffer>>`


***

### dirty_buffers()

```rust
pub fn dirty_buffers<'a>(&self, cx: &'a App) -> impl ? + ?
```

Defined in: [`packages/project/src/project/buffers.rs:165`](../../../../packages/project/src/project/buffers.rs#L165)

#### Parameters

##### cx

`&'a App`

#### Returns

`impl ? + ?`


***

### reload_buffers()

```rust
pub fn reload_buffers(&self, buffers: HashSet<Entity<Buffer>>, push_to_history: bool, cx: &Context<'_, Self>) -> Task<Result<ProjectTransaction>>
```

Defined in: [`packages/project/src/project/buffers.rs:175`](../../../../packages/project/src/project/buffers.rs#L175)

#### Parameters

##### buffers

`HashSet<Entity<Buffer>>`

##### push_to_history

`bool`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<ProjectTransaction>>`


***

### blame_buffer()

```rust
pub fn blame_buffer(&self, buffer: &Entity<Buffer>, version: Option<Global>, cx: &Context<'_, Self>) -> Task<Result<Option<Blame>>>
```

Defined in: [`packages/project/src/project/buffers.rs:565`](../../../../packages/project/src/project/buffers.rs#L565)

Blames a buffer（对齐 Zed `Project::blame_buffer`）。

转发到 `GitStore::blame_buffer`。

#### Parameters

##### buffer

`&Entity<Buffer>`

##### version

`Option<Global>`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<Option<Blame>>>`


***

### in_room()

```rust
pub async fn in_room(remote_id: u64, client: Arc<Client>, user_store: Entity<UserStore>, languages: Arc<LanguageRegistry>, fs: Arc<dyn Fs>, cx: AsyncApp) -> Result<Entity<Self>>
```

Defined in: [`packages/project/src/project/collab.rs:21`](../../../../packages/project/src/project/collab.rs#L21)

#### Parameters

##### remote_id

`u64`

##### client

`Arc<Client>`

##### user_store

`Entity<UserStore>`

##### languages

`Arc<LanguageRegistry>`

##### fs

`Arc<dyn Fs>`

##### cx

`AsyncApp`

#### Returns

`Result<Entity<Self>>`


***

### local()

```rust
pub fn local(client: Arc<Client>, node: NodeRuntime, user_store: Entity<UserStore>, languages: Arc<LanguageRegistry>, fs: Arc<dyn Fs>, env: Option<HashMap<String, String>>, flags: LocalProjectFlags, cx: &App) -> Entity<Self>
```

Defined in: [`packages/project/src/project/constructors.rs:29`](../../../../packages/project/src/project/constructors.rs#L29)

#### Parameters

##### client

`Arc<Client>`

##### node

`NodeRuntime`

##### user_store

`Entity<UserStore>`

##### languages

`Arc<LanguageRegistry>`

##### fs

`Arc<dyn Fs>`

##### env

`Option<HashMap<String, String>>`

##### flags

[`LocalProjectFlags`](LocalProjectFlags.md)

##### cx

`&App`

#### Returns

`Entity<Self>`


***

### remote()

```rust
pub fn remote(remote: Entity<RemoteClient>, client: Arc<Client>, node: NodeRuntime, user_store: Entity<UserStore>, languages: Arc<LanguageRegistry>, fs: Arc<dyn Fs>, init_worktree_trust: bool, cx: &App) -> Entity<Self>
```

Defined in: [`packages/project/src/project/constructors.rs:235`](../../../../packages/project/src/project/constructors.rs#L235)

#### Parameters

##### remote

`Entity<RemoteClient>`

##### client

`Arc<Client>`

##### node

`NodeRuntime`

##### user_store

`Entity<UserStore>`

##### languages

`Arc<LanguageRegistry>`

##### fs

`Arc<dyn Fs>`

##### init_worktree_trust

`bool`

##### cx

`&App`

#### Returns

`Entity<Self>`


***

### active_debug_session()

```rust
pub fn active_debug_session(&self, cx: &App) -> Option<(Entity<Session>, ActiveStackFrame)>
```

Defined in: [`packages/project/src/project/dap.rs:20`](../../../../packages/project/src/project/dap.rs#L20)

#### Parameters

##### cx

`&App`

#### Returns

`Option<(Entity<Session>, ActiveStackFrame)>`


***

### diagnostic_summary()

```rust
pub fn diagnostic_summary(&self, include_ignored: bool, cx: &App) -> DiagnosticSummary
```

Defined in: [`packages/project/src/project/diagnostics.rs:6`](../../../../packages/project/src/project/diagnostics.rs#L6)

#### Parameters

##### include_ignored

`bool`

##### cx

`&App`

#### Returns

[`DiagnosticSummary`](DiagnosticSummary.md)


***

### diagnostic_summary_for_path()

```rust
pub fn diagnostic_summary_for_path(&self, path: &ProjectPath, cx: &App) -> DiagnosticSummary
```

Defined in: [`packages/project/src/project/diagnostics.rs:13`](../../../../packages/project/src/project/diagnostics.rs#L13)

Returns a summary of the diagnostics for the provided project path only.

#### Parameters

##### path

`&ProjectPath`

##### cx

`&App`

#### Returns

[`DiagnosticSummary`](DiagnosticSummary.md)


***

### diagnostic_summaries()

```rust
pub fn diagnostic_summaries<'a>(&self, include_ignored: bool, cx: &'a App) -> impl ? + ?
```

Defined in: [`packages/project/src/project/diagnostics.rs:19`](../../../../packages/project/src/project/diagnostics.rs#L19)

#### Parameters

##### include_ignored

`bool`

##### cx

`&'a App`

#### Returns

`impl ? + ?`


***

### peek_environment_error()

```rust
pub fn peek_environment_error<'a>(&self, cx: &'a App) -> Option<&'a String>
```

Defined in: [`packages/project/src/project/env.rs:14`](../../../../packages/project/src/project/env.rs#L14)

#### Parameters

##### cx

`&'a App`

#### Returns

`Option<&'a String>`


***

### pop_environment_error()

```rust
pub fn pop_environment_error(&self, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/project/env.rs:19`](../../../../packages/project/src/project/env.rs#L19)

#### Parameters

##### cx

`&Context<'_, Self>`


***

### repositories()

```rust
pub fn repositories<'a>(&self, cx: &'a App) -> &'a HashMap<RepositoryId, Entity<Repository>>
```

Defined in: [`packages/project/src/project/git.rs:14`](../../../../packages/project/src/project/git.rs#L14)

#### Parameters

##### cx

`&'a App`

#### Returns

`&'a HashMap<RepositoryId, Entity<Repository>>`


***

### active_repository()

```rust
pub fn active_repository(&self, cx: &App) -> Option<Entity<Repository>>
```

Defined in: [`packages/project/src/project/git.rs:19`](../../../../packages/project/src/project/git.rs#L19)

#### Parameters

##### cx

`&App`

#### Returns

`Option<Entity<Repository>>`


***

### status_for_buffer_id()

```rust
pub fn status_for_buffer_id(&self, buffer_id: BufferId, cx: &App) -> Option<FileStatus>
```

Defined in: [`packages/project/src/project/git.rs:23`](../../../../packages/project/src/project/git.rs#L23)

#### Parameters

##### buffer_id

`BufferId`

##### cx

`&App`

#### Returns

`Option<FileStatus>`


***

### stage_hunks()

```rust
pub fn stage_hunks(&self, buffer: Entity<Buffer>, unstaged_diff: Entity<BufferDiff>, worktree_ranges: Vec<Range<Anchor>>, cx: &Context<'_, Self>) -> Result<()>
```

Defined in: [`packages/project/src/project/git.rs:30`](../../../../packages/project/src/project/git.rs#L30)

Stages the worktree changes covered by `worktree_ranges` (in the worktree
buffer's coordinates), acting on the given unstaged diff. Used by both the
unstaged-changes view and the uncommitted (gutter) controls.

#### Parameters

##### buffer

`Entity<Buffer>`

##### unstaged_diff

`Entity<BufferDiff>`

##### worktree_ranges

`Vec<Range<Anchor>>`

##### cx

`&Context<'_, Self>`

#### Returns

`Result<()>`


***

### unstage_uncommitted_hunks()

```rust
pub fn unstage_uncommitted_hunks(&self, buffer: Entity<Buffer>, uncommitted_diff: Entity<BufferDiff>, worktree_ranges: Vec<Range<Anchor>>, cx: &Context<'_, Self>) -> Result<()>
```

Defined in: [`packages/project/src/project/git.rs:48`](../../../../packages/project/src/project/git.rs#L48)

Unstages the worktree changes covered by `worktree_ranges` (in the worktree
buffer's coordinates), acting on the given uncommitted diff. Used by the
uncommitted (gutter) controls.

#### Parameters

##### buffer

`Entity<Buffer>`

##### uncommitted_diff

`Entity<BufferDiff>`

##### worktree_ranges

`Vec<Range<Anchor>>`

##### cx

`&Context<'_, Self>`

#### Returns

`Result<()>`


***

### unstage_staged_hunks()

```rust
pub fn unstage_staged_hunks(&self, staged_diff: Entity<BufferDiff>, index_ranges: Vec<Range<Anchor>>, cx: &Context<'_, Self>) -> Result<()>
```

Defined in: [`packages/project/src/project/git.rs:66`](../../../../packages/project/src/project/git.rs#L66)

Unstages the staged changes covered by `index_ranges` (in the index
buffer's coordinates), acting on the given staged diff. Used by the
staged-changes view.

#### Parameters

##### staged_diff

`Entity<BufferDiff>`

##### index_ranges

`Vec<Range<Anchor>>`

##### cx

`&Context<'_, Self>`

#### Returns

`Result<()>`


***

### get_permalink_to_line()

```rust
pub fn get_permalink_to_line(&self, buffer: &Entity<Buffer>, selection: Range<u32>, cx: &App) -> Task<Result<Url>>
```

Defined in: [`packages/project/src/project/git.rs:80`](../../../../packages/project/src/project/git.rs#L80)

#### Parameters

##### buffer

`&Entity<Buffer>`

##### selection

`Range<u32>`

##### cx

`&App`

#### Returns

`Task<Result<Url>>`


***

### get_file_permalink()

```rust
pub fn get_file_permalink(&self, project_path: &ProjectPath, cx: &App) -> Task<Result<Url>>
```

Defined in: [`packages/project/src/project/git.rs:91`](../../../../packages/project/src/project/git.rs#L91)

#### Parameters

##### project_path

`&ProjectPath`

##### cx

`&App`

#### Returns

`Task<Result<Url>>`


***

### open_uncommitted_diff()

```rust
pub fn open_uncommitted_diff(&self, buffer: Entity<Buffer>, cx: &Context<'_, Self>) -> Task<Result<Entity<BufferDiff>>>
```

Defined in: [`packages/project/src/project/git.rs:102`](../../../../packages/project/src/project/git.rs#L102)

#### Parameters

##### buffer

`Entity<Buffer>`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<Entity<BufferDiff>>>`


***

### git_init()

```rust
pub fn git_init(&self, path: Arc<Path>, fallback_branch_name: String, cx: &App) -> Task<Result<()>>
```

Defined in: [`packages/project/src/project/git.rs:114`](../../../../packages/project/src/project/git.rs#L114)

#### Parameters

##### path

`Arc<Path>`

##### fallback_branch_name

`String`

##### cx

`&App`

#### Returns

`Task<Result<()>>`


***

### git_config()

```rust
pub fn git_config(&self, path: Arc<Path>, args: Vec<String>, cx: &App) -> Task<Result<String>>
```

Defined in: [`packages/project/src/project/git.rs:124`](../../../../packages/project/src/project/git.rs#L124)

#### Parameters

##### path

`Arc<Path>`

##### args

`Vec<String>`

##### cx

`&App`

#### Returns

`Task<Result<String>>`


***

### project_path_git_status()

```rust
pub fn project_path_git_status(&self, project_path: &ProjectPath, cx: &App) -> Option<FileStatus>
```

Defined in: [`packages/project/src/project/git.rs:128`](../../../../packages/project/src/project/git.rs#L128)

#### Parameters

##### project_path

`&ProjectPath`

##### cx

`&App`

#### Returns

`Option<FileStatus>`


***

### open_unstaged_diff()

```rust
pub fn open_unstaged_diff(&self, buffer: Entity<Buffer>, cx: &Context<'_, Self>) -> Task<Result<Entity<BufferDiff>>>
```

Defined in: [`packages/project/src/project/git.rs:140`](../../../../packages/project/src/project/git.rs#L140)

Opens the unstaged (index-vs-worktree) diff for the given buffer, along
with the worktree text buffer that is the diff's main buffer.

#### Parameters

##### buffer

`Entity<Buffer>`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<Entity<BufferDiff>>>`


***

### open_staged_diff()

```rust
pub fn open_staged_diff(&self, buffer: Entity<Buffer>, cx: &Context<'_, Self>) -> Task<Result<(Entity<BufferDiff>, Entity<Buffer>)>>
```

Defined in: [`packages/project/src/project/git.rs:155`](../../../../packages/project/src/project/git.rs#L155)

Opens the staged (HEAD-vs-index) diff for the given buffer, along with
the index text buffer that is the diff's main buffer.

#### Parameters

##### buffer

`Entity<Buffer>`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<(Entity<BufferDiff>, Entity<Buffer>)>>`


***

### project_group_key()

```rust
pub fn project_group_key(&self, cx: &App) -> ProjectGroupKey
```

Defined in: [`packages/project/src/project/group_key.rs:100`](../../../../packages/project/src/project/group_key.rs#L100)

#### Parameters

##### cx

`&App`

#### Returns

[`ProjectGroupKey`](ProjectGroupKey.md)


***

### open_image()

```rust
pub fn open_image<impl Into<ProjectPath>: Into>(&self, path: impl ?, cx: &Context<'_, Self>) -> Task<Result<Entity<ImageItem>>>
```

Defined in: [`packages/project/src/project/images.rs:13`](../../../../packages/project/src/project/images.rs#L13)

#### Parameters

##### path

`impl ?`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<Entity<ImageItem>>>`


***

### reload_images()

```rust
pub fn reload_images(&self, images: HashSet<Entity<ImageItem>>, cx: &Context<'_, Self>) -> Task<Result<()>>
```

Defined in: [`packages/project/src/project/images.rs:47`](../../../../packages/project/src/project/images.rs#L47)

#### Parameters

##### images

`HashSet<Entity<ImageItem>>`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<()>>`


***

### init()

```rust
pub fn init(client: &Arc<Client>, cx: &App)
```

Defined in: [`packages/project/src/project/init.rs:18`](../../../../packages/project/src/project/init.rs#L18)

#### Parameters

##### client

`&Arc<Client>`

##### cx

`&App`


***

### shared()

```rust
pub fn shared(&self, project_id: u64, cx: &Context<'_, Self>) -> Result<()>
```

Defined in: [`packages/project/src/project/lifecycle.rs:45`](../../../../packages/project/src/project/lifecycle.rs#L45)

#### Parameters

##### project_id

`u64`

##### cx

`&Context<'_, Self>`

#### Returns

`Result<()>`


***

### reshared()

```rust
pub fn reshared(&self, message: ResharedProject, cx: &Context<'_, Self>) -> Result<()>
```

Defined in: [`packages/project/src/project/lifecycle.rs:110`](../../../../packages/project/src/project/lifecycle.rs#L110)

#### Parameters

##### message

`ResharedProject`

##### cx

`&Context<'_, Self>`

#### Returns

`Result<()>`


***

### rejoined()

```rust
pub fn rejoined(&self, message: RejoinedProject, message_id: u32, cx: &Context<'_, Self>) -> Result<()>
```

Defined in: [`packages/project/src/project/lifecycle.rs:130`](../../../../packages/project/src/project/lifecycle.rs#L130)

#### Parameters

##### message

`RejoinedProject`

##### message_id

`u32`

##### cx

`&Context<'_, Self>`

#### Returns

`Result<()>`


***

### unshare()

```rust
pub fn unshare(&self, cx: &Context<'_, Self>) -> Result<()>
```

Defined in: [`packages/project/src/project/lifecycle.rs:164`](../../../../packages/project/src/project/lifecycle.rs#L164)

#### Parameters

##### cx

`&Context<'_, Self>`

#### Returns

`Result<()>`


***

### disconnected_from_host()

```rust
pub fn disconnected_from_host(&self, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/project/lifecycle.rs:212`](../../../../packages/project/src/project/lifecycle.rs#L212)

#### Parameters

##### cx

`&Context<'_, Self>`


***

### set_role()

```rust
pub fn set_role(&self, role: ChannelRole, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/project/lifecycle.rs:240`](../../../../packages/project/src/project/lifecycle.rs#L240)

#### Parameters

##### role

`ChannelRole`

##### cx

`&Context<'_, Self>`


***

### close()

```rust
pub fn close(&self, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/project/lifecycle.rs:258`](../../../../packages/project/src/project/lifecycle.rs#L258)

#### Parameters

##### cx

`&Context<'_, Self>`


***

### is_disconnected()

```rust
pub fn is_disconnected(&self, cx: &App) -> bool
```

Defined in: [`packages/project/src/project/lifecycle.rs:261`](../../../../packages/project/src/project/lifecycle.rs#L261)

#### Parameters

##### cx

`&App`

#### Returns

`bool`


***

### capability()

```rust
pub fn capability(&self) -> Capability
```

Defined in: [`packages/project/src/project/lifecycle.rs:283`](../../../../packages/project/src/project/lifecycle.rs#L283)

#### Returns

`Capability`


***

### is_read_only()

```rust
pub fn is_read_only(&self, cx: &App) -> bool
```

Defined in: [`packages/project/src/project/lifecycle.rs:291`](../../../../packages/project/src/project/lifecycle.rs#L291)

#### Parameters

##### cx

`&App`

#### Returns

`bool`


***

### is_local()

```rust
pub fn is_local(&self) -> bool
```

Defined in: [`packages/project/src/project/lifecycle.rs:296`](../../../../packages/project/src/project/lifecycle.rs#L296)

#### Returns

`bool`


***

### is_via_remote_server()

```rust
pub fn is_via_remote_server(&self) -> bool
```

Defined in: [`packages/project/src/project/lifecycle.rs:304`](../../../../packages/project/src/project/lifecycle.rs#L304)

#### Returns

`bool`


***

### is_via_collab()

```rust
pub fn is_via_collab(&self) -> bool
```

Defined in: [`packages/project/src/project/lifecycle.rs:315`](../../../../packages/project/src/project/lifecycle.rs#L315)

Whether this project is from collab (not counting remote servers).

#### Returns

`bool`


***

### is_remote()

```rust
pub fn is_remote(&self) -> bool
```

Defined in: [`packages/project/src/project/lifecycle.rs:324`](../../../../packages/project/src/project/lifecycle.rs#L324)

`!self.is_local()`

#### Returns

`bool`


***

### is_via_wsl_with_host_interop()

```rust
pub fn is_via_wsl_with_host_interop(&self, cx: &App) -> bool
```

Defined in: [`packages/project/src/project/lifecycle.rs:333`](../../../../packages/project/src/project/lifecycle.rs#L333)

#### Parameters

##### cx

`&App`

#### Returns

`bool`


***

### is_via_wsl()

```rust
pub fn is_via_wsl(&self, cx: &App) -> bool
```

Defined in: [`packages/project/src/project/lifecycle.rs:347`](../../../../packages/project/src/project/lifecycle.rs#L347)

Whether this project is served by a WSL distribution.

#### Parameters

##### cx

`&App`

#### Returns

`bool`


***

### remote_id()

```rust
pub fn remote_id(&self) -> Option<u64>
```

Defined in: [`packages/project/src/project/lifecycle.rs:353`](../../../../packages/project/src/project/lifecycle.rs#L353)

#### Returns

`Option<u64>`


***

### replica_id()

```rust
pub fn replica_id(&self) -> ReplicaId
```

Defined in: [`packages/project/src/project/lifecycle.rs:362`](../../../../packages/project/src/project/lifecycle.rs#L362)

#### Returns

`ReplicaId`


***

### is_shared()

```rust
pub fn is_shared(&self) -> bool
```

Defined in: [`packages/project/src/project/lifecycle.rs:376`](../../../../packages/project/src/project/lifecycle.rs#L376)

#### Returns

`bool`


***

### supports_terminal()

```rust
pub fn supports_terminal(&self, _cx: &App) -> bool
```

Defined in: [`packages/project/src/project/lifecycle.rs:385`](../../../../packages/project/src/project/lifecycle.rs#L385)

Returns the resolved version of `path`, that was found in `buffer`, if it exists.

#### Parameters

##### _cx

`&App`

#### Returns

`bool`


***

### remote_connection_state()

```rust
pub fn remote_connection_state(&self, cx: &App) -> Option<ConnectionState>
```

Defined in: [`packages/project/src/project/lifecycle.rs:390`](../../../../packages/project/src/project/lifecycle.rs#L390)

#### Parameters

##### cx

`&App`

#### Returns

`Option<ConnectionState>`


***

### remote_connection_options()

```rust
pub fn remote_connection_options(&self, cx: &App) -> Option<RemoteConnectionOptions>
```

Defined in: [`packages/project/src/project/lifecycle.rs:397`](../../../../packages/project/src/project/lifecycle.rs#L397)

#### Parameters

##### cx

`&App`

#### Returns

`Option<RemoteConnectionOptions>`


***

### reveal_path()

```rust
pub fn reveal_path(&self, path: &Path, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/project/lifecycle.rs:407`](../../../../packages/project/src/project/lifecycle.rs#L407)

Reveals the given path in the system file manager.

On Windows with a WSL remote connection, this converts the POSIX path
to a Windows UNC path before revealing.

#### Parameters

##### path

`&Path`

##### cx

`&Context<'_, Self>`


***

### set_active_path()

```rust
pub fn set_active_path(&self, entry: Option<ProjectPath>, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/project/lifecycle.rs:425`](../../../../packages/project/src/project/lifecycle.rs#L425)

#### Parameters

##### entry

`Option<ProjectPath>`

##### cx

`&Context<'_, Self>`


***

### language_servers_running_disk_based_diagnostics()

```rust
pub fn language_servers_running_disk_based_diagnostics<'a>(&self, cx: &'a App) -> impl ? + ?
```

Defined in: [`packages/project/src/project/lifecycle.rs:440`](../../../../packages/project/src/project/lifecycle.rs#L440)

#### Parameters

##### cx

`&'a App`

#### Returns

`impl ? + ?`


***

### disable_worktree_scanner()

```rust
pub fn disable_worktree_scanner(&self, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/project/lifecycle.rs:448`](../../../../packages/project/src/project/lifecycle.rs#L448)

#### Parameters

##### cx

`&Context<'_, Self>`


***

### register_buffer_with_language_servers()

```rust
pub fn register_buffer_with_language_servers(&self, buffer: &Entity<Buffer>, cx: &App) -> OpenLspBufferHandle
```

Defined in: [`packages/project/src/project/lsp.rs:34`](../../../../packages/project/src/project/lsp.rs#L34)

#### Parameters

##### buffer

`&Entity<Buffer>`

##### cx

`&App`

#### Returns

[`OpenLspBufferHandle`](OpenLspBufferHandle.md)


***

### set_language_for_buffer()

```rust
pub fn set_language_for_buffer(&self, buffer: &Entity<Buffer>, new_language: Arc<Language>, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/project/lsp.rs:44`](../../../../packages/project/src/project/lsp.rs#L44)

#### Parameters

##### buffer

`&Entity<Buffer>`

##### new_language

`Arc<Language>`

##### cx

`&Context<'_, Self>`


***

### restart_language_servers_for_buffers()

```rust
pub fn restart_language_servers_for_buffers(&self, buffers: Vec<Entity<Buffer>>, only_restart_servers: HashSet<LanguageServerSelector>, clear_stopped: bool, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/project/lsp.rs:58`](../../../../packages/project/src/project/lsp.rs#L58)

#### Parameters

##### buffers

`Vec<Entity<Buffer>>`

##### only_restart_servers

`HashSet<LanguageServerSelector>`

##### clear_stopped

`bool`

##### cx

`&Context<'_, Self>`


***

### stop_language_servers_for_buffers()

```rust
pub fn stop_language_servers_for_buffers(&self, buffers: Vec<Entity<Buffer>>, also_restart_servers: HashSet<LanguageServerSelector>, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/project/lsp.rs:75`](../../../../packages/project/src/project/lsp.rs#L75)

#### Parameters

##### buffers

`Vec<Entity<Buffer>>`

##### also_restart_servers

`HashSet<LanguageServerSelector>`

##### cx

`&Context<'_, Self>`


***

### cancel_language_server_work_for_buffers()

```rust
pub fn cancel_language_server_work_for_buffers<impl IntoIterator<Item = Entity<Buffer>>: IntoIterator>(&self, buffers: impl ?, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/project/lsp.rs:88`](../../../../packages/project/src/project/lsp.rs#L88)

#### Parameters

##### buffers

`impl ?`

##### cx

`&Context<'_, Self>`


***

### cancel_language_server_work()

```rust
pub fn cancel_language_server_work(&self, server_id: LanguageServerId, token_to_cancel: Option<ProgressToken>, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/project/lsp.rs:98`](../../../../packages/project/src/project/lsp.rs#L98)

#### Parameters

##### server_id

`LanguageServerId`

##### token_to_cancel

`Option<ProgressToken>`

##### cx

`&Context<'_, Self>`


***

### language_server_statuses()

```rust
pub fn language_server_statuses<'a>(&self, cx: &'a App) -> impl ?
```

Defined in: [`packages/project/src/project/lsp.rs:109`](../../../../packages/project/src/project/lsp.rs#L109)

#### Parameters

##### cx

`&'a App`

#### Returns

`impl ?`


***

### last_formatting_failure()

```rust
pub fn last_formatting_failure<'a>(&self, cx: &'a App) -> Option<&'a str>
```

Defined in: [`packages/project/src/project/lsp.rs:116`](../../../../packages/project/src/project/lsp.rs#L116)

#### Parameters

##### cx

`&'a App`

#### Returns

`Option<&'a str>`


***

### reset_last_formatting_failure()

```rust
pub fn reset_last_formatting_failure(&self, cx: &App)
```

Defined in: [`packages/project/src/project/lsp.rs:120`](../../../../packages/project/src/project/lsp.rs#L120)

#### Parameters

##### cx

`&App`


***

### format()

```rust
pub fn format(&self, buffers: HashSet<Entity<Buffer>>, target: LspFormatTarget, push_to_history: bool, trigger: FormatTrigger, cx: &Context<'_, Project>) -> Task<Result<ProjectTransaction>>
```

Defined in: [`packages/project/src/project/lsp.rs:125`](../../../../packages/project/src/project/lsp.rs#L125)

#### Parameters

##### buffers

`HashSet<Entity<Buffer>>`

##### target

[`LspFormatTarget`](../enums/LspFormatTarget.md)

##### push_to_history

`bool`

##### trigger

[`FormatTrigger`](../enums/FormatTrigger.md)

##### cx

`&Context<'_, Project>`

#### Returns

`Task<Result<ProjectTransaction>>`


***

### supports_range_formatting()

```rust
pub fn supports_range_formatting(&self, buffer: &Entity<Buffer>, cx: &App) -> bool
```

Defined in: [`packages/project/src/project/lsp.rs:138`](../../../../packages/project/src/project/lsp.rs#L138)

#### Parameters

##### buffer

`&Entity<Buffer>`

##### cx

`&App`

#### Returns

`bool`


***

### definitions()

```rust
pub fn definitions<T: ToPointUtf16>(&self, buffer: &Entity<Buffer>, position: T, cx: &Context<'_, Self>) -> Task<Result<Option<Vec<LocationLink>>>>
```

Defined in: [`packages/project/src/project/lsp.rs:144`](../../../../packages/project/src/project/lsp.rs#L144)

#### Parameters

##### buffer

`&Entity<Buffer>`

##### position

`T`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<Option<Vec<LocationLink>>>>`


***

### edit_prediction_definitions()

```rust
pub fn edit_prediction_definitions<T: ToPointUtf16>(&self, buffer: &Entity<Buffer>, position: T, include_type_definitions: bool, cx: &Context<'_, Self>) -> Task<Result<Vec<EditPredictionDefinition>>>
```

Defined in: [`packages/project/src/project/lsp.rs:162`](../../../../packages/project/src/project/lsp.rs#L162)

#### Parameters

##### buffer

`&Entity<Buffer>`

##### position

`T`

##### include_type_definitions

`bool`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<Vec<EditPredictionDefinition>>>`


***

### declarations()

```rust
pub fn declarations<T: ToPointUtf16>(&self, buffer: &Entity<Buffer>, position: T, cx: &Context<'_, Self>) -> Task<Result<Option<Vec<LocationLink>>>>
```

Defined in: [`packages/project/src/project/lsp.rs:175`](../../../../packages/project/src/project/lsp.rs#L175)

#### Parameters

##### buffer

`&Entity<Buffer>`

##### position

`T`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<Option<Vec<LocationLink>>>>`


***

### type_definitions()

```rust
pub fn type_definitions<T: ToPointUtf16>(&self, buffer: &Entity<Buffer>, position: T, cx: &Context<'_, Self>) -> Task<Result<Option<Vec<LocationLink>>>>
```

Defined in: [`packages/project/src/project/lsp.rs:193`](../../../../packages/project/src/project/lsp.rs#L193)

#### Parameters

##### buffer

`&Entity<Buffer>`

##### position

`T`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<Option<Vec<LocationLink>>>>`


***

### implementations()

```rust
pub fn implementations<T: ToPointUtf16>(&self, buffer: &Entity<Buffer>, position: T, cx: &Context<'_, Self>) -> Task<Result<Option<Vec<LocationLink>>>>
```

Defined in: [`packages/project/src/project/lsp.rs:211`](../../../../packages/project/src/project/lsp.rs#L211)

#### Parameters

##### buffer

`&Entity<Buffer>`

##### position

`T`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<Option<Vec<LocationLink>>>>`


***

### references()

```rust
pub fn references<T: ToPointUtf16>(&self, buffer: &Entity<Buffer>, position: T, cx: &Context<'_, Self>) -> Task<Result<Option<Vec<Location>>>>
```

Defined in: [`packages/project/src/project/lsp.rs:229`](../../../../packages/project/src/project/lsp.rs#L229)

#### Parameters

##### buffer

`&Entity<Buffer>`

##### position

`T`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<Option<Vec<Location>>>>`


***

### prepare_call_hierarchy()

```rust
pub fn prepare_call_hierarchy<T: ToPointUtf16>(&self, buffer: &Entity<Buffer>, position: T, cx: &Context<'_, Self>) -> Task<Result<Option<Vec<CallHierarchyItem>>>>
```

Defined in: [`packages/project/src/project/lsp.rs:247`](../../../../packages/project/src/project/lsp.rs#L247)

#### Parameters

##### buffer

`&Entity<Buffer>`

##### position

`T`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<Option<Vec<CallHierarchyItem>>>>`


***

### incoming_calls()

```rust
pub fn incoming_calls(&self, item: CallHierarchyItem, cx: &Context<'_, Self>) -> Task<Result<Option<Vec<IncomingCall>>>>
```

Defined in: [`packages/project/src/project/lsp.rs:265`](../../../../packages/project/src/project/lsp.rs#L265)

#### Parameters

##### item

[`CallHierarchyItem`](CallHierarchyItem.md)

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<Option<Vec<IncomingCall>>>>`


***

### outgoing_calls()

```rust
pub fn outgoing_calls(&self, item: CallHierarchyItem, cx: &Context<'_, Self>) -> Task<Result<Option<Vec<OutgoingCall>>>>
```

Defined in: [`packages/project/src/project/lsp.rs:281`](../../../../packages/project/src/project/lsp.rs#L281)

#### Parameters

##### item

[`CallHierarchyItem`](CallHierarchyItem.md)

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<Option<Vec<OutgoingCall>>>>`


***

### document_highlights()

```rust
pub fn document_highlights<T: ToPointUtf16>(&self, buffer: &Entity<Buffer>, position: T, cx: &Context<'_, Self>) -> Task<Result<Vec<DocumentHighlight>>>
```

Defined in: [`packages/project/src/project/lsp.rs:297`](../../../../packages/project/src/project/lsp.rs#L297)

#### Parameters

##### buffer

`&Entity<Buffer>`

##### position

`T`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<Vec<DocumentHighlight>>>`


***

### document_symbols()

```rust
pub fn document_symbols(&self, buffer: &Entity<Buffer>, cx: &Context<'_, Self>) -> Task<Result<Vec<DocumentSymbol>>>
```

Defined in: [`packages/project/src/project/lsp.rs:312`](../../../../packages/project/src/project/lsp.rs#L312)

#### Parameters

##### buffer

`&Entity<Buffer>`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<Vec<DocumentSymbol>>>`


***

### symbols()

```rust
pub fn symbols(&self, query: &str, cx: &Context<'_, Self>) -> Task<Result<Vec<Symbol>>>
```

Defined in: [`packages/project/src/project/lsp.rs:325`](../../../../packages/project/src/project/lsp.rs#L325)

#### Parameters

##### query

`&str`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<Vec<Symbol>>>`


***

### open_buffer_for_symbol()

```rust
pub fn open_buffer_for_symbol(&self, symbol: &Symbol, cx: &Context<'_, Self>) -> Task<Result<Entity<Buffer>>>
```

Defined in: [`packages/project/src/project/lsp.rs:330`](../../../../packages/project/src/project/lsp.rs#L330)

#### Parameters

##### symbol

`&Symbol`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<Entity<Buffer>>>`


***

### open_server_settings()

```rust
pub fn open_server_settings(&self, cx: &Context<'_, Self>) -> Task<Result<Entity<Buffer>>>
```

Defined in: [`packages/project/src/project/lsp.rs:340`](../../../../packages/project/src/project/lsp.rs#L340)

#### Parameters

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<Entity<Buffer>>>`


***

### open_local_buffer_via_lsp()

```rust
pub fn open_local_buffer_via_lsp(&self, abs_path: Uri, language_server_id: LanguageServerId, cx: &Context<'_, Self>) -> Task<Result<Entity<Buffer>>>
```

Defined in: [`packages/project/src/project/lsp.rs:371`](../../../../packages/project/src/project/lsp.rs#L371)

#### Parameters

##### abs_path

`Uri`

##### language_server_id

`LanguageServerId`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<Entity<Buffer>>>`


***

### hover()

```rust
pub fn hover<T: ToPointUtf16>(&self, buffer: &Entity<Buffer>, position: T, cx: &Context<'_, Self>) -> Task<Option<Vec<Hover>>>
```

Defined in: [`packages/project/src/project/lsp.rs:382`](../../../../packages/project/src/project/lsp.rs#L382)

#### Parameters

##### buffer

`&Entity<Buffer>`

##### position

`T`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Option<Vec<Hover>>>`


***

### linked_edits()

```rust
pub fn linked_edits(&self, buffer: &Entity<Buffer>, position: Anchor, cx: &Context<'_, Self>) -> Task<Result<Vec<Range<Anchor>>>>
```

Defined in: [`packages/project/src/project/lsp.rs:393`](../../../../packages/project/src/project/lsp.rs#L393)

#### Parameters

##### buffer

`&Entity<Buffer>`

##### position

`Anchor`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<Vec<Range<Anchor>>>>`


***

### completions()

```rust
pub fn completions<T: ToOffset + ToPointUtf16>(&self, buffer: &Entity<Buffer>, position: T, context: CompletionContext, cx: &Context<'_, Self>) -> Task<Result<Vec<CompletionResponse>>>
```

Defined in: [`packages/project/src/project/lsp.rs:404`](../../../../packages/project/src/project/lsp.rs#L404)

#### Parameters

##### buffer

`&Entity<Buffer>`

##### position

`T`

##### context

`CompletionContext`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<Vec<CompletionResponse>>>`


***

### code_actions()

```rust
pub fn code_actions<T: Clone + ToOffset>(&self, buffer_handle: &Entity<Buffer>, range: Range<T>, kinds: Option<Vec<CodeActionKind>>, cx: &Context<'_, Self>) -> Task<Result<Option<Vec<CodeAction>>>>
```

Defined in: [`packages/project/src/project/lsp.rs:417`](../../../../packages/project/src/project/lsp.rs#L417)

#### Parameters

##### buffer_handle

`&Entity<Buffer>`

##### range

`Range<T>`

##### kinds

`Option<Vec<CodeActionKind>>`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<Option<Vec<CodeAction>>>>`


***

### apply_code_action()

```rust
pub fn apply_code_action(&self, buffer_handle: Entity<Buffer>, action: CodeAction, push_to_history: bool, cx: &Context<'_, Self>) -> Task<Result<ProjectTransaction>>
```

Defined in: [`packages/project/src/project/lsp.rs:431`](../../../../packages/project/src/project/lsp.rs#L431)

#### Parameters

##### buffer_handle

`Entity<Buffer>`

##### action

[`CodeAction`](CodeAction.md)

##### push_to_history

`bool`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<ProjectTransaction>>`


***

### apply_code_action_kind()

```rust
pub fn apply_code_action_kind(&self, buffers: HashSet<Entity<Buffer>>, kind: CodeActionKind, push_to_history: bool, cx: &Context<'_, Self>) -> Task<Result<ProjectTransaction>>
```

Defined in: [`packages/project/src/project/lsp.rs:443`](../../../../packages/project/src/project/lsp.rs#L443)

#### Parameters

##### buffers

`HashSet<Entity<Buffer>>`

##### kind

`CodeActionKind`

##### push_to_history

`bool`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<ProjectTransaction>>`


***

### prepare_rename()

```rust
pub fn prepare_rename<T: ToPointUtf16>(&self, buffer: Entity<Buffer>, position: T, cx: &Context<'_, Self>) -> Task<Result<PrepareRenameResponse>>
```

Defined in: [`packages/project/src/project/lsp.rs:455`](../../../../packages/project/src/project/lsp.rs#L455)

#### Parameters

##### buffer

`Entity<Buffer>`

##### position

`T`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<PrepareRenameResponse>>`


***

### perform_rename()

```rust
pub fn perform_rename<T: ToPointUtf16>(&self, buffer: Entity<Buffer>, position: T, new_name: String, language_server_id: Option<LanguageServerId>, cx: &Context<'_, Self>) -> Task<Result<ProjectTransaction>>
```

Defined in: [`packages/project/src/project/lsp.rs:470`](../../../../packages/project/src/project/lsp.rs#L470)

#### Parameters

##### buffer

`Entity<Buffer>`

##### position

`T`

##### new_name

`String`

##### language_server_id

`Option<LanguageServerId>`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<ProjectTransaction>>`


***

### on_type_format()

```rust
pub fn on_type_format<T: ToPointUtf16>(&self, buffer: Entity<Buffer>, position: T, trigger: String, push_to_history: bool, cx: &Context<'_, Self>) -> Option<Task<Result<Option<Transaction>>>>
```

Defined in: [`packages/project/src/project/lsp.rs:503`](../../../../packages/project/src/project/lsp.rs#L503)

#### Parameters

##### buffer

`Entity<Buffer>`

##### position

`T`

##### trigger

`String`

##### push_to_history

`bool`

##### cx

`&Context<'_, Self>`

#### Returns

`Option<Task<Result<Option<Transaction>>>>`


***

### inline_values()

```rust
pub fn inline_values(&self, session: Entity<Session>, active_stack_frame: ActiveStackFrame, buffer_handle: Entity<Buffer>, range: Range<Anchor>, cx: &Context<'_, Self>) -> Task<Result<Vec<InlayHint>>>
```

Defined in: [`packages/project/src/project/lsp.rs:516`](../../../../packages/project/src/project/lsp.rs#L516)

#### Parameters

##### session

`Entity<Session>`

##### active_stack_frame

[`ActiveStackFrame`](ActiveStackFrame.md)

##### buffer_handle

`Entity<Buffer>`

##### range

`Range<Anchor>`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<Vec<InlayHint>>>`


***

### request_lsp()

```rust
pub fn request_lsp<R: [LspCommand](../traits/LspCommand.md)>(&self, buffer_handle: Entity<Buffer>, server: LanguageServerToQuery, request: R, cx: &Context<'_, Self>) -> Task<Result<Response>>
where
    Result: Send,
    Params: Send
```

Defined in: [`packages/project/src/project/lsp.rs:550`](../../../../packages/project/src/project/lsp.rs#L550)

#### Parameters

##### buffer_handle

`Entity<Buffer>`

##### server

[`LanguageServerToQuery`](../enums/LanguageServerToQuery.md)

##### request

`R`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<Response>>`


***

### any_language_server_supports_inlay_hints()

```rust
pub fn any_language_server_supports_inlay_hints(&self, buffer: &Buffer, cx: &App) -> bool
```

Defined in: [`packages/project/src/project/lsp.rs:571`](../../../../packages/project/src/project/lsp.rs#L571)

#### Parameters

##### buffer

`&Buffer`

##### cx

`&App`

#### Returns

`bool`


***

### any_language_server_supports_semantic_tokens()

```rust
pub fn any_language_server_supports_semantic_tokens(&self, buffer: &Buffer, cx: &App) -> bool
```

Defined in: [`packages/project/src/project/lsp.rs:594`](../../../../packages/project/src/project/lsp.rs#L594)

#### Parameters

##### buffer

`&Buffer`

##### cx

`&App`

#### Returns

`bool`


***

### language_server_id_for_name()

```rust
pub fn language_server_id_for_name(&self, buffer: &Buffer, name: &LanguageServerName, cx: &App) -> Option<LanguageServerId>
```

Defined in: [`packages/project/src/project/lsp.rs:620`](../../../../packages/project/src/project/lsp.rs#L620)

#### Parameters

##### buffer

`&Buffer`

##### name

`&LanguageServerName`

##### cx

`&App`

#### Returns

`Option<LanguageServerId>`


***

### find_project_path()

```rust
pub fn find_project_path<impl AsRef<Path>: AsRef>(&self, path: impl ?, cx: &App) -> Option<ProjectPath>
```

Defined in: [`packages/project/src/project/paths.rs:26`](../../../../packages/project/src/project/paths.rs#L26)

#### Parameters

##### path

`impl ?`

##### cx

`&App`

#### Returns

`Option<ProjectPath>`


***

### short_full_path_for_project_path()

```rust
pub fn short_full_path_for_project_path(&self, project_path: &ProjectPath, cx: &App) -> Option<String>
```

Defined in: [`packages/project/src/project/paths.rs:91`](../../../../packages/project/src/project/paths.rs#L91)

If there's only one visible worktree, returns the given worktree-relative path with no prefix.

Otherwise, returns the full path for the project path (obtained by prefixing the worktree-relative path with the name of the worktree).

#### Parameters

##### project_path

`&ProjectPath`

##### cx

`&App`

#### Returns

`Option<String>`


***

### project_path_for_absolute_path()

```rust
pub fn project_path_for_absolute_path(&self, abs_path: &Path, cx: &App) -> Option<ProjectPath>
```

Defined in: [`packages/project/src/project/paths.rs:109`](../../../../packages/project/src/project/paths.rs#L109)

#### Parameters

##### abs_path

`&Path`

##### cx

`&App`

#### Returns

`Option<ProjectPath>`


***

### get_workspace_root()

```rust
pub fn get_workspace_root(&self, project_path: &ProjectPath, cx: &App) -> Option<PathBuf>
```

Defined in: [`packages/project/src/project/paths.rs:114`](../../../../packages/project/src/project/paths.rs#L114)

#### Parameters

##### project_path

`&ProjectPath`

##### cx

`&App`

#### Returns

`Option<PathBuf>`


***

### visibility_for_paths()

```rust
pub fn visibility_for_paths(&self, paths: &[PathBuf], exclude_sub_dirs: bool, cx: &App) -> Option<bool>
```

Defined in: [`packages/project/src/project/paths.rs:122`](../../../../packages/project/src/project/paths.rs#L122)

#### Parameters

##### paths

`&[PathBuf]`

##### exclude_sub_dirs

`bool`

##### cx

`&App`

#### Returns

`Option<bool>`


***

### visibility_for_path()

```rust
pub fn visibility_for_path(&self, path: &Path, exclude_sub_dirs: bool, cx: &App) -> Option<bool>
```

Defined in: [`packages/project/src/project/paths.rs:134`](../../../../packages/project/src/project/paths.rs#L134)

#### Parameters

##### path

`&Path`

##### exclude_sub_dirs

`bool`

##### cx

`&App`

#### Returns

`Option<bool>`


***

### visibility_for_subpaths()

```rust
pub fn visibility_for_subpaths(&self, paths: &[PathBuf], cx: &App) -> Option<bool>
```

Defined in: [`packages/project/src/project/paths.rs:164`](../../../../packages/project/src/project/paths.rs#L164)

#### Parameters

##### paths

`&[PathBuf]`

##### cx

`&App`

#### Returns

`Option<bool>`


***

### resolve_path_in_buffer()

```rust
pub fn resolve_path_in_buffer(&self, path: &str, buffer: &Entity<Buffer>, cx: &Context<'_, Self>) -> Task<Option<ResolvedPath>>
```

Defined in: [`packages/project/src/project/paths.rs:185`](../../../../packages/project/src/project/paths.rs#L185)

#### Parameters

##### path

`&str`

##### buffer

`&Entity<Buffer>`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Option<ResolvedPath>>`


***

### resolve_abs_file_path()

```rust
pub fn resolve_abs_file_path(&self, path: &str, cx: &Context<'_, Self>) -> Task<Option<ResolvedPath>>
```

Defined in: [`packages/project/src/project/paths.rs:197`](../../../../packages/project/src/project/paths.rs#L197)

#### Parameters

##### path

`&str`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Option<ResolvedPath>>`


***

### resolve_abs_path()

```rust
pub fn resolve_abs_path(&self, path: &str, cx: &App) -> Task<Option<ResolvedPath>>
```

Defined in: [`packages/project/src/project/paths.rs:208`](../../../../packages/project/src/project/paths.rs#L208)

#### Parameters

##### path

`&str`

##### cx

`&App`

#### Returns

`Task<Option<ResolvedPath>>`


***

### try_windows_path_to_wsl()

```rust
pub fn try_windows_path_to_wsl(&self, abs_path: &Path, cx: &App) -> impl ? + ?
```

Defined in: [`packages/project/src/project/paths.rs:318`](../../../../packages/project/src/project/paths.rs#L318)

#### Parameters

##### abs_path

`&Path`

##### cx

`&App`

#### Returns

`impl ? + ?`


***

### list_directory()

```rust
pub fn list_directory(&self, query: String, cx: &Context<'_, Self>) -> Task<Result<Vec<DirectoryItem>>>
```

Defined in: [`packages/project/src/project/paths.rs:341`](../../../../packages/project/src/project/paths.rs#L341)

#### Parameters

##### query

`String`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<Vec<DirectoryItem>>>`


***

### search_history()

```rust
pub fn search_history(&self, kind: SearchInputKind) -> &SearchHistory
```

Defined in: [`packages/project/src/project/search.rs:33`](../../../../packages/project/src/project/search.rs#L33)

#### Parameters

##### kind

[`SearchInputKind`](../enums/SearchInputKind.md)

#### Returns

`&SearchHistory`


***

### search_history_mut()

```rust
pub fn search_history_mut(&self, kind: SearchInputKind) -> &SearchHistory
```

Defined in: [`packages/project/src/project/search.rs:41`](../../../../packages/project/src/project/search.rs#L41)

#### Parameters

##### kind

[`SearchInputKind`](../enums/SearchInputKind.md)

#### Returns

`&SearchHistory`


***

### search()

```rust
pub fn search(&self, query: SearchQuery, cx: &Context<'_, Self>) -> SearchResults<SearchResult>
```

Defined in: [`packages/project/src/project/search.rs:88`](../../../../packages/project/src/project/search.rs#L88)

#### Parameters

##### query

[`SearchQuery`](../enums/SearchQuery.md)

##### cx

`&Context<'_, Self>`

#### Returns

[`SearchResults<SearchResult>`](SearchResults.md)


***

### available_toolchains()

```rust
pub fn available_toolchains(&self, path: ProjectPath, language_name: LanguageName, cx: &App) -> Task<Option<Toolchains>>
```

Defined in: [`packages/project/src/project/toolchains.rs:17`](../../../../packages/project/src/project/toolchains.rs#L17)

#### Parameters

##### path

[`ProjectPath`](ProjectPath.md)

##### language_name

`LanguageName`

##### cx

`&App`

#### Returns

`Task<Option<Toolchains>>`


***

### toolchain_metadata()

```rust
pub async fn toolchain_metadata(languages: Arc<LanguageRegistry>, language_name: LanguageName) -> Option<ToolchainMetadata>
```

Defined in: [`packages/project/src/project/toolchains.rs:34`](../../../../packages/project/src/project/toolchains.rs#L34)

#### Parameters

##### languages

`Arc<LanguageRegistry>`

##### language_name

`LanguageName`

#### Returns

`Option<ToolchainMetadata>`


***

### add_toolchain()

```rust
pub fn add_toolchain(&self, toolchain: Toolchain, scope: ToolchainScope, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/project/toolchains.rs:45`](../../../../packages/project/src/project/toolchains.rs#L45)

#### Parameters

##### toolchain

`Toolchain`

##### scope

`ToolchainScope`

##### cx

`&Context<'_, Self>`


***

### remove_toolchain()

```rust
pub fn remove_toolchain(&self, toolchain: Toolchain, scope: ToolchainScope, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/project/toolchains.rs:58`](../../../../packages/project/src/project/toolchains.rs#L58)

#### Parameters

##### toolchain

`Toolchain`

##### scope

`ToolchainScope`

##### cx

`&Context<'_, Self>`


***

### user_toolchains()

```rust
pub fn user_toolchains(&self, cx: &App) -> Option<BTreeMap<ToolchainScope, IndexSet<Toolchain>>>
```

Defined in: [`packages/project/src/project/toolchains.rs:71`](../../../../packages/project/src/project/toolchains.rs#L71)

#### Parameters

##### cx

`&App`

#### Returns

`Option<BTreeMap<ToolchainScope, IndexSet<Toolchain>>>`


***

### resolve_toolchain()

```rust
pub fn resolve_toolchain(&self, path: PathBuf, language_name: LanguageName, cx: &App) -> Task<Result<Toolchain>>
```

Defined in: [`packages/project/src/project/toolchains.rs:77`](../../../../packages/project/src/project/toolchains.rs#L77)

#### Parameters

##### path

`PathBuf`

##### language_name

`LanguageName`

##### cx

`&App`

#### Returns

`Task<Result<Toolchain>>`


***

### toolchain_store()

```rust
pub fn toolchain_store(&self) -> Option<Entity<ToolchainStore>>
```

Defined in: [`packages/project/src/project/toolchains.rs:95`](../../../../packages/project/src/project/toolchains.rs#L95)

#### Returns

`Option<Entity<ToolchainStore>>`


***

### activate_toolchain()

```rust
pub fn activate_toolchain(&self, path: ProjectPath, toolchain: Toolchain, cx: &App) -> Task<Option<()>>
```

Defined in: [`packages/project/src/project/toolchains.rs:96`](../../../../packages/project/src/project/toolchains.rs#L96)

#### Parameters

##### path

[`ProjectPath`](ProjectPath.md)

##### toolchain

`Toolchain`

##### cx

`&App`

#### Returns

`Task<Option<()>>`


***

### active_toolchain()

```rust
pub fn active_toolchain(&self, path: ProjectPath, language_name: LanguageName, cx: &App) -> Task<Option<Toolchain>>
```

Defined in: [`packages/project/src/project/toolchains.rs:107`](../../../../packages/project/src/project/toolchains.rs#L107)

#### Parameters

##### path

[`ProjectPath`](ProjectPath.md)

##### language_name

`LanguageName`

##### cx

`&App`

#### Returns

`Task<Option<Toolchain>>`


***

### worktrees()

```rust
pub fn worktrees<'a>(&self, cx: &'a App) -> impl ? + ?
```

Defined in: [`packages/project/src/project/worktrees.rs:19`](../../../../packages/project/src/project/worktrees.rs#L19)

All worktrees in this project.

#### Parameters

##### cx

`&'a App`

#### Returns

`impl ? + ?`


***

### visible_worktrees()

```rust
pub fn visible_worktrees<'a>(&self, cx: &'a App) -> impl ? + ?
```

Defined in: [`packages/project/src/project/worktrees.rs:28`](../../../../packages/project/src/project/worktrees.rs#L28)

Collect all user-visible worktrees, the ones that appear in the project panel.

#### Parameters

##### cx

`&'a App`

#### Returns

`impl ? + ?`


***

### default_path_list()

```rust
pub fn default_path_list(&self, cx: &App) -> PathList
```

Defined in: [`packages/project/src/project/worktrees.rs:56`](../../../../packages/project/src/project/worktrees.rs#L56)

#### Parameters

##### cx

`&App`

#### Returns

`PathList`


***

### worktree_for_root_name()

```rust
pub fn worktree_for_root_name(&self, root_name: &str, cx: &App) -> Option<Entity<Worktree>>
```

Defined in: [`packages/project/src/project/worktrees.rs:68`](../../../../packages/project/src/project/worktrees.rs#L68)

#### Parameters

##### root_name

`&str`

##### cx

`&App`

#### Returns

`Option<Entity<Worktree>>`


***

### worktree_root_names()

```rust
pub fn worktree_root_names<'a>(&self, cx: &'a App) -> impl ?
```

Defined in: [`packages/project/src/project/worktrees.rs:72`](../../../../packages/project/src/project/worktrees.rs#L72)

#### Parameters

##### cx

`&'a App`

#### Returns

`impl ?`


***

### worktree_for_id()

```rust
pub fn worktree_for_id(&self, id: WorktreeId, cx: &App) -> Option<Entity<Worktree>>
```

Defined in: [`packages/project/src/project/worktrees.rs:78`](../../../../packages/project/src/project/worktrees.rs#L78)

#### Parameters

##### id

`WorktreeId`

##### cx

`&App`

#### Returns

`Option<Entity<Worktree>>`


***

### worktree_for_entry()

```rust
pub fn worktree_for_entry(&self, entry_id: ProjectEntryId, cx: &App) -> Option<Entity<Worktree>>
```

Defined in: [`packages/project/src/project/worktrees.rs:81`](../../../../packages/project/src/project/worktrees.rs#L81)

#### Parameters

##### entry_id

`ProjectEntryId`

##### cx

`&App`

#### Returns

`Option<Entity<Worktree>>`


***

### worktree_id_for_entry()

```rust
pub fn worktree_id_for_entry(&self, entry_id: ProjectEntryId, cx: &App) -> Option<WorktreeId>
```

Defined in: [`packages/project/src/project/worktrees.rs:92`](../../../../packages/project/src/project/worktrees.rs#L92)

#### Parameters

##### entry_id

`ProjectEntryId`

##### cx

`&App`

#### Returns

`Option<WorktreeId>`


***

### entry_is_worktree_root()

```rust
pub fn entry_is_worktree_root(&self, entry_id: ProjectEntryId, cx: &App) -> bool
```

Defined in: [`packages/project/src/project/worktrees.rs:99`](../../../../packages/project/src/project/worktrees.rs#L99)

Checks if the entry is the root of a worktree.

#### Parameters

##### entry_id

`ProjectEntryId`

##### cx

`&App`

#### Returns

`bool`


***

### find_or_create_worktree()

```rust
pub fn find_or_create_worktree<impl AsRef<Path>: AsRef>(&self, abs_path: impl ?, visible: bool, cx: &Context<'_, Self>) -> Task<Result<(Entity<Worktree>, Arc<RelPath>)>>
```

Defined in: [`packages/project/src/project/worktrees.rs:111`](../../../../packages/project/src/project/worktrees.rs#L111)

#### Parameters

##### abs_path

`impl ?`

##### visible

`bool`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<(Entity<Worktree>, Arc<RelPath>)>>`


***

### find_worktree()

```rust
pub fn find_worktree(&self, abs_path: &Path, cx: &App) -> Option<(Entity<Worktree>, Arc<RelPath>)>
```

Defined in: [`packages/project/src/project/worktrees.rs:121`](../../../../packages/project/src/project/worktrees.rs#L121)

#### Parameters

##### abs_path

`&Path`

##### cx

`&App`

#### Returns

`Option<(Entity<Worktree>, Arc<RelPath>)>`


***

### create_worktree()

```rust
pub fn create_worktree<impl AsRef<Path>: AsRef>(&self, abs_path: impl ?, visible: bool, cx: &Context<'_, Self>) -> Task<Result<Entity<Worktree>>>
```

Defined in: [`packages/project/src/project/worktrees.rs:128`](../../../../packages/project/src/project/worktrees.rs#L128)

#### Parameters

##### abs_path

`impl ?`

##### visible

`bool`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<Entity<Worktree>>>`


***

### wait_for_worktree_release()

```rust
pub fn wait_for_worktree_release(&self, worktree_id: WorktreeId, cx: &Context<'_, Self>) -> Task<Result<()>>
```

Defined in: [`packages/project/src/project/worktrees.rs:150`](../../../../packages/project/src/project/worktrees.rs#L150)

Returns a task that resolves when the given worktree's `Entity` is
fully dropped (all strong references released), not merely when
`remove_worktree` is called. `remove_worktree` drops the store's
reference and emits `WorktreeRemoved`, but other code may still
hold a strong handle — the worktree isn't safe to delete from
disk until every handle is gone.

We use `observe_release` on the specific entity rather than
listening for `WorktreeReleased` events because it's simpler at
the call site (one awaitable task, no subscription / channel /
ID filtering).

#### Parameters

##### worktree_id

`WorktreeId`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<()>>`


***

### remove_worktree()

```rust
pub fn remove_worktree(&self, id_to_remove: WorktreeId, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/project/worktrees.rs:176`](../../../../packages/project/src/project/worktrees.rs#L176)

#### Parameters

##### id_to_remove

`WorktreeId`

##### cx

`&Context<'_, Self>`


***

### remove_worktree_for_main_worktree_path()

```rust
pub fn remove_worktree_for_main_worktree_path<impl AsRef<Path>: AsRef>(&self, path: impl ?, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/project/worktrees.rs:181`](../../../../packages/project/src/project/worktrees.rs#L181)

#### Parameters

##### path

`impl ?`

##### cx

`&Context<'_, Self>`


***

### move_worktree()

```rust
pub fn move_worktree(&self, source: WorktreeId, destination: WorktreeId, cx: &Context<'_, Self>) -> Result<()>
```

Defined in: [`packages/project/src/project/worktrees.rs:193`](../../../../packages/project/src/project/worktrees.rs#L193)

#### Parameters

##### source

`WorktreeId`

##### destination

`WorktreeId`

##### cx

`&Context<'_, Self>`

#### Returns

`Result<()>`


***

### worktree_metadata_protos()

```rust
pub fn worktree_metadata_protos(&self, cx: &App) -> Vec<WorktreeMetadata>
```

Defined in: [`packages/project/src/project/worktrees.rs:210`](../../../../packages/project/src/project/worktrees.rs#L210)

#### Parameters

##### cx

`&App`

#### Returns

`Vec<WorktreeMetadata>`


***

### worktree_paths()

```rust
pub fn worktree_paths(&self, cx: &App) -> WorktreePaths
```

Defined in: [`packages/project/src/project/worktrees.rs:214`](../../../../packages/project/src/project/worktrees.rs#L214)

#### Parameters

##### cx

`&App`

#### Returns

[`WorktreePaths`](WorktreePaths.md)


***

### path_style()

```rust
pub fn path_style(&self, cx: &App) -> PathStyle
```

Defined in: [`packages/project/src/project/worktrees.rs:217`](../../../../packages/project/src/project/worktrees.rs#L217)

#### Parameters

##### cx

`&App`

#### Returns

`PathStyle`


***

### contains_local_settings_file()

```rust
pub fn contains_local_settings_file(&self, worktree_id: WorktreeId, rel_path: &RelPath, cx: &App) -> bool
```

Defined in: [`packages/project/src/project/worktrees.rs:220`](../../../../packages/project/src/project/worktrees.rs#L220)

#### Parameters

##### worktree_id

`WorktreeId`

##### rel_path

`&RelPath`

##### cx

`&App`

#### Returns

`bool`


***

### wait_for_initial_scan()

```rust
pub fn wait_for_initial_scan(&self, cx: &App) -> impl ? + ?
```

Defined in: [`packages/project/src/project/worktrees.rs:302`](../../../../packages/project/src/project/worktrees.rs#L302)

Returns a future that resolves when all visible worktrees have completed
their initial scan.

#### Parameters

##### cx

`&App`

#### Returns

`impl ? + ?`


***

### create_entry()

```rust
pub fn create_entry<impl Into<ProjectPath>: Into>(&self, project_path: impl ?, is_directory: bool, cx: &Context<'_, Self>) -> Task<Result<CreatedEntry>>
```

Defined in: [`packages/project/src/project/worktrees.rs:314`](../../../../packages/project/src/project/worktrees.rs#L314)

#### Parameters

##### project_path

`impl ?`

##### is_directory

`bool`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<CreatedEntry>>`


***

### copy_entry()

```rust
pub fn copy_entry(&self, entry_id: ProjectEntryId, new_project_path: ProjectPath, cx: &Context<'_, Self>) -> Task<Result<Option<Entry>>>
```

Defined in: [`packages/project/src/project/worktrees.rs:332`](../../../../packages/project/src/project/worktrees.rs#L332)

#### Parameters

##### entry_id

`ProjectEntryId`

##### new_project_path

[`ProjectPath`](ProjectPath.md)

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<Option<Entry>>>`


***

### rename_entry()

```rust
pub fn rename_entry(&self, entry_id: ProjectEntryId, new_path: ProjectPath, cx: &Context<'_, Self>) -> Task<Result<CreatedEntry>>
```

Defined in: [`packages/project/src/project/worktrees.rs:347`](../../../../packages/project/src/project/worktrees.rs#L347)

Renames the project entry with given `entry_id`.

`new_path` is a relative path to worktree root.
If root entry is renamed then its new root name is used instead.

#### Parameters

##### entry_id

`ProjectEntryId`

##### new_path

[`ProjectPath`](ProjectPath.md)

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<CreatedEntry>>`


***

### trash_file()

```rust
pub fn trash_file(&self, path: ProjectPath, cx: &Context<'_, Self>) -> Option<Task<Result<TrashId>>>
```

Defined in: [`packages/project/src/project/worktrees.rs:416`](../../../../packages/project/src/project/worktrees.rs#L416)

#### Parameters

##### path

[`ProjectPath`](ProjectPath.md)

##### cx

`&Context<'_, Self>`

#### Returns

`Option<Task<Result<TrashId>>>`


***

### delete_file()

```rust
pub fn delete_file(&self, path: ProjectPath, cx: &Context<'_, Self>) -> Option<Task<Result<()>>>
```

Defined in: [`packages/project/src/project/worktrees.rs:426`](../../../../packages/project/src/project/worktrees.rs#L426)

#### Parameters

##### path

[`ProjectPath`](ProjectPath.md)

##### cx

`&Context<'_, Self>`

#### Returns

`Option<Task<Result<()>>>`


***

### trash_entry()

```rust
pub fn trash_entry(&self, entry_id: ProjectEntryId, cx: &Context<'_, Self>) -> Option<Task<Result<TrashId>>>
```

Defined in: [`packages/project/src/project/worktrees.rs:436`](../../../../packages/project/src/project/worktrees.rs#L436)

#### Parameters

##### entry_id

`ProjectEntryId`

##### cx

`&Context<'_, Self>`

#### Returns

`Option<Task<Result<TrashId>>>`


***

### delete_entry()

```rust
pub fn delete_entry(&self, entry_id: ProjectEntryId, cx: &Context<'_, Self>) -> Option<Task<Result<()>>>
```

Defined in: [`packages/project/src/project/worktrees.rs:447`](../../../../packages/project/src/project/worktrees.rs#L447)

#### Parameters

##### entry_id

`ProjectEntryId`

##### cx

`&Context<'_, Self>`

#### Returns

`Option<Task<Result<()>>>`


***

### restore_entry()

```rust
pub fn restore_entry(&self, worktree_id: WorktreeId, trash_id: TrashId, cx: &Context<'_, Self>) -> Task<Result<ProjectPath>>
```

Defined in: [`packages/project/src/project/worktrees.rs:458`](../../../../packages/project/src/project/worktrees.rs#L458)

#### Parameters

##### worktree_id

`WorktreeId`

##### trash_id

`TrashId`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<ProjectPath>>`


***

### expand_entry()

```rust
pub fn expand_entry(&self, worktree_id: WorktreeId, entry_id: ProjectEntryId, cx: &Context<'_, Self>) -> Option<Task<Result<()>>>
```

Defined in: [`packages/project/src/project/worktrees.rs:481`](../../../../packages/project/src/project/worktrees.rs#L481)

#### Parameters

##### worktree_id

`WorktreeId`

##### entry_id

`ProjectEntryId`

##### cx

`&Context<'_, Self>`

#### Returns

`Option<Task<Result<()>>>`


***

### expand_all_for_entry()

```rust
pub fn expand_all_for_entry(&self, worktree_id: WorktreeId, entry_id: ProjectEntryId, cx: &Context<'_, Self>) -> Option<Task<Result<()>>>
```

Defined in: [`packages/project/src/project/worktrees.rs:490`](../../../../packages/project/src/project/worktrees.rs#L490)

#### Parameters

##### worktree_id

`WorktreeId`

##### entry_id

`ProjectEntryId`

##### cx

`&Context<'_, Self>`

#### Returns

`Option<Task<Result<()>>>`


***

### entry_for_path()

```rust
pub fn entry_for_path<'a>(&self, path: &ProjectPath, cx: &'a App) -> Option<&'a Entry>
```

Defined in: [`packages/project/src/project/worktrees.rs:508`](../../../../packages/project/src/project/worktrees.rs#L508)

#### Parameters

##### path

`&ProjectPath`

##### cx

`&'a App`

#### Returns

`Option<&'a Entry>`


***

### path_for_entry()

```rust
pub fn path_for_entry(&self, entry_id: ProjectEntryId, cx: &App) -> Option<ProjectPath>
```

Defined in: [`packages/project/src/project/worktrees.rs:511`](../../../../packages/project/src/project/worktrees.rs#L511)

#### Parameters

##### entry_id

`ProjectEntryId`

##### cx

`&App`

#### Returns

`Option<ProjectPath>`


***

### absolute_path()

```rust
pub fn absolute_path(&self, project_path: &ProjectPath, cx: &App) -> Option<PathBuf>
```

Defined in: [`packages/project/src/project/worktrees.rs:518`](../../../../packages/project/src/project/worktrees.rs#L518)

#### Parameters

##### project_path

`&ProjectPath`

##### cx

`&App`

#### Returns

`Option<PathBuf>`


***

### dap_store()

```rust
pub fn dap_store(&self) -> Entity<DapStore>
```

Defined in: [`packages/project/src/project/mod.rs:149`](../../../../packages/project/src/project/mod.rs#L149)

#### Returns

`Entity<DapStore>`


***

### bookmark_store()

```rust
pub fn bookmark_store(&self) -> Entity<BookmarkStore>
```

Defined in: [`packages/project/src/project/mod.rs:151`](../../../../packages/project/src/project/mod.rs#L151)

#### Returns

`Entity<BookmarkStore>`


***

### breakpoint_store()

```rust
pub fn breakpoint_store(&self) -> Entity<BreakpointStore>
```

Defined in: [`packages/project/src/project/mod.rs:153`](../../../../packages/project/src/project/mod.rs#L153)

#### Returns

`Entity<BreakpointStore>`


***

### lsp_store()

```rust
pub fn lsp_store(&self) -> Entity<LspStore>
```

Defined in: [`packages/project/src/project/mod.rs:155`](../../../../packages/project/src/project/mod.rs#L155)

#### Returns

`Entity<LspStore>`


***

### worktree_store()

```rust
pub fn worktree_store(&self) -> Entity<WorktreeStore>
```

Defined in: [`packages/project/src/project/mod.rs:157`](../../../../packages/project/src/project/mod.rs#L157)

#### Returns

`Entity<WorktreeStore>`


***

### context_server_store()

```rust
pub fn context_server_store(&self) -> Entity<ContextServerStore>
```

Defined in: [`packages/project/src/project/mod.rs:159`](../../../../packages/project/src/project/mod.rs#L159)

#### Returns

`Entity<ContextServerStore>`


***

### buffer_store()

```rust
pub fn buffer_store(&self) -> &Entity<BufferStore>
```

Defined in: [`packages/project/src/project/mod.rs:163`](../../../../packages/project/src/project/mod.rs#L163)

#### Returns

`&Entity<BufferStore>`


***

### git_store()

```rust
pub fn git_store(&self) -> &Entity<GitStore>
```

Defined in: [`packages/project/src/project/mod.rs:165`](../../../../packages/project/src/project/mod.rs#L165)

#### Returns

`&Entity<GitStore>`


***

### agent_server_store()

```rust
pub fn agent_server_store(&self) -> &Entity<AgentServerStore>
```

Defined in: [`packages/project/src/project/mod.rs:167`](../../../../packages/project/src/project/mod.rs#L167)

#### Returns

`&Entity<AgentServerStore>`


***

### task_store()

```rust
pub fn task_store(&self) -> &Entity<TaskStore>
```

Defined in: [`packages/project/src/project/mod.rs:171`](../../../../packages/project/src/project/mod.rs#L171)

#### Returns

`&Entity<TaskStore>`


***

### snippets()

```rust
pub fn snippets(&self) -> &Entity<SnippetProvider>
```

Defined in: [`packages/project/src/project/mod.rs:173`](../../../../packages/project/src/project/mod.rs#L173)

#### Returns

`&Entity<SnippetProvider>`


***

### languages()

```rust
pub fn languages(&self) -> &Arc<LanguageRegistry>
```

Defined in: [`packages/project/src/project/mod.rs:175`](../../../../packages/project/src/project/mod.rs#L175)

#### Returns

`&Arc<LanguageRegistry>`


***

### client()

```rust
pub fn client(&self) -> Arc<Client>
```

Defined in: [`packages/project/src/project/mod.rs:177`](../../../../packages/project/src/project/mod.rs#L177)

#### Returns

`Arc<Client>`


***

### remote_client()

```rust
pub fn remote_client(&self) -> Option<Entity<RemoteClient>>
```

Defined in: [`packages/project/src/project/mod.rs:179`](../../../../packages/project/src/project/mod.rs#L179)

#### Returns

`Option<Entity<RemoteClient>>`


***

### user_store()

```rust
pub fn user_store(&self) -> Entity<UserStore>
```

Defined in: [`packages/project/src/project/mod.rs:181`](../../../../packages/project/src/project/mod.rs#L181)

#### Returns

`Entity<UserStore>`


***

### node_runtime()

```rust
pub fn node_runtime(&self) -> Option<&NodeRuntime>
```

Defined in: [`packages/project/src/project/mod.rs:183`](../../../../packages/project/src/project/mod.rs#L183)

#### Returns

`Option<&NodeRuntime>`


***

### fs()

```rust
pub fn fs(&self) -> &Arc<dyn Fs>
```

Defined in: [`packages/project/src/project/mod.rs:185`](../../../../packages/project/src/project/mod.rs#L185)

#### Returns

`&Arc<dyn Fs>`


***

### environment()

```rust
pub fn environment(&self) -> &Entity<ProjectEnvironment>
```

Defined in: [`packages/project/src/project/mod.rs:187`](../../../../packages/project/src/project/mod.rs#L187)

#### Returns

`&Entity<ProjectEnvironment>`


***

### cli_environment()

```rust
pub fn cli_environment(&self, cx: &App) -> Option<HashMap<String, String>>
```

Defined in: [`packages/project/src/project/mod.rs:189`](../../../../packages/project/src/project/mod.rs#L189)

#### Parameters

##### cx

`&App`

#### Returns

`Option<HashMap<String, String>>`


***

### active_entry()

```rust
pub fn active_entry(&self) -> Option<ProjectEntryId>
```

Defined in: [`packages/project/src/project/mod.rs:193`](../../../../packages/project/src/project/mod.rs#L193)

#### Returns

`Option<ProjectEntryId>`


***

### collaborators()

```rust
pub fn collaborators(&self) -> &HashMap<PeerId, Collaborator>
```

Defined in: [`packages/project/src/project/mod.rs:195`](../../../../packages/project/src/project/mod.rs#L195)

#### Returns

`&HashMap<PeerId, Collaborator>`


***

### host()

```rust
pub fn host(&self) -> Option<&Collaborator>
```

Defined in: [`packages/project/src/project/mod.rs:199`](../../../../packages/project/src/project/mod.rs#L199)

#### Returns

`Option<&Collaborator>`


***

### opened_buffers()

```rust
pub fn opened_buffers(&self, cx: &App) -> Vec<Entity<Buffer>>
```

Defined in: [`packages/project/src/project/mod.rs:201`](../../../../packages/project/src/project/mod.rs#L201)

#### Parameters

##### cx

`&App`

#### Returns

`Vec<Entity<Buffer>>`


***

### buffer_for_id()

```rust
pub fn buffer_for_id(&self, remote_id: BufferId, cx: &App) -> Option<Entity<Buffer>>
```

Defined in: [`packages/project/src/project/mod.rs:205`](../../../../packages/project/src/project/mod.rs#L205)

#### Parameters

##### remote_id

`BufferId`

##### cx

`&App`

#### Returns

`Option<Entity<Buffer>>`


***

### download_file()

```rust
pub fn download_file(&self, _worktree_id: WorktreeId, _path: Arc<RelPath>, _destination_path: PathBuf, _cx: &Context<'_, Self>) -> Task<Result<()>>
```

Defined in: [`packages/project/src/project/mod.rs:210`](../../../../packages/project/src/project/mod.rs#L210)

Zed 远程项目功能 — 我们没移植，返回错误

#### Parameters

##### _worktree_id

`WorktreeId`

##### _path

`Arc<RelPath>`

##### _destination_path

`PathBuf`

##### _cx

`&Context<'_, Self>`

#### Returns

`Task<Result<()>>`

## Trait Implementations

- `impl Borrow for Project`
- `impl BorrowMut for Project`
- `impl Into for Project`
- `impl From for Project`
- `impl TryInto for Project`
- `impl TryFrom for Project`
- `impl Any for Project`
- `impl VZip for Project`
- `impl CastableFrom for Project`
- `impl CastableFrom for Project`
- `impl Read for Project`
- `impl IntoEither for Project`
- `impl ErasedDestructor for Project`
- `impl Same for Project`
- `impl Pointable for Project`
- `impl Instrument for Project`
- `impl WithSubscriber for Project`
- `impl FromAngle for Project`
- `impl IntoAngle for Project`
- `impl IntoCam16Unclamped for Project`
- `impl Cam16IntoUnclamped for Project`
- `impl ArraysFrom for Project`
- `impl ArraysInto for Project`
- `impl ComponentsFrom for Project`
- `impl TryComponentsInto for Project`
- `impl UintsFrom for Project`
- `impl UintsInto for Project`
- `impl AdaptIntoUnclamped for Project`
- `impl AdaptInto for Project`
- `impl IntoColor for Project`
- `impl IntoColorUnclamped for Project`
- `impl TryIntoColor for Project`
- `impl FromStimulus for Project`
- `impl IntoStimulus for Project`
- `impl EventEmitter for Project`

## Auto Trait Implementations

`Freeze` `Unpin` `UnsafeUnpin`

