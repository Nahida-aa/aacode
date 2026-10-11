---
id: Completion
title: Completion
---

# Struct: Completion

Defined in: [`packages/project/src/types/completion.rs:43`](../../../../packages/project/src/types/completion.rs#L43)

Similar to `CoreCompletion`, but with extra metadata attached.

## Definition

```rust
pub struct Completion {
    // The range of text that will be replaced by this completion.
    pub replace_range: Range<Anchor>,
    // The new text that will be inserted.
    pub new_text: String,
    // A label for this completion that is shown in the menu.
    pub label: CodeLabel,
    // The documentation for this completion.
    pub documentation: Option<CompletionDocumentation>,
    // Completion data source which it was constructed from.
    pub source: CompletionSource,
    // A path to an icon for this completion that is shown in the menu.
    pub icon_path: Option<SharedString>,
    // An optional color to tint this completion's icon with in the menu. When `None`, the menu's default muted color is used.
    pub icon_color: Option<Hsla>,
    // Text starting here and ending at the cursor will be used as the query for filtering this completion. If None, the start of the surrounding word is used.
    pub match_start: Option<Anchor>,
    // Key used for de-duplicating snippets. If None, always considered unique.
    pub snippet_deduplication_key: Option<(usize, usize)>,
    // Whether to adjust indentation (the default) or not.
    pub insert_text_mode: Option<InsertTextMode>,
    // An optional callback to invoke when this completion is confirmed. Returns whether new completions should be retriggered after the current one. If `true` is returned, the editor will show a new completion menu after this completion is confirmed. if no confirmation is provided or `false` is returned, the completion will be committed.
    pub confirm: Option<Arc<dyn Send + Sync + Fn(CompletionIntent, &Window, &App) -> bool>>,
    // An optional group for this completion. When the group changes between consecutive items, the completion menu inserts a divider. If the group also carries a label, a non-selectable header row is rendered below the divider.
    pub group: Option<CompletionGroup>,
}
```

## Fields

### replace_range

Defined in: [`packages/project/src/types/completion.rs:45`](../../../../packages/project/src/types/completion.rs#L45)

The range of text that will be replaced by this completion.


***

### new_text

Defined in: [`packages/project/src/types/completion.rs:47`](../../../../packages/project/src/types/completion.rs#L47)

The new text that will be inserted.


***

### label

Defined in: [`packages/project/src/types/completion.rs:49`](../../../../packages/project/src/types/completion.rs#L49)

A label for this completion that is shown in the menu.


***

### documentation

Defined in: [`packages/project/src/types/completion.rs:51`](../../../../packages/project/src/types/completion.rs#L51)

The documentation for this completion.


***

### source

Defined in: [`packages/project/src/types/completion.rs:53`](../../../../packages/project/src/types/completion.rs#L53)

Completion data source which it was constructed from.


***

### icon_path

Defined in: [`packages/project/src/types/completion.rs:55`](../../../../packages/project/src/types/completion.rs#L55)

A path to an icon for this completion that is shown in the menu.


***

### icon_color

Defined in: [`packages/project/src/types/completion.rs:58`](../../../../packages/project/src/types/completion.rs#L58)

An optional color to tint this completion's icon with in the menu.
When `None`, the menu's default muted color is used.


***

### match_start

Defined in: [`packages/project/src/types/completion.rs:62`](../../../../packages/project/src/types/completion.rs#L62)

Text starting here and ending at the cursor will be used as the query for filtering this completion.

If None, the start of the surrounding word is used.


***

### snippet_deduplication_key

Defined in: [`packages/project/src/types/completion.rs:64`](../../../../packages/project/src/types/completion.rs#L64)

Key used for de-duplicating snippets. If None, always considered unique.


***

### insert_text_mode

Defined in: [`packages/project/src/types/completion.rs:66`](../../../../packages/project/src/types/completion.rs#L66)

Whether to adjust indentation (the default) or not.


***

### confirm

Defined in: [`packages/project/src/types/completion.rs:71`](../../../../packages/project/src/types/completion.rs#L71)

An optional callback to invoke when this completion is confirmed.
Returns whether new completions should be retriggered after the current one.
If `true` is returned, the editor will show a new completion menu after this completion is confirmed.
if no confirmation is provided or `false` is returned, the completion will be committed.


***

### group

Defined in: [`packages/project/src/types/completion.rs:75`](../../../../packages/project/src/types/completion.rs#L75)

An optional group for this completion. When the group changes between consecutive
items, the completion menu inserts a divider. If the group also carries a label,
a non-selectable header row is rendered below the divider.

## Implementations

### kind()

```rust
pub fn kind(&self) -> Option<CompletionItemKind>
```

Defined in: [`packages/project/src/impls.rs:38`](../../../../packages/project/src/impls.rs#L38)

#### Returns

`Option<CompletionItemKind>`


***

### label()

```rust
pub fn label(&self) -> Option<String>
```

Defined in: [`packages/project/src/impls.rs:45`](../../../../packages/project/src/impls.rs#L45)

#### Returns

`Option<String>`


***

### filter_text()

```rust
pub fn filter_text(&self) -> &str
```

Defined in: [`packages/project/src/impls.rs:51`](../../../../packages/project/src/impls.rs#L51)

#### Returns

`&str`


***

### sort_key()

```rust
pub fn sort_key(&self) -> (usize, &str)
```

Defined in: [`packages/project/src/impls.rs:59`](../../../../packages/project/src/impls.rs#L59)

A key that can be used to sort completions when displaying
them to the user.

#### Returns

`(usize, &str)`


***

### is_snippet_kind()

```rust
pub fn is_snippet_kind(&self) -> bool
```

Defined in: [`packages/project/src/impls.rs:75`](../../../../packages/project/src/impls.rs#L75)

Whether this completion is a snippet.

#### Returns

`bool`


***

### is_snippet()

```rust
pub fn is_snippet(&self) -> bool
```

Defined in: [`packages/project/src/impls.rs:84`](../../../../packages/project/src/impls.rs#L84)

Whether this completion is a snippet or snippet-style LSP completion.

#### Returns

`bool`


***

### color()

```rust
pub fn color(&self) -> Option<Hsla>
```

Defined in: [`packages/project/src/impls.rs:96`](../../../../packages/project/src/impls.rs#L96)

Returns the corresponding color for this completion.

Will return `None` if this completion's kind is not `CompletionItemKind::COLOR`.

#### Returns

`Option<Hsla>`

## Trait Implementations

- `impl Borrow for Completion`
- `impl BorrowMut for Completion`
- `impl CloneToUninit for Completion`
- `impl Into for Completion`
- `impl From for Completion`
- `impl TryInto for Completion`
- `impl TryFrom for Completion`
- `impl Any for Completion`
- `impl ToOwned for Completion`
- `impl DynClone for Completion`
- `impl VZip for Completion`
- `impl CastableFrom for Completion`
- `impl CastableFrom for Completion`
- `impl Read for Completion`
- `impl IntoEither for Completion`
- `impl ErasedDestructor for Completion`
- `impl Same for Completion`
- `impl Pointable for Completion`
- `impl Instrument for Completion`
- `impl WithSubscriber for Completion`
- `impl FromAngle for Completion`
- `impl IntoAngle for Completion`
- `impl IntoCam16Unclamped for Completion`
- `impl Cam16IntoUnclamped for Completion`
- `impl ArraysFrom for Completion`
- `impl ArraysInto for Completion`
- `impl ComponentsFrom for Completion`
- `impl TryComponentsInto for Completion`
- `impl UintsFrom for Completion`
- `impl UintsInto for Completion`
- `impl AdaptIntoUnclamped for Completion`
- `impl AdaptInto for Completion`
- `impl IntoColor for Completion`
- `impl IntoColorUnclamped for Completion`
- `impl TryIntoColor for Completion`
- `impl FromStimulus for Completion`
- `impl IntoStimulus for Completion`
- `impl Clone for Completion`
- `impl Debug for Completion`

## Auto Trait Implementations

`Freeze` `Send` `Sync` `Unpin` `UnsafeUnpin`

