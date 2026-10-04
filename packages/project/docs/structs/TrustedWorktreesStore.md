---
id: TrustedWorktreesStore
title: TrustedWorktreesStore
---

# Struct: TrustedWorktreesStore

Defined in: [`packages/project/src/trusted_worktrees.rs:135`](../../../../packages/project/src/trusted_worktrees.rs#L135)

A collection of worktrees that are considered trusted and not trusted.
This can be used when checking for this criteria before enabling certain features.

Emits an event each time the worktree was checked and found not trusted,
or a certain worktree had been trusted.

## Definition

```rust
pub struct TrustedWorktreesStore
```

_（存在非公开字段）_

## Implementations

### has_restricted_worktrees()

```rust
pub fn has_restricted_worktrees(&self, worktree_store: &Entity<WorktreeStore>, cx: &App) -> bool
```

Defined in: [`packages/project/src/trusted_worktrees.rs:249`](../../../../packages/project/src/trusted_worktrees.rs#L249)

Whether a particular worktree store has associated worktrees that are restricted, or an associated host is restricted.

#### Parameters

##### worktree_store

`&Entity<WorktreeStore>`

##### cx

`&App`

#### Returns

`bool`


***

### trust()

```rust
pub fn trust(&self, worktree_store: &Entity<WorktreeStore>, trusted_paths: HashSet<PathTrust>, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/trusted_worktrees.rs:280`](../../../../packages/project/src/trusted_worktrees.rs#L280)

Adds certain entities on this host to the trusted list.
This will emit [`TrustedWorktreesEvent::Trusted`](../enums/TrustedWorktreesEvent.md) event for all passed entries
and the ones that got auto trusted based on trust hierarchy (see module-level docs).

#### Parameters

##### worktree_store

`&Entity<WorktreeStore>`

##### trusted_paths

`HashSet<PathTrust>`

##### cx

`&Context<'_, Self>`


***

### restrict()

```rust
pub fn restrict(&self, worktree_store: WeakEntity<WorktreeStore>, restricted_paths: HashSet<PathTrust>, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/trusted_worktrees.rs:426`](../../../../packages/project/src/trusted_worktrees.rs#L426)

Restricts certain entities on this host.
This will emit [`TrustedWorktreesEvent::Restricted`](../enums/TrustedWorktreesEvent.md) event for all passed entries.

#### Parameters

##### worktree_store

`WeakEntity<WorktreeStore>`

##### restricted_paths

`HashSet<PathTrust>`

##### cx

`&Context<'_, Self>`


***

### clear_trusted_paths()

```rust
pub fn clear_trusted_paths(&self)
```

Defined in: [`packages/project/src/trusted_worktrees.rs:454`](../../../../packages/project/src/trusted_worktrees.rs#L454)

Erases all trust information.
Requires Zed's restart to take proper effect.


***

### can_trust()

```rust
pub fn can_trust(&self, worktree_store: &Entity<WorktreeStore>, worktree_id: WorktreeId, cx: &Context<'_, Self>) -> bool
```

Defined in: [`packages/project/src/trusted_worktrees.rs:463`](../../../../packages/project/src/trusted_worktrees.rs#L463)

Checks whether a certain worktree is trusted (or on a larger trust level).
If not, emits [`TrustedWorktreesEvent::Restricted`](../enums/TrustedWorktreesEvent.md) event if for the first time and not trusted, or no corresponding worktree store was found.

No events or data adjustment happens when `trust_all_worktrees` auto trust is enabled.

#### Parameters

##### worktree_store

`&Entity<WorktreeStore>`

##### worktree_id

`WorktreeId`

##### cx

`&Context<'_, Self>`

#### Returns

`bool`


***

### restricted_worktrees()

```rust
pub fn restricted_worktrees(&self, worktree_store: &Entity<WorktreeStore>, cx: &App) -> HashSet<(WorktreeId, Arc<Path>)>
```

Defined in: [`packages/project/src/trusted_worktrees.rs:557`](../../../../packages/project/src/trusted_worktrees.rs#L557)

Lists all explicitly restricted worktrees (via [`TrustedWorktreesStore::can_trust`](TrustedWorktreesStore.md) method calls) for a particular worktree store on a particular host.

#### Parameters

##### worktree_store

`&Entity<WorktreeStore>`

##### cx

`&App`

#### Returns

`HashSet<(WorktreeId, Arc<Path>)>`


***

### auto_trust_all()

```rust
pub fn auto_trust_all(&self, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/trusted_worktrees.rs:593`](../../../../packages/project/src/trusted_worktrees.rs#L593)

Switches the "trust nothing" mode to "automatically trust everything".
This does not influence already persisted data, but stops adding new worktrees there.

#### Parameters

##### cx

`&Context<'_, Self>`


***

### schedule_serialization()

```rust
pub fn schedule_serialization<S>(&self, cx: &Context<'_, Self>, serialize: S)
where
    S: FnOnce(HashMap<Option<RemoteHostLocation>, HashSet<PathBuf>>, &App) -> Task<()> + 'static
```

Defined in: [`packages/project/src/trusted_worktrees.rs:609`](../../../../packages/project/src/trusted_worktrees.rs#L609)

#### Parameters

##### cx

`&Context<'_, Self>`

##### serialize

`S`

## Trait Implementations

- `impl Borrow for TrustedWorktreesStore`
- `impl BorrowMut for TrustedWorktreesStore`
- `impl Into for TrustedWorktreesStore`
- `impl From for TrustedWorktreesStore`
- `impl TryInto for TrustedWorktreesStore`
- `impl TryFrom for TrustedWorktreesStore`
- `impl Any for TrustedWorktreesStore`
- `impl VZip for TrustedWorktreesStore`
- `impl CastableFrom for TrustedWorktreesStore`
- `impl CastableFrom for TrustedWorktreesStore`
- `impl Read for TrustedWorktreesStore`
- `impl IntoEither for TrustedWorktreesStore`
- `impl ErasedDestructor for TrustedWorktreesStore`
- `impl Same for TrustedWorktreesStore`
- `impl Pointable for TrustedWorktreesStore`
- `impl Instrument for TrustedWorktreesStore`
- `impl WithSubscriber for TrustedWorktreesStore`
- `impl FromAngle for TrustedWorktreesStore`
- `impl IntoAngle for TrustedWorktreesStore`
- `impl IntoCam16Unclamped for TrustedWorktreesStore`
- `impl Cam16IntoUnclamped for TrustedWorktreesStore`
- `impl ArraysFrom for TrustedWorktreesStore`
- `impl ArraysInto for TrustedWorktreesStore`
- `impl ComponentsFrom for TrustedWorktreesStore`
- `impl TryComponentsInto for TrustedWorktreesStore`
- `impl UintsFrom for TrustedWorktreesStore`
- `impl UintsInto for TrustedWorktreesStore`
- `impl AdaptIntoUnclamped for TrustedWorktreesStore`
- `impl AdaptInto for TrustedWorktreesStore`
- `impl IntoColor for TrustedWorktreesStore`
- `impl IntoColorUnclamped for TrustedWorktreesStore`
- `impl TryIntoColor for TrustedWorktreesStore`
- `impl FromStimulus for TrustedWorktreesStore`
- `impl IntoStimulus for TrustedWorktreesStore`
- `impl Debug for TrustedWorktreesStore`
- `impl EventEmitter for TrustedWorktreesStore`

## Auto Trait Implementations

`Freeze` `Send` `Sync` `Unpin` `UnsafeUnpin`

