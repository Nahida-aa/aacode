---
id: GitState
title: GitState
---

# Struct: GitState

Defined in: [`packages/project/src/telemetry_snapshot.rs:40`](../../../../packages/project/src/telemetry_snapshot.rs#L40)

## Definition

```rust
pub struct GitState
{
    pub remote_url: Option<String>,
    pub head_sha: Option<String>,
    pub current_branch: Option<String>,
    pub diff: Option<String>,
}
```

## Trait Implementations

- `impl Borrow for GitState`
- `impl BorrowMut for GitState`
- `impl CloneToUninit for GitState`
- `impl Into for GitState`
- `impl From for GitState`
- `impl TryInto for GitState`
- `impl TryFrom for GitState`
- `impl Any for GitState`
- `impl ToOwned for GitState`
- `impl DeserializeOwned for GitState`
- `impl Serialize for GitState`
- `impl DynClone for GitState`
- `impl VZip for GitState`
- `impl CastableFrom for GitState`
- `impl CastableFrom for GitState`
- `impl Read for GitState`
- `impl IntoEither for GitState`
- `impl ErasedDestructor for GitState`
- `impl Same for GitState`
- `impl Pointable for GitState`
- `impl Instrument for GitState`
- `impl WithSubscriber for GitState`
- `impl FromAngle for GitState`
- `impl IntoAngle for GitState`
- `impl IntoCam16Unclamped for GitState`
- `impl Cam16IntoUnclamped for GitState`
- `impl ArraysFrom for GitState`
- `impl ArraysInto for GitState`
- `impl ComponentsFrom for GitState`
- `impl TryComponentsInto for GitState`
- `impl UintsFrom for GitState`
- `impl UintsInto for GitState`
- `impl AdaptIntoUnclamped for GitState`
- `impl AdaptInto for GitState`
- `impl IntoColor for GitState`
- `impl IntoColorUnclamped for GitState`
- `impl TryIntoColor for GitState`
- `impl FromStimulus for GitState`
- `impl IntoStimulus for GitState`
- `impl Debug for GitState`
- `impl Clone for GitState`
- `impl Serialize for GitState`
- `impl Deserialize for GitState`
- `impl StructuralPartialEq for GitState`
- `impl PartialEq for GitState`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

