---
id: CommitFile
title: CommitFile
---

# Struct: CommitFile

Defined in: [`packages/project/src/git_store/mod.rs:228`](../../../../packages/project/src/git_store/mod.rs#L228)

## Definition

```rust
pub struct CommitFile
{
    pub path: RepoPath,
    pub old_text: Option<String>,
    pub new_text: Option<String>,
    pub is_binary: bool,
}
```

## Implementations

### status()

```rust
pub fn status(&self) -> CommitFileStatus
```

Defined in: [`packages/project/src/git_store/mod.rs:236`](../../../../packages/project/src/git_store/mod.rs#L236)

#### Returns

`CommitFileStatus`

## Trait Implementations

- `impl Borrow for CommitFile`
- `impl BorrowMut for CommitFile`
- `impl Into for CommitFile`
- `impl From for CommitFile`
- `impl TryInto for CommitFile`
- `impl TryFrom for CommitFile`
- `impl Any for CommitFile`
- `impl VZip for CommitFile`
- `impl CastableFrom for CommitFile`
- `impl CastableFrom for CommitFile`
- `impl Read for CommitFile`
- `impl IntoEither for CommitFile`
- `impl ErasedDestructor for CommitFile`
- `impl Same for CommitFile`
- `impl Pointable for CommitFile`
- `impl Instrument for CommitFile`
- `impl WithSubscriber for CommitFile`
- `impl FromAngle for CommitFile`
- `impl IntoAngle for CommitFile`
- `impl IntoCam16Unclamped for CommitFile`
- `impl Cam16IntoUnclamped for CommitFile`
- `impl ArraysFrom for CommitFile`
- `impl ArraysInto for CommitFile`
- `impl ComponentsFrom for CommitFile`
- `impl TryComponentsInto for CommitFile`
- `impl UintsFrom for CommitFile`
- `impl UintsInto for CommitFile`
- `impl AdaptIntoUnclamped for CommitFile`
- `impl AdaptInto for CommitFile`
- `impl IntoColor for CommitFile`
- `impl IntoColorUnclamped for CommitFile`
- `impl TryIntoColor for CommitFile`
- `impl FromStimulus for CommitFile`
- `impl IntoStimulus for CommitFile`
- `impl Debug for CommitFile`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

