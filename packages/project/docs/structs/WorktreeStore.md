---
id: WorktreeStore
title: WorktreeStore
---

# Struct: WorktreeStore

Defined in: [`packages/project/src/worktree_store.rs:207`](../../../../packages/project/src/worktree_store.rs#L207)

## Definition

```rust
pub struct WorktreeStore
```

_（存在非公开字段）_

## Implementations

### init()

```rust
pub fn init(client: &AnyProtoClient)
```

Defined in: [`packages/project/src/worktree_store.rs:237`](../../../../packages/project/src/worktree_store.rs#L237)

#### Parameters

##### client

`&AnyProtoClient`


***

### init_remote()

```rust
pub fn init_remote(client: &AnyProtoClient)
```

Defined in: [`packages/project/src/worktree_store.rs:247`](../../../../packages/project/src/worktree_store.rs#L247)

#### Parameters

##### client

`&AnyProtoClient`


***

### local()

```rust
pub fn local(retain_worktrees: bool, fs: Arc<dyn Fs>, next_worktree_id: WorktreeIdCounter) -> Self
```

Defined in: [`packages/project/src/worktree_store.rs:251`](../../../../packages/project/src/worktree_store.rs#L251)

#### Parameters

##### retain_worktrees

`bool`

##### fs

`Arc<dyn Fs>`

##### next_worktree_id

[`WorktreeIdCounter`](WorktreeIdCounter.md)

#### Returns

`Self`


***

### remote()

```rust
pub fn remote(retain_worktrees: bool, upstream_client: AnyProtoClient, upstream_project_id: u64, path_style: PathStyle, next_worktree_id: WorktreeIdCounter) -> Self
```

Defined in: [`packages/project/src/worktree_store.rs:269`](../../../../packages/project/src/worktree_store.rs#L269)

#### Parameters

##### retain_worktrees

`bool`

##### upstream_client

`AnyProtoClient`

##### upstream_project_id

`u64`

##### path_style

`PathStyle`

##### next_worktree_id

[`WorktreeIdCounter`](WorktreeIdCounter.md)

#### Returns

`Self`


***

### next_worktree_id()

```rust
pub fn next_worktree_id(&self) -> impl ? + ?
```

Defined in: [`packages/project/src/worktree_store.rs:293`](../../../../packages/project/src/worktree_store.rs#L293)

#### Returns

`impl ? + ?`


***

### disable_scanner()

```rust
pub fn disable_scanner(&self)
```

Defined in: [`packages/project/src/worktree_store.rs:318`](../../../../packages/project/src/worktree_store.rs#L318)


***

### wait_for_initial_scan()

```rust
pub fn wait_for_initial_scan(&self) -> impl ? + ?
```

Defined in: [`packages/project/src/worktree_store.rs:325`](../../../../packages/project/src/worktree_store.rs#L325)

Returns a future that resolves when all visible worktrees have completed
their initial scan (entries populated, git repos detected).

#### Returns

`impl ? + ?`


***

### initial_scan_completed()

```rust
pub fn initial_scan_completed(&self) -> bool
```

Defined in: [`packages/project/src/worktree_store.rs:340`](../../../../packages/project/src/worktree_store.rs#L340)

Returns whether all visible worktrees have completed their initial scan.

#### Returns

`bool`


***

### diagnostics()

```rust
pub fn diagnostics(&self, cx: &App) -> WorktreeStoreDiagnostics
```

Defined in: [`packages/project/src/worktree_store.rs:374`](../../../../packages/project/src/worktree_store.rs#L374)

Returns worktree ownership and current snapshot size diagnostics.

#### Parameters

##### cx

`&App`

#### Returns

[`WorktreeStoreDiagnostics`](WorktreeStoreDiagnostics.md)


***

### worktrees()

```rust
pub fn worktrees(&self) -> impl ? + ?
```

Defined in: [`packages/project/src/worktree_store.rs:425`](../../../../packages/project/src/worktree_store.rs#L425)

Iterates through all worktrees, including ones that don't appear in the project panel

#### Returns

`impl ? + ?`


***

### visible_worktrees()

```rust
pub fn visible_worktrees<'a>(&self, cx: &'a App) -> impl ? + ?
```

Defined in: [`packages/project/src/worktree_store.rs:432`](../../../../packages/project/src/worktree_store.rs#L432)

Iterates through all user-visible worktrees, the ones that appear in the project panel.

#### Parameters

##### cx

`&'a App`

#### Returns

`impl ? + ?`


***

### visible_worktrees_and_single_files()

```rust
pub fn visible_worktrees_and_single_files<'a>(&self, cx: &'a App) -> impl ? + ?
```

Defined in: [`packages/project/src/worktree_store.rs:441`](../../../../packages/project/src/worktree_store.rs#L441)

Iterates through all user-visible worktrees (directories and files that appear in the project panel) and other, invisible single files that could appear e.g. due to drag and drop.

#### Parameters

##### cx

`&'a App`

#### Returns

`impl ? + ?`


***

### worktree_for_id()

```rust
pub fn worktree_for_id(&self, id: WorktreeId, cx: &App) -> Option<Entity<Worktree>>
```

Defined in: [`packages/project/src/worktree_store.rs:449`](../../../../packages/project/src/worktree_store.rs#L449)

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

Defined in: [`packages/project/src/worktree_store.rs:454`](../../../../packages/project/src/worktree_store.rs#L454)

#### Parameters

##### entry_id

`ProjectEntryId`

##### cx

`&App`

#### Returns

`Option<Entity<Worktree>>`


***

### find_worktree()

```rust
pub fn find_worktree<impl AsRef<Path>: AsRef>(&self, abs_path: impl ?, cx: &App) -> Option<(Entity<Worktree>, Arc<RelPath>)>
```

Defined in: [`packages/project/src/worktree_store.rs:463`](../../../../packages/project/src/worktree_store.rs#L463)

#### Parameters

##### abs_path

`impl ?`

##### cx

`&App`

#### Returns

`Option<(Entity<Worktree>, Arc<RelPath>)>`


***

### project_path_for_absolute_path()

```rust
pub fn project_path_for_absolute_path(&self, abs_path: &Path, cx: &App) -> Option<ProjectPath>
```

Defined in: [`packages/project/src/worktree_store.rs:480`](../../../../packages/project/src/worktree_store.rs#L480)

#### Parameters

##### abs_path

`&Path`

##### cx

`&App`

#### Returns

`Option<ProjectPath>`


***

### absolutize()

```rust
pub fn absolutize(&self, project_path: &ProjectPath, cx: &App) -> Option<PathBuf>
```

Defined in: [`packages/project/src/worktree_store.rs:488`](../../../../packages/project/src/worktree_store.rs#L488)

#### Parameters

##### project_path

`&ProjectPath`

##### cx

`&App`

#### Returns

`Option<PathBuf>`


***

### path_style()

```rust
pub fn path_style(&self) -> PathStyle
```

Defined in: [`packages/project/src/worktree_store.rs:493`](../../../../packages/project/src/worktree_store.rs#L493)

#### Returns

`PathStyle`


***

### find_or_create_worktree()

```rust
pub fn find_or_create_worktree<impl AsRef<Path>: AsRef>(&self, abs_path: impl ?, visible: bool, cx: &Context<'_, Self>) -> Task<Result<(Entity<Worktree>, Arc<RelPath>)>>
```

Defined in: [`packages/project/src/worktree_store.rs:500`](../../../../packages/project/src/worktree_store.rs#L500)

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

### entry_for_id()

```rust
pub fn entry_for_id<'a>(&self, entry_id: ProjectEntryId, cx: &'a App) -> Option<&'a Entry>
```

Defined in: [`packages/project/src/worktree_store.rs:515`](../../../../packages/project/src/worktree_store.rs#L515)

#### Parameters

##### entry_id

`ProjectEntryId`

##### cx

`&'a App`

#### Returns

`Option<&'a Entry>`


***

### worktree_and_entry_for_id()

```rust
pub fn worktree_and_entry_for_id<'a>(&self, entry_id: ProjectEntryId, cx: &'a App) -> Option<(Entity<Worktree>, &'a Entry)>
```

Defined in: [`packages/project/src/worktree_store.rs:520`](../../../../packages/project/src/worktree_store.rs#L520)

#### Parameters

##### entry_id

`ProjectEntryId`

##### cx

`&'a App`

#### Returns

`Option<(Entity<Worktree>, &'a Entry)>`


***

### entry_for_path()

```rust
pub fn entry_for_path<'a>(&self, path: &ProjectPath, cx: &'a App) -> Option<&'a Entry>
```

Defined in: [`packages/project/src/worktree_store.rs:533`](../../../../packages/project/src/worktree_store.rs#L533)

#### Parameters

##### path

`&ProjectPath`

##### cx

`&'a App`

#### Returns

`Option<&'a Entry>`


***

### copy_entry()

```rust
pub fn copy_entry(&self, entry_id: ProjectEntryId, new_project_path: ProjectPath, cx: &Context<'_, Self>) -> Task<Result<Option<Entry>>>
```

Defined in: [`packages/project/src/worktree_store.rs:539`](../../../../packages/project/src/worktree_store.rs#L539)

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
pub fn rename_entry(&self, entry_id: ProjectEntryId, new_project_path: ProjectPath, cx: &Context<'_, Self>) -> Task<Result<CreatedEntry>>
```

Defined in: [`packages/project/src/worktree_store.rs:614`](../../../../packages/project/src/worktree_store.rs#L614)

#### Parameters

##### entry_id

`ProjectEntryId`

##### new_project_path

[`ProjectPath`](ProjectPath.md)

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<CreatedEntry>>`


***

### create_worktree()

```rust
pub fn create_worktree<impl AsRef<Path>: AsRef>(&self, abs_path: impl ?, visible: bool, cx: &Context<'_, Self>) -> Task<Result<Entity<Worktree>>>
```

Defined in: [`packages/project/src/worktree_store.rs:773`](../../../../packages/project/src/worktree_store.rs#L773)

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

### add()

```rust
pub fn add(&self, worktree: &Entity<Worktree>, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/worktree_store.rs:960`](../../../../packages/project/src/worktree_store.rs#L960)

#### Parameters

##### worktree

`&Entity<Worktree>`

##### cx

`&Context<'_, Self>`


***

### remove_worktree()

```rust
pub fn remove_worktree(&self, id_to_remove: WorktreeId, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/worktree_store.rs:1020`](../../../../packages/project/src/worktree_store.rs#L1020)

#### Parameters

##### id_to_remove

`WorktreeId`

##### cx

`&Context<'_, Self>`


***

### worktree_for_main_worktree_path()

```rust
pub fn worktree_for_main_worktree_path(&self, path: &Path, cx: &App) -> Option<Entity<Worktree>>
```

Defined in: [`packages/project/src/worktree_store.rs:1040`](../../../../packages/project/src/worktree_store.rs#L1040)

#### Parameters

##### path

`&Path`

##### cx

`&App`

#### Returns

`Option<Entity<Worktree>>`


***

### set_worktrees_from_proto()

```rust
pub fn set_worktrees_from_proto(&self, worktrees: Vec<WorktreeMetadata>, replica_id: ReplicaId, cx: &Context<'_, Self>) -> Result<()>
```

Defined in: [`packages/project/src/worktree_store.rs:1066`](../../../../packages/project/src/worktree_store.rs#L1066)

#### Parameters

##### worktrees

`Vec<WorktreeMetadata>`

##### replica_id

`ReplicaId`

##### cx

`&Context<'_, Self>`

#### Returns

`Result<()>`


***

### move_worktree()

```rust
pub fn move_worktree(&self, source: WorktreeId, destination: WorktreeId, cx: &Context<'_, Self>) -> Result<()>
```

Defined in: [`packages/project/src/worktree_store.rs:1114`](../../../../packages/project/src/worktree_store.rs#L1114)

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

### disconnected_from_host()

```rust
pub fn disconnected_from_host(&self, cx: &App)
```

Defined in: [`packages/project/src/worktree_store.rs:1159`](../../../../packages/project/src/worktree_store.rs#L1159)

#### Parameters

##### cx

`&App`


***

### send_project_updates()

```rust
pub fn send_project_updates(&self, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/worktree_store.rs:1171`](../../../../packages/project/src/worktree_store.rs#L1171)

#### Parameters

##### cx

`&Context<'_, Self>`


***

### worktree_metadata_protos()

```rust
pub fn worktree_metadata_protos(&self, cx: &App) -> Vec<WorktreeMetadata>
```

Defined in: [`packages/project/src/worktree_store.rs:1225`](../../../../packages/project/src/worktree_store.rs#L1225)

#### Parameters

##### cx

`&App`

#### Returns

`Vec<WorktreeMetadata>`


***

### shared()

```rust
pub fn shared(&self, remote_id: u64, downstream_client: AnyProtoClient, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/worktree_store.rs:1243`](../../../../packages/project/src/worktree_store.rs#L1243)

#### Parameters

##### remote_id

`u64`

##### downstream_client

`AnyProtoClient`

##### cx

`&Context<'_, Self>`


***

### unshared()

```rust
pub fn unshared(&self, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/worktree_store.rs:1272`](../../../../packages/project/src/worktree_store.rs#L1272)

#### Parameters

##### cx

`&Context<'_, Self>`


***

### handle_create_project_entry()

```rust
pub async fn handle_create_project_entry(this: Entity<Self>, envelope: TypedEnvelope<CreateProjectEntry>, cx: AsyncApp) -> Result<ProjectEntryResponse>
```

Defined in: [`packages/project/src/worktree_store.rs:1290`](../../../../packages/project/src/worktree_store.rs#L1290)

#### Parameters

##### this

`Entity<Self>`

##### envelope

`TypedEnvelope<CreateProjectEntry>`

##### cx

`AsyncApp`

#### Returns

`Result<ProjectEntryResponse>`


***

### handle_copy_project_entry()

```rust
pub async fn handle_copy_project_entry(this: Entity<Self>, envelope: TypedEnvelope<CopyProjectEntry>, cx: AsyncApp) -> Result<ProjectEntryResponse>
```

Defined in: [`packages/project/src/worktree_store.rs:1303`](../../../../packages/project/src/worktree_store.rs#L1303)

#### Parameters

##### this

`Entity<Self>`

##### envelope

`TypedEnvelope<CopyProjectEntry>`

##### cx

`AsyncApp`

#### Returns

`Result<ProjectEntryResponse>`


***

### handle_trash_project_entry()

```rust
pub async fn handle_trash_project_entry(this: Entity<Self>, envelope: TypedEnvelope<TrashProjectEntry>, cx: AsyncApp) -> Result<TrashProjectEntryResponse>
```

Defined in: [`packages/project/src/worktree_store.rs:1341`](../../../../packages/project/src/worktree_store.rs#L1341)

#### Parameters

##### this

`Entity<Self>`

##### envelope

`TypedEnvelope<TrashProjectEntry>`

##### cx

`AsyncApp`

#### Returns

`Result<TrashProjectEntryResponse>`


***

### handle_delete_project_entry()

```rust
pub async fn handle_delete_project_entry(this: Entity<Self>, envelope: TypedEnvelope<DeleteProjectEntry>, cx: AsyncApp) -> Result<ProjectEntryResponse>
```

Defined in: [`packages/project/src/worktree_store.rs:1363`](../../../../packages/project/src/worktree_store.rs#L1363)

#### Parameters

##### this

`Entity<Self>`

##### envelope

`TypedEnvelope<DeleteProjectEntry>`

##### cx

`AsyncApp`

#### Returns

`Result<ProjectEntryResponse>`


***

### handle_restore_project_entry()

```rust
pub async fn handle_restore_project_entry(this: Entity<Self>, envelope: TypedEnvelope<RestoreProjectEntry>, cx: AsyncApp) -> Result<RestoreProjectEntryResponse>
```

Defined in: [`packages/project/src/worktree_store.rs:1385`](../../../../packages/project/src/worktree_store.rs#L1385)

#### Parameters

##### this

`Entity<Self>`

##### envelope

`TypedEnvelope<RestoreProjectEntry>`

##### cx

`AsyncApp`

#### Returns

`Result<RestoreProjectEntryResponse>`


***

### handle_rename_project_entry()

```rust
pub async fn handle_rename_project_entry(this: Entity<Self>, request: RenameProjectEntry, cx: AsyncApp) -> Result<ProjectEntryResponse>
```

Defined in: [`packages/project/src/worktree_store.rs:1400`](../../../../packages/project/src/worktree_store.rs#L1400)

#### Parameters

##### this

`Entity<Self>`

##### request

`RenameProjectEntry`

##### cx

`AsyncApp`

#### Returns

`Result<ProjectEntryResponse>`


***

### handle_expand_project_entry()

```rust
pub async fn handle_expand_project_entry(this: Entity<Self>, envelope: TypedEnvelope<ExpandProjectEntry>, cx: AsyncApp) -> Result<ExpandProjectEntryResponse>
```

Defined in: [`packages/project/src/worktree_store.rs:1441`](../../../../packages/project/src/worktree_store.rs#L1441)

#### Parameters

##### this

`Entity<Self>`

##### envelope

`TypedEnvelope<ExpandProjectEntry>`

##### cx

`AsyncApp`

#### Returns

`Result<ExpandProjectEntryResponse>`


***

### handle_expand_all_for_project_entry()

```rust
pub async fn handle_expand_all_for_project_entry(this: Entity<Self>, envelope: TypedEnvelope<ExpandAllForProjectEntry>, cx: AsyncApp) -> Result<ExpandAllForProjectEntryResponse>
```

Defined in: [`packages/project/src/worktree_store.rs:1453`](../../../../packages/project/src/worktree_store.rs#L1453)

#### Parameters

##### this

`Entity<Self>`

##### envelope

`TypedEnvelope<ExpandAllForProjectEntry>`

##### cx

`AsyncApp`

#### Returns

`Result<ExpandAllForProjectEntryResponse>`


***

### handle_allocate_worktree_id()

```rust
pub async fn handle_allocate_worktree_id(_this: Entity<Self>, _envelope: TypedEnvelope<AllocateWorktreeId>, cx: AsyncApp) -> Result<AllocateWorktreeIdResponse>
```

Defined in: [`packages/project/src/worktree_store.rs:1465`](../../../../packages/project/src/worktree_store.rs#L1465)

#### Parameters

##### _this

`Entity<Self>`

##### _envelope

`TypedEnvelope<AllocateWorktreeId>`

##### cx

`AsyncApp`

#### Returns

`Result<AllocateWorktreeIdResponse>`


***

### fs()

```rust
pub fn fs(&self) -> Option<Arc<dyn Fs>>
```

Defined in: [`packages/project/src/worktree_store.rs:1474`](../../../../packages/project/src/worktree_store.rs#L1474)

#### Returns

`Option<Arc<dyn Fs>>`


***

### paths()

```rust
pub fn paths(&self, cx: &App) -> WorktreePaths
```

Defined in: [`packages/project/src/worktree_store.rs:1481`](../../../../packages/project/src/worktree_store.rs#L1481)

#### Parameters

##### cx

`&App`

#### Returns

[`WorktreePaths`](WorktreePaths.md)

## Trait Implementations

- `impl Borrow for WorktreeStore`
- `impl BorrowMut for WorktreeStore`
- `impl Into for WorktreeStore`
- `impl From for WorktreeStore`
- `impl TryInto for WorktreeStore`
- `impl TryFrom for WorktreeStore`
- `impl Any for WorktreeStore`
- `impl VZip for WorktreeStore`
- `impl CastableFrom for WorktreeStore`
- `impl CastableFrom for WorktreeStore`
- `impl Read for WorktreeStore`
- `impl IntoEither for WorktreeStore`
- `impl ErasedDestructor for WorktreeStore`
- `impl Same for WorktreeStore`
- `impl Pointable for WorktreeStore`
- `impl Instrument for WorktreeStore`
- `impl WithSubscriber for WorktreeStore`
- `impl FromAngle for WorktreeStore`
- `impl IntoAngle for WorktreeStore`
- `impl IntoCam16Unclamped for WorktreeStore`
- `impl Cam16IntoUnclamped for WorktreeStore`
- `impl ArraysFrom for WorktreeStore`
- `impl ArraysInto for WorktreeStore`
- `impl ComponentsFrom for WorktreeStore`
- `impl TryComponentsInto for WorktreeStore`
- `impl UintsFrom for WorktreeStore`
- `impl UintsInto for WorktreeStore`
- `impl AdaptIntoUnclamped for WorktreeStore`
- `impl AdaptInto for WorktreeStore`
- `impl IntoColor for WorktreeStore`
- `impl IntoColorUnclamped for WorktreeStore`
- `impl TryIntoColor for WorktreeStore`
- `impl FromStimulus for WorktreeStore`
- `impl IntoStimulus for WorktreeStore`
- `impl EventEmitter for WorktreeStore`

## Auto Trait Implementations

`Send` `Sync` `Unpin` `UnsafeUnpin`

