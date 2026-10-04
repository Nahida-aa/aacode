---
id: InvalidationStrategy
title: InvalidationStrategy
---

# Enum: InvalidationStrategy

Defined in: [`packages/project/src/lsp_store/inlay_hints.rs:53`](../../../../packages/project/src/lsp_store/inlay_hints.rs#L53)

A logic to apply when querying for new inlay hints and deciding what to do with the old entries in the cache in case of conflicts.

## Definition

```rust
pub enum InvalidationStrategy {
    // Language servers reset hints via <a href="https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#workspace_inlayHint_refresh">request</a>. Demands to re-query all inlay hints needed and invalidate all cached entries, but does not require instant update with invalidation. Despite nothing forbids language server from sending this request on every edit, it is expected to be sent only when certain internal server state update, invisible for the editor otherwise.
    RefreshRequested{ .. },
    // Multibuffer excerpt(s) and/or singleton buffer(s) were edited at least on one place. Neither editor nor LSP is able to tell which open file hints' are not affected, so all of them have to be invalidated, re-queried and do that fast enough to avoid being slow, but also debounce to avoid loading hints on every fast keystroke sequence.
    BufferEdited,
    // A new file got opened/new excerpt was added to a multibuffer/a [multi]buffer was scrolled to a new position. No invalidation should be done at all, all new hints are added to the cache. A special case is the editor toggles and settings change: in addition to LSP capabilities, Zed allows omitting certain hint kinds (defined by the corresponding LSP part: type/parameter/other) and toggling hints. This does not lead to cache invalidation, but would require cache usage for determining which hints are not displayed and issuing an update to inlays on the screen.
    None,
}
```

## Variants

### RefreshRequested

Defined in: [`packages/project/src/lsp_store/inlay_hints.rs:58`](../../../../packages/project/src/lsp_store/inlay_hints.rs#L58)

Language servers reset hints via <a href="https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#workspace_inlayHint_refresh">request</a>.
Demands to re-query all inlay hints needed and invalidate all cached entries, but does not require instant update with invalidation.

Despite nothing forbids language server from sending this request on every edit, it is expected to be sent only when certain internal server state update, invisible for the editor otherwise.


***

### BufferEdited

Defined in: [`packages/project/src/lsp_store/inlay_hints.rs:61`](../../../../packages/project/src/lsp_store/inlay_hints.rs#L61)

Multibuffer excerpt(s) and/or singleton buffer(s) were edited at least on one place.
Neither editor nor LSP is able to tell which open file hints' are not affected, so all of them have to be invalidated, re-queried and do that fast enough to avoid being slow, but also debounce to avoid loading hints on every fast keystroke sequence.


***

### None

Defined in: [`packages/project/src/lsp_store/inlay_hints.rs:68`](../../../../packages/project/src/lsp_store/inlay_hints.rs#L68)

A new file got opened/new excerpt was added to a multibuffer/a [multi]buffer was scrolled to a new position.
No invalidation should be done at all, all new hints are added to the cache.

A special case is the editor toggles and settings change:
in addition to LSP capabilities, Zed allows omitting certain hint kinds (defined by the corresponding LSP part: type/parameter/other) and toggling hints.
This does not lead to cache invalidation, but would require cache usage for determining which hints are not displayed and issuing an update to inlays on the screen.

## Implementations

### should_invalidate()

```rust
pub fn should_invalidate(&self) -> bool
```

Defined in: [`packages/project/src/lsp_store/inlay_hints.rs:72`](../../../../packages/project/src/lsp_store/inlay_hints.rs#L72)

#### Returns

`bool`

## Trait Implementations

- `impl Borrow for InvalidationStrategy`
- `impl BorrowMut for InvalidationStrategy`
- `impl CloneToUninit for InvalidationStrategy`
- `impl Into for InvalidationStrategy`
- `impl From for InvalidationStrategy`
- `impl TryInto for InvalidationStrategy`
- `impl TryFrom for InvalidationStrategy`
- `impl Any for InvalidationStrategy`
- `impl ToOwned for InvalidationStrategy`
- `impl DynClone for InvalidationStrategy`
- `impl VZip for InvalidationStrategy`
- `impl CastableFrom for InvalidationStrategy`
- `impl CastableFrom for InvalidationStrategy`
- `impl Read for InvalidationStrategy`
- `impl IntoEither for InvalidationStrategy`
- `impl ErasedDestructor for InvalidationStrategy`
- `impl Same for InvalidationStrategy`
- `impl Pointable for InvalidationStrategy`
- `impl Instrument for InvalidationStrategy`
- `impl WithSubscriber for InvalidationStrategy`
- `impl FromAngle for InvalidationStrategy`
- `impl IntoAngle for InvalidationStrategy`
- `impl IntoCam16Unclamped for InvalidationStrategy`
- `impl Cam16IntoUnclamped for InvalidationStrategy`
- `impl ArraysFrom for InvalidationStrategy`
- `impl ArraysInto for InvalidationStrategy`
- `impl ComponentsFrom for InvalidationStrategy`
- `impl TryComponentsInto for InvalidationStrategy`
- `impl UintsFrom for InvalidationStrategy`
- `impl UintsInto for InvalidationStrategy`
- `impl AdaptIntoUnclamped for InvalidationStrategy`
- `impl AdaptInto for InvalidationStrategy`
- `impl IntoColor for InvalidationStrategy`
- `impl IntoColorUnclamped for InvalidationStrategy`
- `impl TryIntoColor for InvalidationStrategy`
- `impl FromStimulus for InvalidationStrategy`
- `impl IntoStimulus for InvalidationStrategy`
- `impl ResetDiscriminant for InvalidationStrategy`
- `impl Debug for InvalidationStrategy`
- `impl Clone for InvalidationStrategy`
- `impl Copy for InvalidationStrategy`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

