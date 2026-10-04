---
id: Repository
title: Repository
---

# Struct: Repository

Defined in: [`packages/project/src/git_store/mod.rs:683`](../../../../packages/project/src/git_store/mod.rs#L683)

## Definition

```rust
pub struct Repository
```

_（存在非公开字段）_

## Implementations

### is_trusted()

```rust
pub fn is_trusted(&self) -> bool
```

Defined in: [`packages/project/src/git_store/mod.rs:6453`](../../../../packages/project/src/git_store/mod.rs#L6453)

#### Returns

`bool`


***

### snapshot()

```rust
pub fn snapshot(&self) -> RepositorySnapshot
```

Defined in: [`packages/project/src/git_store/mod.rs:6460`](../../../../packages/project/src/git_store/mod.rs#L6460)

#### Returns

[`RepositorySnapshot`](RepositorySnapshot.md)


***

### pending_ops()

```rust
pub fn pending_ops(&self) -> impl ? + ?
```

Defined in: [`packages/project/src/git_store/mod.rs:6464`](../../../../packages/project/src/git_store/mod.rs#L6464)

#### Returns

`impl ? + ?`


***

### pending_ops_summary()

```rust
pub fn pending_ops_summary(&self) -> PathSummary<PendingOpsSummary>
```

Defined in: [`packages/project/src/git_store/mod.rs:6468`](../../../../packages/project/src/git_store/mod.rs#L6468)

#### Returns

`PathSummary<PendingOpsSummary>`


***

### pending_ops_for_path()

```rust
pub fn pending_ops_for_path(&self, path: &RepoPath) -> Option<PendingOps>
```

Defined in: [`packages/project/src/git_store/mod.rs:6472`](../../../../packages/project/src/git_store/mod.rs#L6472)

#### Parameters

##### path

`&RepoPath`

#### Returns

`Option<PendingOps>`


***

### git_store()

```rust
pub fn git_store(&self) -> Option<Entity<GitStore>>
```

Defined in: [`packages/project/src/git_store/mod.rs:6654`](../../../../packages/project/src/git_store/mod.rs#L6654)

#### Returns

`Option<Entity<GitStore>>`


***

### send_job()

```rust
pub fn send_job<F, Fut, R>(&self, description: &'static str, status: Option<SharedString>, job: F) -> Receiver<R>
where
    F: FnOnce(RepositoryState, AsyncApp) -> Fut + 'static,
    Fut: Future + 'static,
    R: Send + 'static
```

Defined in: [`packages/project/src/git_store/mod.rs:6847`](../../../../packages/project/src/git_store/mod.rs#L6847)

#### Parameters

##### description

`&'static str`

##### status

`Option<SharedString>`

##### job

`F`

#### Returns

`Receiver<R>`


***

### set_as_active_repository()

```rust
pub fn set_as_active_repository(&self, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/git_store/mod.rs:6922`](../../../../packages/project/src/git_store/mod.rs#L6922)

#### Parameters

##### cx

`&Context<'_, Self>`


***

### cached_status()

```rust
pub fn cached_status(&self) -> impl ? + ?
```

Defined in: [`packages/project/src/git_store/mod.rs:6940`](../../../../packages/project/src/git_store/mod.rs#L6940)

#### Returns

`impl ? + ?`


***

### diff_stat_for_path()

```rust
pub fn diff_stat_for_path(&self, path: &RepoPath) -> Option<DiffStat>
```

Defined in: [`packages/project/src/git_store/mod.rs:6944`](../../../../packages/project/src/git_store/mod.rs#L6944)

#### Parameters

##### path

`&RepoPath`

#### Returns

`Option<DiffStat>`


***

### cached_stash()

```rust
pub fn cached_stash(&self) -> GitStash
```

Defined in: [`packages/project/src/git_store/mod.rs:6948`](../../../../packages/project/src/git_store/mod.rs#L6948)

#### Returns

`GitStash`


***

### repo_path_to_project_path()

```rust
pub fn repo_path_to_project_path(&self, path: &RepoPath, cx: &App) -> Option<ProjectPath>
```

Defined in: [`packages/project/src/git_store/mod.rs:6952`](../../../../packages/project/src/git_store/mod.rs#L6952)

#### Parameters

##### path

`&RepoPath`

##### cx

`&App`

#### Returns

`Option<ProjectPath>`


***

### project_path_to_repo_path()

```rust
pub fn project_path_to_repo_path(&self, path: &ProjectPath, cx: &App) -> Option<RepoPath>
```

Defined in: [`packages/project/src/git_store/mod.rs:6964`](../../../../packages/project/src/git_store/mod.rs#L6964)

#### Parameters

##### path

`&ProjectPath`

##### cx

`&App`

#### Returns

`Option<RepoPath>`


***

### contains_sub_repo()

```rust
pub fn contains_sub_repo(&self, other: &Entity<Self>, cx: &App) -> bool
```

Defined in: [`packages/project/src/git_store/mod.rs:6971`](../../../../packages/project/src/git_store/mod.rs#L6971)

#### Parameters

##### other

`&Entity<Self>`

##### cx

`&App`

#### Returns

`bool`


***

### commit_message_buffer()

```rust
pub fn commit_message_buffer(&self) -> Option<&Entity<Buffer>>
```

Defined in: [`packages/project/src/git_store/mod.rs:6979`](../../../../packages/project/src/git_store/mod.rs#L6979)

#### Returns

`Option<&Entity<Buffer>>`


***

### open_commit_buffer()

```rust
pub fn open_commit_buffer(&self, languages: Option<Arc<LanguageRegistry>>, buffer_store: Entity<BufferStore>, cx: &Context<'_, Self>) -> Task<Result<Entity<Buffer>>>
```

Defined in: [`packages/project/src/git_store/mod.rs:6983`](../../../../packages/project/src/git_store/mod.rs#L6983)

#### Parameters

##### languages

`Option<Arc<LanguageRegistry>>`

##### buffer_store

`Entity<BufferStore>`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<Entity<Buffer>>>`


***

### checkout_files()

```rust
pub fn checkout_files(&self, commit: &str, paths: Vec<RepoPath>, cx: &Context<'_, Self>) -> Task<Result<()>>
```

Defined in: [`packages/project/src/git_store/mod.rs:7065`](../../../../packages/project/src/git_store/mod.rs#L7065)

#### Parameters

##### commit

`&str`

##### paths

`Vec<RepoPath>`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<()>>`


***

### reset()

```rust
pub fn reset(&self, commit: String, reset_mode: ResetMode, cx: &Context<'_, Self>) -> Receiver<Result<()>>
```

Defined in: [`packages/project/src/git_store/mod.rs:7121`](../../../../packages/project/src/git_store/mod.rs#L7121)

#### Parameters

##### commit

`String`

##### reset_mode

`ResetMode`

##### cx

`&Context<'_, Self>`

#### Returns

`Receiver<Result<()>>`


***

### show()

```rust
pub fn show(&self, commit: String) -> Receiver<Result<CommitDetails>>
```

Defined in: [`packages/project/src/git_store/mod.rs:7171`](../../../../packages/project/src/git_store/mod.rs#L7171)

#### Parameters

##### commit

`String`

#### Returns

`Receiver<Result<CommitDetails>>`


***

### show_commit()

```rust
pub fn show_commit(&self, sha: String, cx: &App) -> Task<Result<CommitDetails>>
```

Defined in: [`packages/project/src/git_store/mod.rs:7178`](../../../../packages/project/src/git_store/mod.rs#L7178)

#### Parameters

##### sha

`String`

##### cx

`&App`

#### Returns

`Task<Result<CommitDetails>>`


***

### load_commit_diff()

```rust
pub fn load_commit_diff(&self, commit: String, ignore_shallow_boundary: bool, cx: &App) -> Task<Result<CommitDiff>>
```

Defined in: [`packages/project/src/git_store/mod.rs:7218`](../../../../packages/project/src/git_store/mod.rs#L7218)

#### Parameters

##### commit

`String`

##### ignore_shallow_boundary

`bool`

##### cx

`&App`

#### Returns

`Task<Result<CommitDiff>>`


***

### file_history_changed_files()

```rust
pub fn file_history_changed_files(&self, paths: Vec<RepoPath>, commit_limit: usize) -> Receiver<Result<Vec<FileHistoryChangedFileSets>>>
```

Defined in: [`packages/project/src/git_store/mod.rs:7265`](../../../../packages/project/src/git_store/mod.rs#L7265)

#### Parameters

##### paths

`Vec<RepoPath>`

##### commit_limit

`usize`

#### Returns

`Receiver<Result<Vec<FileHistoryChangedFileSets>>>`


***

### get_graph_data()

```rust
pub fn get_graph_data(&self, log_source: LogSource, log_order: LogOrder) -> Option<&InitialGitGraphData>
```

Defined in: [`packages/project/src/git_store/mod.rs:7288`](../../../../packages/project/src/git_store/mod.rs#L7288)

#### Parameters

##### log_source

`LogSource`

##### log_order

`LogOrder`

#### Returns

`Option<&InitialGitGraphData>`


***

### search_commits()

```rust
pub fn search_commits(&self, log_source: LogSource, search_args: SearchCommitArgs, request_tx: Sender<Oid>, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/git_store/mod.rs:7296`](../../../../packages/project/src/git_store/mod.rs#L7296)

#### Parameters

##### log_source

`LogSource`

##### search_args

`SearchCommitArgs`

##### request_tx

`Sender<Oid>`

##### cx

`&Context<'_, Self>`


***

### graph_data()

```rust
pub fn graph_data(&self, log_source: LogSource, log_order: LogOrder, range: Range<usize>, cx: &Context<'_, Self>) -> GraphDataResponse<'_>
```

Defined in: [`packages/project/src/git_store/mod.rs:7365`](../../../../packages/project/src/git_store/mod.rs#L7365)

#### Parameters

##### log_source

`LogSource`

##### log_order

`LogOrder`

##### range

`Range<usize>`

##### cx

`&Context<'_, Self>`

#### Returns

[`GraphDataResponse<'_>`](GraphDataResponse.md)


***

### fetch_commit_data()

```rust
pub fn fetch_commit_data(&self, sha: Oid, await_result: bool, cx: &Context<'_, Self>) -> &CommitDataState
```

Defined in: [`packages/project/src/git_store/mod.rs:7572`](../../../../packages/project/src/git_store/mod.rs#L7572)

#### Parameters

##### sha

`Oid`

##### await_result

`bool`

##### cx

`&Context<'_, Self>`

#### Returns

`&CommitDataState`


***

### stage_entries()

```rust
pub fn stage_entries(&self, entries: Vec<RepoPath>, cx: &Context<'_, Self>) -> Task<Result<()>>
```

Defined in: [`packages/project/src/git_store/mod.rs:7982`](../../../../packages/project/src/git_store/mod.rs#L7982)

#### Parameters

##### entries

`Vec<RepoPath>`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<()>>`


***

### unstage_entries()

```rust
pub fn unstage_entries(&self, entries: Vec<RepoPath>, cx: &Context<'_, Self>) -> Task<Result<()>>
```

Defined in: [`packages/project/src/git_store/mod.rs:7990`](../../../../packages/project/src/git_store/mod.rs#L7990)

#### Parameters

##### entries

`Vec<RepoPath>`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<()>>`


***

### stage_all()

```rust
pub fn stage_all(&self, cx: &Context<'_, Self>) -> Task<Result<()>>
```

Defined in: [`packages/project/src/git_store/mod.rs:8164`](../../../../packages/project/src/git_store/mod.rs#L8164)

#### Parameters

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<()>>`


***

### unstage_all()

```rust
pub fn unstage_all(&self, cx: &Context<'_, Self>) -> Task<Result<()>>
```

Defined in: [`packages/project/src/git_store/mod.rs:8198`](../../../../packages/project/src/git_store/mod.rs#L8198)

#### Parameters

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<()>>`


***

### stash_all()

```rust
pub fn stash_all(&self, message: Option<String>, cx: &Context<'_, Self>) -> Task<Result<()>>
```

Defined in: [`packages/project/src/git_store/mod.rs:8232`](../../../../packages/project/src/git_store/mod.rs#L8232)

#### Parameters

##### message

`Option<String>`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<()>>`


***

### stash_tracked()

```rust
pub fn stash_tracked(&self, message: Option<String>, cx: &Context<'_, Self>) -> Task<Result<()>>
```

Defined in: [`packages/project/src/git_store/mod.rs:8242`](../../../../packages/project/src/git_store/mod.rs#L8242)

#### Parameters

##### message

`Option<String>`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<()>>`


***

### stash_entries()

```rust
pub fn stash_entries(&self, entries: Vec<RepoPath>, message: Option<String>, cx: &Context<'_, Self>) -> Task<Result<()>>
```

Defined in: [`packages/project/src/git_store/mod.rs:8263`](../../../../packages/project/src/git_store/mod.rs#L8263)

#### Parameters

##### entries

`Vec<RepoPath>`

##### message

`Option<String>`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<()>>`


***

### stash_staged()

```rust
pub fn stash_staged(&self, message: Option<String>, cx: &Context<'_, Self>) -> Task<Result<()>>
```

Defined in: [`packages/project/src/git_store/mod.rs:8303`](../../../../packages/project/src/git_store/mod.rs#L8303)

#### Parameters

##### message

`Option<String>`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<()>>`


***

### stash_pop()

```rust
pub fn stash_pop(&self, index: Option<usize>, cx: &Context<'_, Self>) -> Task<Result<()>>
```

Defined in: [`packages/project/src/git_store/mod.rs:8339`](../../../../packages/project/src/git_store/mod.rs#L8339)

#### Parameters

##### index

`Option<usize>`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<()>>`


***

### stash_apply()

```rust
pub fn stash_apply(&self, index: Option<usize>, cx: &Context<'_, Self>) -> Task<Result<()>>
```

Defined in: [`packages/project/src/git_store/mod.rs:8373`](../../../../packages/project/src/git_store/mod.rs#L8373)

#### Parameters

##### index

`Option<usize>`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<()>>`


***

### add_path_to_gitignore()

```rust
pub fn add_path_to_gitignore(&self, repo_path: &RepoPath, is_dir: bool) -> Receiver<Result<()>>
```

Defined in: [`packages/project/src/git_store/mod.rs:8407`](../../../../packages/project/src/git_store/mod.rs#L8407)

#### Parameters

##### repo_path

`&RepoPath`

##### is_dir

`bool`

#### Returns

`Receiver<Result<()>>`


***

### add_path_to_git_info_exclude()

```rust
pub fn add_path_to_git_info_exclude(&self, repo_path: &RepoPath, is_dir: bool) -> Receiver<Result<()>>
```

Defined in: [`packages/project/src/git_store/mod.rs:8452`](../../../../packages/project/src/git_store/mod.rs#L8452)

#### Parameters

##### repo_path

`&RepoPath`

##### is_dir

`bool`

#### Returns

`Receiver<Result<()>>`


***

### stash_drop()

```rust
pub fn stash_drop(&self, index: Option<usize>, cx: &Context<'_, Self>) -> Receiver<Result<()>>
```

Defined in: [`packages/project/src/git_store/mod.rs:8497`](../../../../packages/project/src/git_store/mod.rs#L8497)

#### Parameters

##### index

`Option<usize>`

##### cx

`&Context<'_, Self>`

#### Returns

`Receiver<Result<()>>`


***

### run_hook()

```rust
pub fn run_hook(&self, hook: RunHook, _cx: &App) -> Receiver<Result<()>>
```

Defined in: [`packages/project/src/git_store/mod.rs:8558`](../../../../packages/project/src/git_store/mod.rs#L8558)

#### Parameters

##### hook

`RunHook`

##### _cx

`&App`

#### Returns

`Receiver<Result<()>>`


***

### commit()

```rust
pub fn commit(&self, message: SharedString, name_and_email: Option<(SharedString, SharedString)>, options: CommitOptions, askpass: AskPassDelegate, _cx: &App) -> Receiver<Result<()>>
```

Defined in: [`packages/project/src/git_store/mod.rs:8586`](../../../../packages/project/src/git_store/mod.rs#L8586)

#### Parameters

##### message

`SharedString`

##### name_and_email

`Option<(SharedString, SharedString)>`

##### options

`CommitOptions`

##### askpass

`AskPassDelegate`

##### _cx

`&App`

#### Returns

`Receiver<Result<()>>`


***

### unshallow_state()

```rust
pub fn unshallow_state(&self) -> UnshallowState
```

Defined in: [`packages/project/src/git_store/mod.rs:8674`](../../../../packages/project/src/git_store/mod.rs#L8674)

#### Returns

[`UnshallowState`](../enums/UnshallowState.md)


***

### fetch_unshallow()

```rust
pub fn fetch_unshallow(&self, askpass: AskPassDelegate, cx: &Context<'_, Self>) -> Receiver<Result<RemoteCommandOutput>>
```

Defined in: [`packages/project/src/git_store/mod.rs:8678`](../../../../packages/project/src/git_store/mod.rs#L8678)

#### Parameters

##### askpass

`AskPassDelegate`

##### cx

`&Context<'_, Self>`

#### Returns

`Receiver<Result<RemoteCommandOutput>>`


***

### fetch()

```rust
pub fn fetch(&self, fetch_options: FetchOptions, askpass: AskPassDelegate, cx: &Context<'_, Self>) -> Receiver<Result<RemoteCommandOutput>>
```

Defined in: [`packages/project/src/git_store/mod.rs:8707`](../../../../packages/project/src/git_store/mod.rs#L8707)

#### Parameters

##### fetch_options

`FetchOptions`

##### askpass

`AskPassDelegate`

##### cx

`&Context<'_, Self>`

#### Returns

`Receiver<Result<RemoteCommandOutput>>`


***

### push()

```rust
pub fn push(&self, branch: SharedString, remote_branch: SharedString, remote: SharedString, options: Option<PushOptions>, askpass: AskPassDelegate, cx: &Context<'_, Self>) -> Receiver<Result<RemoteCommandOutput>>
```

Defined in: [`packages/project/src/git_store/mod.rs:8769`](../../../../packages/project/src/git_store/mod.rs#L8769)

#### Parameters

##### branch

`SharedString`

##### remote_branch

`SharedString`

##### remote

`SharedString`

##### options

`Option<PushOptions>`

##### askpass

`AskPassDelegate`

##### cx

`&Context<'_, Self>`

#### Returns

`Receiver<Result<RemoteCommandOutput>>`


***

### pull()

```rust
pub fn pull(&self, branch: Option<SharedString>, remote: SharedString, rebase: bool, askpass: AskPassDelegate, _cx: &App) -> Receiver<Result<RemoteCommandOutput>>
```

Defined in: [`packages/project/src/git_store/mod.rs:8857`](../../../../packages/project/src/git_store/mod.rs#L8857)

#### Parameters

##### branch

`Option<SharedString>`

##### remote

`SharedString`

##### rebase

`bool`

##### askpass

`AskPassDelegate`

##### _cx

`&App`

#### Returns

`Receiver<Result<RemoteCommandOutput>>`


***

### create_remote()

```rust
pub fn create_remote(&self, remote_name: String, remote_url: String) -> Receiver<Result<()>>
```

Defined in: [`packages/project/src/git_store/mod.rs:9007`](../../../../packages/project/src/git_store/mod.rs#L9007)

#### Parameters

##### remote_name

`String`

##### remote_url

`String`

#### Returns

`Receiver<Result<()>>`


***

### remove_remote()

```rust
pub fn remove_remote(&self, remote_name: String) -> Receiver<Result<()>>
```

Defined in: [`packages/project/src/git_store/mod.rs:9038`](../../../../packages/project/src/git_store/mod.rs#L9038)

#### Parameters

##### remote_name

`String`

#### Returns

`Receiver<Result<()>>`


***

### get_remotes()

```rust
pub fn get_remotes(&self, branch_name: Option<String>, is_push: bool) -> Receiver<Result<Vec<Remote>>>
```

Defined in: [`packages/project/src/git_store/mod.rs:9064`](../../../../packages/project/src/git_store/mod.rs#L9064)

#### Parameters

##### branch_name

`Option<String>`

##### is_push

`bool`

#### Returns

`Receiver<Result<Vec<Remote>>>`


***

### remote_urls()

```rust
pub fn remote_urls(&self) -> Receiver<Result<HashMap<String, String>>>
```

Defined in: [`packages/project/src/git_store/mod.rs:9112`](../../../../packages/project/src/git_store/mod.rs#L9112)

#### Returns

`Receiver<Result<HashMap<String, String>>>`


***

### branches()

```rust
pub fn branches(&self) -> Receiver<Result<BranchesScanResult>>
```

Defined in: [`packages/project/src/git_store/mod.rs:9139`](../../../../packages/project/src/git_store/mod.rs#L9139)

#### Returns

`Receiver<Result<BranchesScanResult>>`


***

### linked_worktree_path()

```rust
pub fn linked_worktree_path(&self) -> Option<&Arc<Path>>
```

Defined in: [`packages/project/src/git_store/mod.rs:9173`](../../../../packages/project/src/git_store/mod.rs#L9173)

If this is a linked worktree (*NOT* the main checkout of a repository),
returns the path for the linked worktree.

Returns None if this is the main checkout.

#### Returns

`Option<&Arc<Path>>`


***

### path_for_new_linked_worktree()

```rust
pub fn path_for_new_linked_worktree(&self, branch_name: &str, worktree_directory_setting: &str) -> Result<PathBuf>
```

Defined in: [`packages/project/src/git_store/mod.rs:9186`](../../../../packages/project/src/git_store/mod.rs#L9186)

#### Parameters

##### branch_name

`&str`

##### worktree_directory_setting

`&str`

#### Returns

`Result<PathBuf>`


***

### worktrees()

```rust
pub fn worktrees(&self) -> Receiver<Result<Vec<GitWorktree>>>
```

Defined in: [`packages/project/src/git_store/mod.rs:9205`](../../../../packages/project/src/git_store/mod.rs#L9205)

#### Returns

`Receiver<Result<Vec<GitWorktree>>>`


***

### create_worktree()

```rust
pub fn create_worktree(&self, target: CreateWorktreeTarget, path: PathBuf) -> Receiver<Result<()>>
```

Defined in: [`packages/project/src/git_store/mod.rs:9232`](../../../../packages/project/src/git_store/mod.rs#L9232)

#### Parameters

##### target

`CreateWorktreeTarget`

##### path

`PathBuf`

#### Returns

`Receiver<Result<()>>`


***

### worktree_created_at()

```rust
pub fn worktree_created_at(&self, worktree_path: PathBuf) -> Receiver<Result<Option<SystemTime>>>
```

Defined in: [`packages/project/src/git_store/mod.rs:9284`](../../../../packages/project/src/git_store/mod.rs#L9284)

Returns the creation time of a linked worktree's git metadata
directory. See `GitRepository::worktree_created_at`. For remote
projects the stat runs on the remote host, where the worktree's
filesystem lives.

#### Parameters

##### worktree_path

`PathBuf`

#### Returns

`Receiver<Result<Option<SystemTime>>>`


***

### create_worktree_detached()

```rust
pub fn create_worktree_detached(&self, path: PathBuf, commit: String) -> Receiver<Result<()>>
```

Defined in: [`packages/project/src/git_store/mod.rs:9308`](../../../../packages/project/src/git_store/mod.rs#L9308)

#### Parameters

##### path

`PathBuf`

##### commit

`String`

#### Returns

`Receiver<Result<()>>`


***

### checkout_branch_in_worktree()

```rust
pub fn checkout_branch_in_worktree(&self, branch_name: String, worktree_path: PathBuf, create: bool) -> Receiver<Result<()>>
```

Defined in: [`packages/project/src/git_store/mod.rs:9321`](../../../../packages/project/src/git_store/mod.rs#L9321)

#### Parameters

##### branch_name

`String`

##### worktree_path

`PathBuf`

##### create

`bool`

#### Returns

`Receiver<Result<()>>`


***

### head_sha()

```rust
pub fn head_sha(&self) -> Receiver<Result<Option<String>>>
```

Defined in: [`packages/project/src/git_store/mod.rs:9353`](../../../../packages/project/src/git_store/mod.rs#L9353)

#### Returns

`Receiver<Result<Option<String>>>`


***

### update_ref()

```rust
pub fn update_ref(&self, ref_name: String, commit: String) -> Receiver<Result<()>>
```

Defined in: [`packages/project/src/git_store/mod.rs:9407`](../../../../packages/project/src/git_store/mod.rs#L9407)

#### Parameters

##### ref_name

`String`

##### commit

`String`

#### Returns

`Receiver<Result<()>>`


***

### delete_ref()

```rust
pub fn delete_ref(&self, ref_name: String) -> Receiver<Result<()>>
```

Defined in: [`packages/project/src/git_store/mod.rs:9415`](../../../../packages/project/src/git_store/mod.rs#L9415)

#### Parameters

##### ref_name

`String`

#### Returns

`Receiver<Result<()>>`


***

### repair_worktrees()

```rust
pub fn repair_worktrees(&self) -> Receiver<Result<()>>
```

Defined in: [`packages/project/src/git_store/mod.rs:9419`](../../../../packages/project/src/git_store/mod.rs#L9419)

#### Returns

`Receiver<Result<()>>`


***

### create_archive_checkpoint()

```rust
pub fn create_archive_checkpoint(&self) -> Receiver<Result<(String, String)>>
```

Defined in: [`packages/project/src/git_store/mod.rs:9439`](../../../../packages/project/src/git_store/mod.rs#L9439)

#### Returns

`Receiver<Result<(String, String)>>`


***

### restore_archive_checkpoint()

```rust
pub fn restore_archive_checkpoint(&self, staged_sha: String, unstaged_sha: String) -> Receiver<Result<()>>
```

Defined in: [`packages/project/src/git_store/mod.rs:9463`](../../../../packages/project/src/git_store/mod.rs#L9463)

#### Parameters

##### staged_sha

`String`

##### unstaged_sha

`String`

#### Returns

`Receiver<Result<()>>`


***

### remove_worktree()

```rust
pub fn remove_worktree(&self, path: PathBuf, force: bool) -> Receiver<Result<()>>
```

Defined in: [`packages/project/src/git_store/mod.rs:9495`](../../../../packages/project/src/git_store/mod.rs#L9495)

#### Parameters

##### path

`PathBuf`

##### force

`bool`

#### Returns

`Receiver<Result<()>>`


***

### rename_worktree()

```rust
pub fn rename_worktree(&self, old_path: PathBuf, new_path: PathBuf) -> Receiver<Result<()>>
```

Defined in: [`packages/project/src/git_store/mod.rs:9577`](../../../../packages/project/src/git_store/mod.rs#L9577)

#### Parameters

##### old_path

`PathBuf`

##### new_path

`PathBuf`

#### Returns

`Receiver<Result<()>>`


***

### default_branch()

```rust
pub fn default_branch(&self, include_remote_name: bool) -> Receiver<Result<Option<SharedString>>>
```

Defined in: [`packages/project/src/git_store/mod.rs:9608`](../../../../packages/project/src/git_store/mod.rs#L9608)

#### Parameters

##### include_remote_name

`bool`

#### Returns

`Receiver<Result<Option<SharedString>>>`


***

### diff_tree()

```rust
pub fn diff_tree(&self, diff_type: DiffTreeType, _cx: &App) -> Receiver<Result<TreeDiff>>
```

Defined in: [`packages/project/src/git_store/mod.rs:9633`](../../../../packages/project/src/git_store/mod.rs#L9633)

#### Parameters

##### diff_type

`DiffTreeType`

##### _cx

`&App`

#### Returns

`Receiver<Result<TreeDiff>>`


***

### diff()

```rust
pub fn diff(&self, diff_type: DiffType, _cx: &App) -> Receiver<Result<String>>
```

Defined in: [`packages/project/src/git_store/mod.rs:9703`](../../../../packages/project/src/git_store/mod.rs#L9703)

#### Parameters

##### diff_type

`DiffType`

##### _cx

`&App`

#### Returns

`Receiver<Result<String>>`


***

### create_branch()

```rust
pub fn create_branch(&self, branch_name: String, base_branch: Option<String>) -> Receiver<Result<()>>
```

Defined in: [`packages/project/src/git_store/mod.rs:9738`](../../../../packages/project/src/git_store/mod.rs#L9738)

#### Parameters

##### branch_name

`String`

##### base_branch

`Option<String>`

#### Returns

`Receiver<Result<()>>`


***

### change_branch()

```rust
pub fn change_branch(&self, branch_name: String) -> Receiver<Result<()>>
```

Defined in: [`packages/project/src/git_store/mod.rs:9774`](../../../../packages/project/src/git_store/mod.rs#L9774)

#### Parameters

##### branch_name

`String`

#### Returns

`Receiver<Result<()>>`


***

### delete_branch()

```rust
pub fn delete_branch(&self, is_remote: bool, branch_name: String, force: bool) -> Receiver<Result<()>>
```

Defined in: [`packages/project/src/git_store/mod.rs:9800`](../../../../packages/project/src/git_store/mod.rs#L9800)

#### Parameters

##### is_remote

`bool`

##### branch_name

`String`

##### force

`bool`

#### Returns

`Receiver<Result<()>>`


***

### rename_branch()

```rust
pub fn rename_branch(&self, branch: String, new_name: String) -> Receiver<Result<()>>
```

Defined in: [`packages/project/src/git_store/mod.rs:9837`](../../../../packages/project/src/git_store/mod.rs#L9837)

#### Parameters

##### branch

`String`

##### new_name

`String`

#### Returns

`Receiver<Result<()>>`


***

### check_for_pushed_commits()

```rust
pub fn check_for_pushed_commits(&self) -> Receiver<Result<Vec<SharedString>>>
```

Defined in: [`packages/project/src/git_store/mod.rs:9868`](../../../../packages/project/src/git_store/mod.rs#L9868)

#### Returns

`Receiver<Result<Vec<SharedString>>>`


***

### checkpoint()

```rust
pub fn checkpoint(&self) -> Receiver<Result<GitRepositoryCheckpoint>>
```

Defined in: [`packages/project/src/git_store/mod.rs:9895`](../../../../packages/project/src/git_store/mod.rs#L9895)

#### Returns

`Receiver<Result<GitRepositoryCheckpoint>>`


***

### restore_checkpoint()

```rust
pub fn restore_checkpoint(&self, checkpoint: GitRepositoryCheckpoint) -> Receiver<Result<()>>
```

Defined in: [`packages/project/src/git_store/mod.rs:9918`](../../../../packages/project/src/git_store/mod.rs#L9918)

#### Parameters

##### checkpoint

`GitRepositoryCheckpoint`

#### Returns

`Receiver<Result<()>>`


***

### compare_checkpoints()

```rust
pub fn compare_checkpoints(&self, left: GitRepositoryCheckpoint, right: GitRepositoryCheckpoint) -> Receiver<Result<bool>>
```

Defined in: [`packages/project/src/git_store/mod.rs:10043`](../../../../packages/project/src/git_store/mod.rs#L10043)

#### Parameters

##### left

`GitRepositoryCheckpoint`

##### right

`GitRepositoryCheckpoint`

#### Returns

`Receiver<Result<bool>>`


***

### diff_checkpoints()

```rust
pub fn diff_checkpoints(&self, base_checkpoint: GitRepositoryCheckpoint, target_checkpoint: GitRepositoryCheckpoint) -> Receiver<Result<String>>
```

Defined in: [`packages/project/src/git_store/mod.rs:10069`](../../../../packages/project/src/git_store/mod.rs#L10069)

#### Parameters

##### base_checkpoint

`GitRepositoryCheckpoint`

##### target_checkpoint

`GitRepositoryCheckpoint`

#### Returns

`Receiver<Result<String>>`


***

### load_commit_template_text()

```rust
pub fn load_commit_template_text(&self) -> Receiver<Result<Option<GitCommitTemplate>>>
```

Defined in: [`packages/project/src/git_store/mod.rs:10343`](../../../../packages/project/src/git_store/mod.rs#L10343)

#### Returns

`Receiver<Result<Option<GitCommitTemplate>>>`


***

### blame_buffer_at_revision()

```rust
pub fn blame_buffer_at_revision(&self, path: RepoPath, revision: Oid, cx: &App) -> Task<Result<(String, Blame)>>
```

Defined in: [`packages/project/src/git_store/mod.rs:10371`](../../../../packages/project/src/git_store/mod.rs#L10371)

#### Parameters

##### path

`RepoPath`

##### revision

`Oid`

##### cx

`&App`

#### Returns

`Task<Result<(String, Blame)>>`


***

### current_job()

```rust
pub fn current_job(&self) -> Option<JobInfo>
```

Defined in: [`packages/project/src/git_store/mod.rs:10600`](../../../../packages/project/src/git_store/mod.rs#L10600)

currently running git command and when it started

#### Returns

`Option<JobInfo>`


***

### job_debug_queue()

```rust
pub fn job_debug_queue(&self) -> &GitJobDebugQueue
```

Defined in: [`packages/project/src/git_store/mod.rs:10604`](../../../../packages/project/src/git_store/mod.rs#L10604)

#### Returns

`&GitJobDebugQueue`


***

### barrier()

```rust
pub fn barrier(&self) -> Receiver<()>
```

Defined in: [`packages/project/src/git_store/mod.rs:10608`](../../../../packages/project/src/git_store/mod.rs#L10608)

#### Returns

`Receiver<()>`


***

### access()

```rust
pub fn access(&self, _cx: &App) -> Receiver<GitAccess>
```

Defined in: [`packages/project/src/git_store/mod.rs:10678`](../../../../packages/project/src/git_store/mod.rs#L10678)

#### Parameters

##### _cx

`&App`

#### Returns

`Receiver<GitAccess>`


***

### default_remote_url()

```rust
pub fn default_remote_url(&self) -> Option<String>
```

Defined in: [`packages/project/src/git_store/mod.rs:10694`](../../../../packages/project/src/git_store/mod.rs#L10694)

#### Returns

`Option<String>`

## Trait Implementations

- `impl Borrow for Repository`
- `impl BorrowMut for Repository`
- `impl Into for Repository`
- `impl From for Repository`
- `impl TryInto for Repository`
- `impl TryFrom for Repository`
- `impl Receiver for Repository`
- `impl Any for Repository`
- `impl VZip for Repository`
- `impl CastableFrom for Repository`
- `impl CastableFrom for Repository`
- `impl Read for Repository`
- `impl IntoEither for Repository`
- `impl ErasedDestructor for Repository`
- `impl Same for Repository`
- `impl Pointable for Repository`
- `impl Instrument for Repository`
- `impl WithSubscriber for Repository`
- `impl FromAngle for Repository`
- `impl IntoAngle for Repository`
- `impl IntoCam16Unclamped for Repository`
- `impl Cam16IntoUnclamped for Repository`
- `impl ArraysFrom for Repository`
- `impl ArraysInto for Repository`
- `impl ComponentsFrom for Repository`
- `impl TryComponentsInto for Repository`
- `impl UintsFrom for Repository`
- `impl UintsInto for Repository`
- `impl AdaptIntoUnclamped for Repository`
- `impl AdaptInto for Repository`
- `impl IntoColor for Repository`
- `impl IntoColorUnclamped for Repository`
- `impl TryIntoColor for Repository`
- `impl FromStimulus for Repository`
- `impl IntoStimulus for Repository`
- `impl Deref for Repository`
- `impl EventEmitter for Repository`
- `impl EventEmitter for Repository`

## Auto Trait Implementations

`Freeze` `Unpin` `UnsafeUnpin`

