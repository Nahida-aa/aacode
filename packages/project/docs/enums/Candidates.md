---
id: Candidates
title: Candidates
---

# Enum: Candidates

Defined in: [`packages/project/src/fuzzy.rs:16`](../../../../packages/project/src/fuzzy.rs#L16)

## Definition

```rust
pub enum Candidates {
    // Only consider directories.
    Directories,
    // Only consider files.
    Files,
    // Consider directories and files.
    Entries,
}
```

## Variants

### Directories

Defined in: [`packages/project/src/fuzzy.rs:18`](../../../../packages/project/src/fuzzy.rs#L18)

Only consider directories.


***

### Files

Defined in: [`packages/project/src/fuzzy.rs:20`](../../../../packages/project/src/fuzzy.rs#L20)

Only consider files.


***

### Entries

Defined in: [`packages/project/src/fuzzy.rs:22`](../../../../packages/project/src/fuzzy.rs#L22)

Consider directories and files.

## Trait Implementations

- `impl Borrow for Candidates`
- `impl BorrowMut for Candidates`
- `impl Into for Candidates`
- `impl From for Candidates`
- `impl TryInto for Candidates`
- `impl TryFrom for Candidates`
- `impl Any for Candidates`
- `impl VZip for Candidates`
- `impl CastableFrom for Candidates`
- `impl CastableFrom for Candidates`
- `impl Read for Candidates`
- `impl IntoEither for Candidates`
- `impl ErasedDestructor for Candidates`
- `impl Same for Candidates`
- `impl Pointable for Candidates`
- `impl Instrument for Candidates`
- `impl WithSubscriber for Candidates`
- `impl FromAngle for Candidates`
- `impl IntoAngle for Candidates`
- `impl IntoCam16Unclamped for Candidates`
- `impl Cam16IntoUnclamped for Candidates`
- `impl ArraysFrom for Candidates`
- `impl ArraysInto for Candidates`
- `impl ComponentsFrom for Candidates`
- `impl TryComponentsInto for Candidates`
- `impl UintsFrom for Candidates`
- `impl UintsInto for Candidates`
- `impl AdaptIntoUnclamped for Candidates`
- `impl AdaptInto for Candidates`
- `impl IntoColor for Candidates`
- `impl IntoColorUnclamped for Candidates`
- `impl TryIntoColor for Candidates`
- `impl FromStimulus for Candidates`
- `impl IntoStimulus for Candidates`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

