---
id: ConflictSet
title: ConflictSet
---

# Struct: ConflictSet

Defined in: [`packages/project/src/git_store/conflict_set.rs:5`](../../../../packages/project/src/git_store/conflict_set.rs#L5)

## Definition

```rust
pub struct ConflictSet
{
    pub has_conflict: bool,
    pub snapshot: ConflictSetSnapshot,
}
```

## Implementations

### new()

```rust
pub fn new(buffer_id: BufferId, has_conflict: bool, _: &Context<'_, Self>) -> Self
```

Defined in: [`packages/project/src/git_store/conflict_set.rs:144`](../../../../packages/project/src/git_store/conflict_set.rs#L144)

#### Parameters

##### buffer_id

`BufferId`

##### has_conflict

`bool`

##### _

`&Context<'_, Self>`

#### Returns

`Self`


***

### set_has_conflict()

```rust
pub fn set_has_conflict(&self, has_conflict: bool, cx: &Context<'_, Self>) -> bool
```

Defined in: [`packages/project/src/git_store/conflict_set.rs:154`](../../../../packages/project/src/git_store/conflict_set.rs#L154)

#### Parameters

##### has_conflict

`bool`

##### cx

`&Context<'_, Self>`

#### Returns

`bool`


***

### snapshot()

```rust
pub fn snapshot(&self) -> ConflictSetSnapshot
```

Defined in: [`packages/project/src/git_store/conflict_set.rs:171`](../../../../packages/project/src/git_store/conflict_set.rs#L171)

#### Returns

[`ConflictSetSnapshot`](ConflictSetSnapshot.md)


***

### set_snapshot()

```rust
pub fn set_snapshot(&self, snapshot: ConflictSetSnapshot, update: ConflictSetUpdate, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/git_store/conflict_set.rs:175`](../../../../packages/project/src/git_store/conflict_set.rs#L175)

#### Parameters

##### snapshot

[`ConflictSetSnapshot`](ConflictSetSnapshot.md)

##### update

[`ConflictSetUpdate`](ConflictSetUpdate.md)

##### cx

`&Context<'_, Self>`


***

### parse()

```rust
pub fn parse(buffer: &BufferSnapshot) -> ConflictSetSnapshot
```

Defined in: [`packages/project/src/git_store/conflict_set.rs:185`](../../../../packages/project/src/git_store/conflict_set.rs#L185)

#### Parameters

##### buffer

`&BufferSnapshot`

#### Returns

[`ConflictSetSnapshot`](ConflictSetSnapshot.md)

## Trait Implementations

- `impl Borrow for ConflictSet`
- `impl BorrowMut for ConflictSet`
- `impl Into for ConflictSet`
- `impl From for ConflictSet`
- `impl TryInto for ConflictSet`
- `impl TryFrom for ConflictSet`
- `impl Any for ConflictSet`
- `impl VZip for ConflictSet`
- `impl CastableFrom for ConflictSet`
- `impl CastableFrom for ConflictSet`
- `impl Read for ConflictSet`
- `impl IntoEither for ConflictSet`
- `impl ErasedDestructor for ConflictSet`
- `impl Same for ConflictSet`
- `impl Pointable for ConflictSet`
- `impl Instrument for ConflictSet`
- `impl WithSubscriber for ConflictSet`
- `impl FromAngle for ConflictSet`
- `impl IntoAngle for ConflictSet`
- `impl IntoCam16Unclamped for ConflictSet`
- `impl Cam16IntoUnclamped for ConflictSet`
- `impl ArraysFrom for ConflictSet`
- `impl ArraysInto for ConflictSet`
- `impl ComponentsFrom for ConflictSet`
- `impl TryComponentsInto for ConflictSet`
- `impl UintsFrom for ConflictSet`
- `impl UintsInto for ConflictSet`
- `impl AdaptIntoUnclamped for ConflictSet`
- `impl AdaptInto for ConflictSet`
- `impl IntoColor for ConflictSet`
- `impl IntoColorUnclamped for ConflictSet`
- `impl TryIntoColor for ConflictSet`
- `impl FromStimulus for ConflictSet`
- `impl IntoStimulus for ConflictSet`
- `impl EventEmitter for ConflictSet`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

