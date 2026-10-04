---
id: track_worktree_trust
title: track_worktree_trust
---

# Function: track_worktree_trust()

```rust
pub fn track_worktree_trust(worktree_store: Entity<WorktreeStore>, remote_host: Option<RemoteHostLocation>, downstream_client: Option<(AnyProtoClient, ProjectId)>, upstream_client: Option<(AnyProtoClient, ProjectId)>, cx: &App)
```

Defined in: [`packages/project/src/trusted_worktrees.rs:66`](../../../../packages/project/src/trusted_worktrees.rs#L66)

An initialization call to set up trust global for a particular project (remote or local).

## Parameters

### worktree_store

`Entity<WorktreeStore>`

### remote_host

`Option<RemoteHostLocation>`

### downstream_client

`Option<(AnyProtoClient, ProjectId)>`

### upstream_client

`Option<(AnyProtoClient, ProjectId)>`

### cx

`&App`

