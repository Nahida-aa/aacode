---
id: TelemetrySnapshot
title: TelemetrySnapshot
---

# Struct: TelemetrySnapshot

Defined in: [`packages/project/src/telemetry_snapshot.rs:12`](../../../../packages/project/src/telemetry_snapshot.rs#L12)

## Definition

```rust
pub struct TelemetrySnapshot {
    pub worktree_snapshots: Vec<TelemetryWorktreeSnapshot>,
}
```

## Implementations

### new()

```rust
pub fn new(project: &Entity<Project>, cx: &App) -> Task<TelemetrySnapshot>
```

Defined in: [`packages/project/src/telemetry_snapshot.rs:17`](../../../../packages/project/src/telemetry_snapshot.rs#L17)

#### Parameters

##### project

`&Entity<Project>`

##### cx

`&App`

#### Returns

`Task<TelemetrySnapshot>`

## Trait Implementations

- `impl Borrow for TelemetrySnapshot`
- `impl BorrowMut for TelemetrySnapshot`
- `impl CloneToUninit for TelemetrySnapshot`
- `impl Into for TelemetrySnapshot`
- `impl From for TelemetrySnapshot`
- `impl TryInto for TelemetrySnapshot`
- `impl TryFrom for TelemetrySnapshot`
- `impl Any for TelemetrySnapshot`
- `impl ToOwned for TelemetrySnapshot`
- `impl DeserializeOwned for TelemetrySnapshot`
- `impl Serialize for TelemetrySnapshot`
- `impl DynClone for TelemetrySnapshot`
- `impl VZip for TelemetrySnapshot`
- `impl CastableFrom for TelemetrySnapshot`
- `impl CastableFrom for TelemetrySnapshot`
- `impl Read for TelemetrySnapshot`
- `impl IntoEither for TelemetrySnapshot`
- `impl ErasedDestructor for TelemetrySnapshot`
- `impl Same for TelemetrySnapshot`
- `impl Pointable for TelemetrySnapshot`
- `impl Instrument for TelemetrySnapshot`
- `impl WithSubscriber for TelemetrySnapshot`
- `impl FromAngle for TelemetrySnapshot`
- `impl IntoAngle for TelemetrySnapshot`
- `impl IntoCam16Unclamped for TelemetrySnapshot`
- `impl Cam16IntoUnclamped for TelemetrySnapshot`
- `impl ArraysFrom for TelemetrySnapshot`
- `impl ArraysInto for TelemetrySnapshot`
- `impl ComponentsFrom for TelemetrySnapshot`
- `impl TryComponentsInto for TelemetrySnapshot`
- `impl UintsFrom for TelemetrySnapshot`
- `impl UintsInto for TelemetrySnapshot`
- `impl AdaptIntoUnclamped for TelemetrySnapshot`
- `impl AdaptInto for TelemetrySnapshot`
- `impl IntoColor for TelemetrySnapshot`
- `impl IntoColorUnclamped for TelemetrySnapshot`
- `impl TryIntoColor for TelemetrySnapshot`
- `impl FromStimulus for TelemetrySnapshot`
- `impl IntoStimulus for TelemetrySnapshot`
- `impl Debug for TelemetrySnapshot`
- `impl Clone for TelemetrySnapshot`
- `impl Serialize for TelemetrySnapshot`
- `impl Deserialize for TelemetrySnapshot`
- `impl StructuralPartialEq for TelemetrySnapshot`
- `impl PartialEq for TelemetrySnapshot`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

