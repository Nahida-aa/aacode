---
id: CompletionsQuery
title: CompletionsQuery
---

# Struct: CompletionsQuery

Defined in: [`packages/project/src/debugger/session.rs:769`](../../../../packages/project/src/debugger/session.rs#L769)

## Definition

```rust
pub struct CompletionsQuery
{
    pub query: String,
    pub column: u64,
    pub line: Option<u64>,
    pub frame_id: Option<u64>,
}
```

## Implementations

### new()

```rust
pub fn new(buffer: &Buffer, cursor_position: Anchor, frame_id: Option<u64>) -> Self
```

Defined in: [`packages/project/src/debugger/session.rs:777`](../../../../packages/project/src/debugger/session.rs#L777)

#### Parameters

##### buffer

`&Buffer`

##### cursor_position

`Anchor`

##### frame_id

`Option<u64>`

#### Returns

`Self`

## Trait Implementations

- `impl Borrow for CompletionsQuery`
- `impl BorrowMut for CompletionsQuery`
- `impl CloneToUninit for CompletionsQuery`
- `impl Into for CompletionsQuery`
- `impl From for CompletionsQuery`
- `impl TryInto for CompletionsQuery`
- `impl TryFrom for CompletionsQuery`
- `impl Any for CompletionsQuery`
- `impl ToOwned for CompletionsQuery`
- `impl Equivalent for CompletionsQuery`
- `impl DynClone for CompletionsQuery`
- `impl VZip for CompletionsQuery`
- `impl CastableFrom for CompletionsQuery`
- `impl CastableFrom for CompletionsQuery`
- `impl Read for CompletionsQuery`
- `impl IntoEither for CompletionsQuery`
- `impl ErasedDestructor for CompletionsQuery`
- `impl Same for CompletionsQuery`
- `impl Pointable for CompletionsQuery`
- `impl Instrument for CompletionsQuery`
- `impl WithSubscriber for CompletionsQuery`
- `impl FromAngle for CompletionsQuery`
- `impl IntoAngle for CompletionsQuery`
- `impl IntoCam16Unclamped for CompletionsQuery`
- `impl Cam16IntoUnclamped for CompletionsQuery`
- `impl ArraysFrom for CompletionsQuery`
- `impl ArraysInto for CompletionsQuery`
- `impl ComponentsFrom for CompletionsQuery`
- `impl TryComponentsInto for CompletionsQuery`
- `impl UintsFrom for CompletionsQuery`
- `impl UintsInto for CompletionsQuery`
- `impl AdaptIntoUnclamped for CompletionsQuery`
- `impl AdaptInto for CompletionsQuery`
- `impl IntoColor for CompletionsQuery`
- `impl IntoColorUnclamped for CompletionsQuery`
- `impl TryIntoColor for CompletionsQuery`
- `impl FromStimulus for CompletionsQuery`
- `impl IntoStimulus for CompletionsQuery`
- `impl Equivalent for CompletionsQuery`
- `impl LocalDapCommand for CompletionsQuery`
- `impl DapCommand for CompletionsQuery`
- `impl Debug for CompletionsQuery`
- `impl Clone for CompletionsQuery`
- `impl Hash for CompletionsQuery`
- `impl StructuralPartialEq for CompletionsQuery`
- `impl PartialEq for CompletionsQuery`
- `impl Eq for CompletionsQuery`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

