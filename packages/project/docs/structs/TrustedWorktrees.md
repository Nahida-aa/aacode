---
id: TrustedWorktrees
title: TrustedWorktrees
---

# Struct: TrustedWorktrees

Defined in: [`packages/project/src/trusted_worktrees.rs:108`](../../../../packages/project/src/trusted_worktrees.rs#L108)

A collection of worktree trust metadata, can be accessed globally (if initialized) and subscribed to.

## Definition

```rust
pub struct TrustedWorktrees
```

## Implementations

### try_get_global()

```rust
pub fn try_get_global(cx: &App) -> Option<Entity<TrustedWorktreesStore>>
```

Defined in: [`packages/project/src/trusted_worktrees.rs:113`](../../../../packages/project/src/trusted_worktrees.rs#L113)

#### Parameters

##### cx

`&App`

#### Returns

`Option<Entity<TrustedWorktreesStore>>`


***

### has_restricted_worktrees()

```rust
pub fn has_restricted_worktrees(worktree_store: &Entity<WorktreeStore>, cx: &App) -> bool
```

Defined in: [`packages/project/src/trusted_worktrees.rs:118`](../../../../packages/project/src/trusted_worktrees.rs#L118)

Whether the given project store has any restricted worktrees.

#### Parameters

##### worktree_store

`&Entity<WorktreeStore>`

##### cx

`&App`

#### Returns

`bool`

## Trait Implementations

- `impl Borrow for TrustedWorktrees`
- `impl BorrowMut for TrustedWorktrees`
- `impl Into for TrustedWorktrees`
- `impl From for TrustedWorktrees`
- `impl TryInto for TrustedWorktrees`
- `impl TryFrom for TrustedWorktrees`
- `impl Any for TrustedWorktrees`
- `impl ReadGlobal for TrustedWorktrees`
- `impl UpdateGlobal for TrustedWorktrees`
- `impl VZip for TrustedWorktrees`
- `impl CastableFrom for TrustedWorktrees`
- `impl CastableFrom for TrustedWorktrees`
- `impl Read for TrustedWorktrees`
- `impl IntoEither for TrustedWorktrees`
- `impl ErasedDestructor for TrustedWorktrees`
- `impl Same for TrustedWorktrees`
- `impl Pointable for TrustedWorktrees`
- `impl Instrument for TrustedWorktrees`
- `impl WithSubscriber for TrustedWorktrees`
- `impl FromAngle for TrustedWorktrees`
- `impl IntoAngle for TrustedWorktrees`
- `impl IntoCam16Unclamped for TrustedWorktrees`
- `impl Cam16IntoUnclamped for TrustedWorktrees`
- `impl ArraysFrom for TrustedWorktrees`
- `impl ArraysInto for TrustedWorktrees`
- `impl ComponentsFrom for TrustedWorktrees`
- `impl TryComponentsInto for TrustedWorktrees`
- `impl UintsFrom for TrustedWorktrees`
- `impl UintsInto for TrustedWorktrees`
- `impl AdaptIntoUnclamped for TrustedWorktrees`
- `impl AdaptInto for TrustedWorktrees`
- `impl IntoColor for TrustedWorktrees`
- `impl IntoColorUnclamped for TrustedWorktrees`
- `impl TryIntoColor for TrustedWorktrees`
- `impl FromStimulus for TrustedWorktrees`
- `impl IntoStimulus for TrustedWorktrees`
- `impl Global for TrustedWorktrees`

## Auto Trait Implementations

`Freeze` `Send` `Sync` `Unpin` `UnsafeUnpin`

