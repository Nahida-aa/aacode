---
id: GitStore
title: GitStore
---

# Struct: GitStore

Defined in: [`packages/project/src/git_store/mod.rs:102`](../../../../packages/project/src/git_store/mod.rs#L102)

## Definition

```rust
pub struct GitStore
```

_（存在非公开字段）_

## Implementations

### local()

```rust
pub fn local(worktree_store: &Entity<WorktreeStore>, buffer_store: Entity<BufferStore>, environment: Entity<ProjectEnvironment>, fs: Arc<dyn Fs>, cx: &Context<'_, Self>) -> Self
```

Defined in: [`packages/project/src/git_store/mod.rs:884`](../../../../packages/project/src/git_store/mod.rs#L884)

#### Parameters

##### worktree_store

`&Entity<WorktreeStore>`

##### buffer_store

`Entity<BufferStore>`

##### environment

`Entity<ProjectEnvironment>`

##### fs

`Arc<dyn Fs>`

##### cx

`&Context<'_, Self>`

#### Returns

`Self`


***

### remote()

```rust
pub fn remote(worktree_store: &Entity<WorktreeStore>, buffer_store: Entity<BufferStore>, upstream_client: AnyProtoClient, project_id: u64, cx: &Context<'_, Self>) -> Self
```

Defined in: [`packages/project/src/git_store/mod.rs:969`](../../../../packages/project/src/git_store/mod.rs#L969)

#### Parameters

##### worktree_store

`&Entity<WorktreeStore>`

##### buffer_store

`Entity<BufferStore>`

##### upstream_client

`AnyProtoClient`

##### project_id

`u64`

##### cx

`&Context<'_, Self>`

#### Returns

`Self`


***

### init()

```rust
pub fn init(client: &AnyProtoClient)
```

Defined in: [`packages/project/src/git_store/mod.rs:1036`](../../../../packages/project/src/git_store/mod.rs#L1036)

#### Parameters

##### client

`&AnyProtoClient`


***

### is_local()

```rust
pub fn is_local(&self) -> bool
```

Defined in: [`packages/project/src/git_store/mod.rs:1101`](../../../../packages/project/src/git_store/mod.rs#L1101)

#### Returns

`bool`


***

### set_active_repo_for_path()

```rust
pub fn set_active_repo_for_path(&self, project_path: &ProjectPath, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/git_store/mod.rs:1112`](../../../../packages/project/src/git_store/mod.rs#L1112)

#### Parameters

##### project_path

`&ProjectPath`

##### cx

`&Context<'_, Self>`


***

### set_active_repo_for_worktree()

```rust
pub fn set_active_repo_for_worktree(&self, worktree_id: WorktreeId, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/git_store/mod.rs:1118`](../../../../packages/project/src/git_store/mod.rs#L1118)

#### Parameters

##### worktree_id

`WorktreeId`

##### cx

`&Context<'_, Self>`


***

### shared()

```rust
pub fn shared(&self, project_id: u64, client: AnyProtoClient, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/git_store/mod.rs:1154`](../../../../packages/project/src/git_store/mod.rs#L1154)

#### Parameters

##### project_id

`u64`

##### client

`AnyProtoClient`

##### cx

`&Context<'_, Self>`


***

### unshared()

```rust
pub fn unshared(&self, _cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/git_store/mod.rs:1235`](../../../../packages/project/src/git_store/mod.rs#L1235)

#### Parameters

##### _cx

`&Context<'_, Self>`


***

### active_repository()

```rust
pub fn active_repository(&self) -> Option<Entity<Repository>>
```

Defined in: [`packages/project/src/git_store/mod.rs:1257`](../../../../packages/project/src/git_store/mod.rs#L1257)

#### Returns

`Option<Entity<Repository>>`


***

### open_unstaged_diff()

```rust
pub fn open_unstaged_diff(&self, buffer: Entity<Buffer>, cx: &Context<'_, Self>) -> Task<Result<Entity<BufferDiff>>>
```

Defined in: [`packages/project/src/git_store/mod.rs:1274`](../../../../packages/project/src/git_store/mod.rs#L1274)

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

Defined in: [`packages/project/src/git_store/mod.rs:1346`](../../../../packages/project/src/git_store/mod.rs#L1346)

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

### stage_hunks()

```rust
pub fn stage_hunks(&self, buffer: Entity<Buffer>, unstaged_diff: Entity<BufferDiff>, worktree_ranges: Vec<Range<Anchor>>, cx: &Context<'_, Self>) -> Result<()>
```

Defined in: [`packages/project/src/git_store/mod.rs:1416`](../../../../packages/project/src/git_store/mod.rs#L1416)

Stages the worktree changes covered by `worktree_ranges`, acting on the
given unstaged (index-vs-worktree) diff. Used by both the unstaged-changes
view and the uncommitted (gutter) controls: "stage" means the same index
change regardless of which view it was invoked from, so callers holding an
uncommitted diff pass its unstaged secondary.

Decomposes the worktree region into the unstaged hunks it covers, so no
worktree->index projection is needed. Optimistically suppresses the staged
hunks from the unstaged diff and, if the uncommitted diff happens to be
open, marks the corresponding uncommitted hunks as staging.

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

Defined in: [`packages/project/src/git_store/mod.rs:1531`](../../../../packages/project/src/git_store/mod.rs#L1531)

Unstages the worktree changes covered by `worktree_ranges`, acting on the
given uncommitted (HEAD-vs-worktree) diff, invoked from the uncommitted
(gutter) controls. Uses the worktree->index projection (the hard part)
because the acted-on hunks are HEAD-vs-worktree.

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

Defined in: [`packages/project/src/git_store/mod.rs:1592`](../../../../packages/project/src/git_store/mod.rs#L1592)

Unstages staged (HEAD-vs-index) hunks covered by `index_ranges` (in the
index text buffer's coordinates), acting on the given staged diff,
invoked from the staged-changes view. The acted-on hunks already carry
an index range, so no projection is needed; optimistically suppresses
them from the staged diff.

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

### open_diff_since()

```rust
pub fn open_diff_since(&self, oid: Option<Oid>, buffer: Entity<Buffer>, repo: Entity<Repository>, cx: &Context<'_, Self>) -> Task<Result<Entity<BufferDiff>>>
```

Defined in: [`packages/project/src/git_store/mod.rs:1656`](../../../../packages/project/src/git_store/mod.rs#L1656)

#### Parameters

##### oid

`Option<Oid>`

##### buffer

`Entity<Buffer>`

##### repo

`Entity<Repository>`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<Entity<BufferDiff>>>`


***

### open_display_diff()

```rust
pub fn open_display_diff(&self, buffer: Entity<Buffer>, cx: &Context<'_, Self>) -> Task<Result<Entity<BufferDiff>>>
```

Defined in: [`packages/project/src/git_store/mod.rs:1755`](../../../../packages/project/src/git_store/mod.rs#L1755)

Opens the diff that editors display in the gutter: the uncommitted diff
when the diff base is HEAD, otherwise a diff against the base blob.

#### Parameters

##### buffer

`Entity<Buffer>`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<Entity<BufferDiff>>>`


***

### open_uncommitted_diff()

```rust
pub fn open_uncommitted_diff(&self, buffer: Entity<Buffer>, cx: &Context<'_, Self>) -> Task<Result<Entity<BufferDiff>>>
```

Defined in: [`packages/project/src/git_store/mod.rs:1789`](../../../../packages/project/src/git_store/mod.rs#L1789)

#### Parameters

##### buffer

`Entity<Buffer>`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<Entity<BufferDiff>>>`


***

### get_unstaged_diff()

```rust
pub fn get_unstaged_diff(&self, buffer_id: BufferId, cx: &App) -> Option<Entity<BufferDiff>>
```

Defined in: [`packages/project/src/git_store/mod.rs:2035`](../../../../packages/project/src/git_store/mod.rs#L2035)

#### Parameters

##### buffer_id

`BufferId`

##### cx

`&App`

#### Returns

`Option<Entity<BufferDiff>>`


***

### get_staged_diff()

```rust
pub fn get_staged_diff(&self, buffer_id: BufferId, cx: &App) -> Option<Entity<BufferDiff>>
```

Defined in: [`packages/project/src/git_store/mod.rs:2040`](../../../../packages/project/src/git_store/mod.rs#L2040)

#### Parameters

##### buffer_id

`BufferId`

##### cx

`&App`

#### Returns

`Option<Entity<BufferDiff>>`


***

### get_uncommitted_diff()

```rust
pub fn get_uncommitted_diff(&self, buffer_id: BufferId, cx: &App) -> Option<Entity<BufferDiff>>
```

Defined in: [`packages/project/src/git_store/mod.rs:2045`](../../../../packages/project/src/git_store/mod.rs#L2045)

#### Parameters

##### buffer_id

`BufferId`

##### cx

`&App`

#### Returns

`Option<Entity<BufferDiff>>`


***

### get_diff_since_oid()

```rust
pub fn get_diff_since_oid(&self, buffer_id: BufferId, oid: Option<Oid>, cx: &App) -> Option<Entity<BufferDiff>>
```

Defined in: [`packages/project/src/git_store/mod.rs:2054`](../../../../packages/project/src/git_store/mod.rs#L2054)

#### Parameters

##### buffer_id

`BufferId`

##### oid

`Option<Oid>`

##### cx

`&App`

#### Returns

`Option<Entity<BufferDiff>>`


***

### open_conflict_set()

```rust
pub fn open_conflict_set(&self, buffer: Entity<Buffer>, cx: &Context<'_, Self>) -> Task<Entity<ConflictSet>>
```

Defined in: [`packages/project/src/git_store/mod.rs:2075`](../../../../packages/project/src/git_store/mod.rs#L2075)

#### Parameters

##### buffer

`Entity<Buffer>`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Entity<ConflictSet>>`


***

### project_path_git_status()

```rust
pub fn project_path_git_status(&self, project_path: &ProjectPath, cx: &App) -> Option<FileStatus>
```

Defined in: [`packages/project/src/git_store/mod.rs:2129`](../../../../packages/project/src/git_store/mod.rs#L2129)

#### Parameters

##### project_path

`&ProjectPath`

##### cx

`&App`

#### Returns

`Option<FileStatus>`


***

### checkpoint()

```rust
pub fn checkpoint(&self, cx: &App) -> Task<Result<GitStoreCheckpoint>>
```

Defined in: [`packages/project/src/git_store/mod.rs:2138`](../../../../packages/project/src/git_store/mod.rs#L2138)

#### Parameters

##### cx

`&App`

#### Returns

`Task<Result<GitStoreCheckpoint>>`


***

### restore_checkpoint()

```rust
pub fn restore_checkpoint(&self, checkpoint: GitStoreCheckpoint, cx: &App) -> Task<Result<()>>
```

Defined in: [`packages/project/src/git_store/mod.rs:2159`](../../../../packages/project/src/git_store/mod.rs#L2159)

#### Parameters

##### checkpoint

[`GitStoreCheckpoint`](GitStoreCheckpoint.md)

##### cx

`&App`

#### Returns

`Task<Result<()>>`


***

### compare_checkpoints()

```rust
pub fn compare_checkpoints(&self, left: GitStoreCheckpoint, right: GitStoreCheckpoint, cx: &App) -> Task<Result<bool>>
```

Defined in: [`packages/project/src/git_store/mod.rs:2186`](../../../../packages/project/src/git_store/mod.rs#L2186)

Compares two checkpoints, returning true if they are equal.

#### Parameters

##### left

[`GitStoreCheckpoint`](GitStoreCheckpoint.md)

##### right

[`GitStoreCheckpoint`](GitStoreCheckpoint.md)

##### cx

`&App`

#### Returns

`Task<Result<bool>>`


***

### blame_buffer()

```rust
pub fn blame_buffer(&self, buffer: &Entity<Buffer>, version: Option<Global>, cx: &Context<'_, Self>) -> Task<Result<Option<Blame>>>
```

Defined in: [`packages/project/src/git_store/mod.rs:2225`](../../../../packages/project/src/git_store/mod.rs#L2225)

Blames a buffer.

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

### get_permalink_to_line()

```rust
pub fn get_permalink_to_line(&self, buffer: &Entity<Buffer>, selection: Range<u32>, cx: &App) -> Task<Result<Url>>
```

Defined in: [`packages/project/src/git_store/mod.rs:2271`](../../../../packages/project/src/git_store/mod.rs#L2271)

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

Defined in: [`packages/project/src/git_store/mod.rs:2319`](../../../../packages/project/src/git_store/mod.rs#L2319)

#### Parameters

##### project_path

`&ProjectPath`

##### cx

`&App`

#### Returns

`Task<Result<Url>>`


***

### recalculate_buffer_diffs()

```rust
pub fn recalculate_buffer_diffs(&self, buffers: Vec<Entity<Buffer>>, cx: &Context<'_, Self>) -> impl ? + ?
```

Defined in: [`packages/project/src/git_store/mod.rs:3118`](../../../../packages/project/src/git_store/mod.rs#L3118)

#### Parameters

##### buffers

`Vec<Entity<Buffer>>`

##### cx

`&Context<'_, Self>`

#### Returns

`impl ? + ?`


***

### repositories()

```rust
pub fn repositories(&self) -> &HashMap<RepositoryId, Entity<Repository>>
```

Defined in: [`packages/project/src/git_store/mod.rs:3218`](../../../../packages/project/src/git_store/mod.rs#L3218)

#### Returns

`&HashMap<RepositoryId, Entity<Repository>>`


***

### original_repo_path_for_worktree()

```rust
pub fn original_repo_path_for_worktree(&self, worktree_id: WorktreeId, cx: &App) -> Option<Arc<Path>>
```

Defined in: [`packages/project/src/git_store/mod.rs:3226`](../../../../packages/project/src/git_store/mod.rs#L3226)

Returns the main repository working directory for the given worktree.
For normal checkouts this equals the worktree's own path. For linked
worktrees it points back to the main worktree, if one exists. Linked
worktrees attached to a bare repository have no main worktree path.

#### Parameters

##### worktree_id

`WorktreeId`

##### cx

`&App`

#### Returns

`Option<Arc<Path>>`


***

### status_for_buffer_id()

```rust
pub fn status_for_buffer_id(&self, buffer_id: BufferId, cx: &App) -> Option<FileStatus>
```

Defined in: [`packages/project/src/git_store/mod.rs:3248`](../../../../packages/project/src/git_store/mod.rs#L3248)

#### Parameters

##### buffer_id

`BufferId`

##### cx

`&App`

#### Returns

`Option<FileStatus>`


***

### repository_and_path_for_buffer_id()

```rust
pub fn repository_and_path_for_buffer_id(&self, buffer_id: BufferId, cx: &App) -> Option<(Entity<Repository>, RepoPath)>
```

Defined in: [`packages/project/src/git_store/mod.rs:3254`](../../../../packages/project/src/git_store/mod.rs#L3254)

#### Parameters

##### buffer_id

`BufferId`

##### cx

`&App`

#### Returns

`Option<(Entity<Repository>, RepoPath)>`


***

### repository_and_path_for_project_path()

```rust
pub fn repository_and_path_for_project_path(&self, path: &ProjectPath, cx: &App) -> Option<(Entity<Repository>, RepoPath)>
```

Defined in: [`packages/project/src/git_store/mod.rs:3264`](../../../../packages/project/src/git_store/mod.rs#L3264)

#### Parameters

##### path

`&ProjectPath`

##### cx

`&App`

#### Returns

`Option<(Entity<Repository>, RepoPath)>`


***

### git_init()

```rust
pub fn git_init(&self, path: Arc<Path>, fallback_branch_name: String, cx: &App) -> Task<Result<()>>
```

Defined in: [`packages/project/src/git_store/mod.rs:3279`](../../../../packages/project/src/git_store/mod.rs#L3279)

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

### git_clone()

```rust
pub fn git_clone<impl Into<Arc<std::path::Path>>: Into>(&self, repo: String, path: impl ?, cx: &App) -> Task<Result<()>>
```

Defined in: [`packages/project/src/git_store/mod.rs:3312`](../../../../packages/project/src/git_store/mod.rs#L3312)

#### Parameters

##### repo

`String`

##### path

`impl ?`

##### cx

`&App`

#### Returns

`Task<Result<()>>`


***

### git_config()

```rust
pub fn git_config(&self, path: Arc<Path>, args: Vec<String>, cx: &App) -> Task<Result<String>>
```

Defined in: [`packages/project/src/git_store/mod.rs:3353`](../../../../packages/project/src/git_store/mod.rs#L3353)

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

### repo_snapshots()

```rust
pub fn repo_snapshots(&self, cx: &App) -> HashMap<RepositoryId, RepositorySnapshot>
```

Defined in: [`packages/project/src/git_store/mod.rs:5062`](../../../../packages/project/src/git_store/mod.rs#L5062)

#### Parameters

##### cx

`&App`

#### Returns

`HashMap<RepositoryId, RepositorySnapshot>`


***

### display_repo_snapshots()

```rust
pub fn display_repo_snapshots(&self, cx: &App) -> HashMap<RepositoryId, RepositorySnapshot>
```

Defined in: [`packages/project/src/git_store/mod.rs:5069`](../../../../packages/project/src/git_store/mod.rs#L5069)

#### Parameters

##### cx

`&App`

#### Returns

`HashMap<RepositoryId, RepositorySnapshot>`


***

### diff_base()

```rust
pub fn diff_base(&self) -> GitDiffBaseSetting
```

Defined in: [`packages/project/src/git_store/mod.rs:5087`](../../../../packages/project/src/git_store/mod.rs#L5087)

#### Returns

`GitDiffBaseSetting`


***

### display_status_for_buffer_id()

```rust
pub fn display_status_for_buffer_id(&self, buffer_id: BufferId, cx: &App) -> Option<FileStatus>
```

Defined in: [`packages/project/src/git_store/mod.rs:5106`](../../../../packages/project/src/git_store/mod.rs#L5106)

#### Parameters

##### buffer_id

`BufferId`

##### cx

`&App`

#### Returns

`Option<FileStatus>`


***

### display_status_for_project_path()

```rust
pub fn display_status_for_project_path(&self, project_path: &ProjectPath, cx: &App) -> Option<FileStatus>
```

Defined in: [`packages/project/src/git_store/mod.rs:5115`](../../../../packages/project/src/git_store/mod.rs#L5115)

#### Parameters

##### project_path

`&ProjectPath`

##### cx

`&App`

#### Returns

`Option<FileStatus>`


***

### display_diff_for_repo()

```rust
pub fn display_diff_for_repo(&self, repo_id: RepositoryId) -> Option<Entity<DiffBufferList>>
```

Defined in: [`packages/project/src/git_store/mod.rs:5143`](../../../../packages/project/src/git_store/mod.rs#L5143)

#### Parameters

##### repo_id

[`RepositoryId`](RepositoryId.md)

#### Returns

`Option<Entity<DiffBufferList>>`


***

### ensure_display_diff()

```rust
pub fn ensure_display_diff(&self, repo: Entity<Repository>, resolved_ref: SharedString, cx: &Context<'_, Self>) -> Entity<DiffBufferList>
```

Defined in: [`packages/project/src/git_store/mod.rs:5151`](../../../../packages/project/src/git_store/mod.rs#L5151)

#### Parameters

##### repo

`Entity<Repository>`

##### resolved_ref

`SharedString`

##### cx

`&Context<'_, Self>`

#### Returns

`Entity<DiffBufferList>`

## Trait Implementations

- `impl Borrow for GitStore`
- `impl BorrowMut for GitStore`
- `impl Into for GitStore`
- `impl From for GitStore`
- `impl TryInto for GitStore`
- `impl TryFrom for GitStore`
- `impl Any for GitStore`
- `impl VZip for GitStore`
- `impl CastableFrom for GitStore`
- `impl CastableFrom for GitStore`
- `impl Read for GitStore`
- `impl IntoEither for GitStore`
- `impl ErasedDestructor for GitStore`
- `impl Same for GitStore`
- `impl Pointable for GitStore`
- `impl Instrument for GitStore`
- `impl WithSubscriber for GitStore`
- `impl FromAngle for GitStore`
- `impl IntoAngle for GitStore`
- `impl IntoCam16Unclamped for GitStore`
- `impl Cam16IntoUnclamped for GitStore`
- `impl ArraysFrom for GitStore`
- `impl ArraysInto for GitStore`
- `impl ComponentsFrom for GitStore`
- `impl TryComponentsInto for GitStore`
- `impl UintsFrom for GitStore`
- `impl UintsInto for GitStore`
- `impl AdaptIntoUnclamped for GitStore`
- `impl AdaptInto for GitStore`
- `impl IntoColor for GitStore`
- `impl IntoColorUnclamped for GitStore`
- `impl TryIntoColor for GitStore`
- `impl FromStimulus for GitStore`
- `impl IntoStimulus for GitStore`
- `impl EventEmitter for GitStore`

## Auto Trait Implementations

`Freeze` `Unpin` `UnsafeUnpin`

