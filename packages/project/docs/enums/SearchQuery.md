---
id: SearchQuery
title: SearchQuery
---

# Enum: SearchQuery

Defined in: [`packages/project/src/search/mod.rs:76`](../../../../packages/project/src/search/mod.rs#L76)

## Definition

```rust
pub enum SearchQuery
{
    Text{ .. },
    Regex{ .. },
}
```

## Implementations

### text()

```rust
pub fn text<impl ToString: ToString>(query: impl ?, whole_word: bool, case_sensitive: bool, include_ignored: bool, files_to_include: PathMatcher, files_to_exclude: PathMatcher, match_full_paths: bool, buffers: Option<Vec<Entity<Buffer>>>) -> Result<Self>
```

Defined in: [`packages/project/src/search/mod.rs:108`](../../../../packages/project/src/search/mod.rs#L108)

Create a text query

If `match_full_paths` is true, include/exclude patterns will always be matched against fully qualified project paths beginning with a project root.
If `match_full_paths` is false, patterns will be matched against worktree-relative paths.

#### Parameters

##### query

`impl ?`

##### whole_word

`bool`

##### case_sensitive

`bool`

##### include_ignored

`bool`

##### files_to_include

`PathMatcher`

##### files_to_exclude

`PathMatcher`

##### match_full_paths

`bool`

##### buffers

`Option<Vec<Entity<Buffer>>>`

#### Returns

`Result<Self>`


***

### regex()

```rust
pub fn regex<impl ToString: ToString>(query: impl ?, whole_word: bool, case_sensitive: bool, include_ignored: bool, one_match_per_line: bool, files_to_include: PathMatcher, files_to_exclude: PathMatcher, match_full_paths: bool, buffers: Option<Vec<Entity<Buffer>>>) -> Result<Self>
```

Defined in: [`packages/project/src/search/mod.rs:160`](../../../../packages/project/src/search/mod.rs#L160)

Create a regex query

If `match_full_paths` is true, include/exclude patterns will be matched against fully qualified project paths
beginning with a project root name. If false, they will be matched against project-relative paths (which don't start
with their respective project root).

#### Parameters

##### query

`impl ?`

##### whole_word

`bool`

##### case_sensitive

`bool`

##### include_ignored

`bool`

##### one_match_per_line

`bool`

##### files_to_include

`PathMatcher`

##### files_to_exclude

`PathMatcher`

##### match_full_paths

`bool`

##### buffers

`Option<Vec<Entity<Buffer>>>`

#### Returns

`Result<Self>`


***

### escaped_regex()

```rust
pub fn escaped_regex<impl ToString: ToString>(query: impl ?, whole_word: bool, case_sensitive: bool, include_ignored: bool, files_to_include: PathMatcher, files_to_exclude: PathMatcher, match_full_paths: bool, buffers: Option<Vec<Entity<Buffer>>>) -> Result<Self>
```

Defined in: [`packages/project/src/search/mod.rs:195`](../../../../packages/project/src/search/mod.rs#L195)

Create a regex query from a literal string, escaping any regex
metacharacters so that the resulting query matches the literal text.

Unlike `regex`, the query stored on the resulting `SearchQuery` is the
original unescaped text, so `as_str` returns what the user typed.

#### Parameters

##### query

`impl ?`

##### whole_word

`bool`

##### case_sensitive

`bool`

##### include_ignored

`bool`

##### files_to_include

`PathMatcher`

##### files_to_exclude

`PathMatcher`

##### match_full_paths

`bool`

##### buffers

`Option<Vec<Entity<Buffer>>>`

#### Returns

`Result<Self>`


***

### from_proto()

```rust
pub fn from_proto(message: SearchQuery, path_style: PathStyle) -> Result<Self>
```

Defined in: [`packages/project/src/search/mod.rs:313`](../../../../packages/project/src/search/mod.rs#L313)

#### Parameters

##### message

`SearchQuery`

##### path_style

`PathStyle`

#### Returns

`Result<Self>`


***

### with_replacement()

```rust
pub fn with_replacement(self, new_replacement: String) -> Self
```

Defined in: [`packages/project/src/search/mod.rs:364`](../../../../packages/project/src/search/mod.rs#L364)

#### Parameters

##### new_replacement

`String`

#### Returns

`Self`


***

### to_proto()

```rust
pub fn to_proto(&self) -> SearchQuery
```

Defined in: [`packages/project/src/search/mod.rs:380`](../../../../packages/project/src/search/mod.rs#L380)

#### Returns

`SearchQuery`


***

### detect()

```rust
pub async fn detect(&self, reader: BufReader<Box<dyn Read + Send + Sync>>) -> Result<Option<MatchPositionHint>>
```

Defined in: [`packages/project/src/search/mod.rs:398`](../../../../packages/project/src/search/mod.rs#L398)

#### Parameters

##### reader

`BufReader<Box<dyn Read + Send + Sync>>`

#### Returns

`Result<Option<MatchPositionHint>>`


***

### replacement()

```rust
pub fn replacement(&self) -> Option<&str>
```

Defined in: [`packages/project/src/search/mod.rs:453`](../../../../packages/project/src/search/mod.rs#L453)

Returns the replacement text for this `SearchQuery`.

#### Returns

`Option<&str>`


***

### replacement_for()

```rust
pub fn replacement_for<'a>(&self, line: &'a str, hit: Range<usize>) -> Option<Cow<'a, str>>
```

Defined in: [`packages/project/src/search/mod.rs:461`](../../../../packages/project/src/search/mod.rs#L461)

Expands `hit` against its line so lookaround assertions retain context.

#### Parameters

##### line

`&'a str`

##### hit

`Range<usize>`

#### Returns

`Option<Cow<'a, str>>`


***

### search()

```rust
pub async fn search(&self, buffer: &BufferSnapshot, subrange: Option<Range<usize>>) -> Vec<Range<usize>>
```

Defined in: [`packages/project/src/search/mod.rs:509`](../../../../packages/project/src/search/mod.rs#L509)

#### Parameters

##### buffer

`&BufferSnapshot`

##### subrange

`Option<Range<usize>>`

#### Returns

`Vec<Range<usize>>`


***

### is_empty()

```rust
pub fn is_empty(&self) -> bool
```

Defined in: [`packages/project/src/search/mod.rs:594`](../../../../packages/project/src/search/mod.rs#L594)

#### Returns

`bool`


***

### as_str()

```rust
pub fn as_str(&self) -> &str
```

Defined in: [`packages/project/src/search/mod.rs:598`](../../../../packages/project/src/search/mod.rs#L598)

#### Returns

`&str`


***

### whole_word()

```rust
pub fn whole_word(&self) -> bool
```

Defined in: [`packages/project/src/search/mod.rs:602`](../../../../packages/project/src/search/mod.rs#L602)

#### Returns

`bool`


***

### case_sensitive()

```rust
pub fn case_sensitive(&self) -> bool
```

Defined in: [`packages/project/src/search/mod.rs:609`](../../../../packages/project/src/search/mod.rs#L609)

#### Returns

`bool`


***

### include_ignored()

```rust
pub fn include_ignored(&self) -> bool
```

Defined in: [`packages/project/src/search/mod.rs:616`](../../../../packages/project/src/search/mod.rs#L616)

#### Returns

`bool`


***

### is_regex()

```rust
pub fn is_regex(&self) -> bool
```

Defined in: [`packages/project/src/search/mod.rs:627`](../../../../packages/project/src/search/mod.rs#L627)

#### Returns

`bool`


***

### replacement_requires_context()

```rust
pub fn replacement_requires_context(&self) -> bool
```

Defined in: [`packages/project/src/search/mod.rs:631`](../../../../packages/project/src/search/mod.rs#L631)

#### Returns

`bool`


***

### files_to_include()

```rust
pub fn files_to_include(&self) -> &PathMatcher
```

Defined in: [`packages/project/src/search/mod.rs:635`](../../../../packages/project/src/search/mod.rs#L635)

#### Returns

`&PathMatcher`


***

### files_to_exclude()

```rust
pub fn files_to_exclude(&self) -> &PathMatcher
```

Defined in: [`packages/project/src/search/mod.rs:639`](../../../../packages/project/src/search/mod.rs#L639)

#### Returns

`&PathMatcher`


***

### buffers()

```rust
pub fn buffers(&self) -> Option<&Vec<Entity<Buffer>>>
```

Defined in: [`packages/project/src/search/mod.rs:643`](../../../../packages/project/src/search/mod.rs#L643)

#### Returns

`Option<&Vec<Entity<Buffer>>>`


***

### is_opened_only()

```rust
pub fn is_opened_only(&self) -> bool
```

Defined in: [`packages/project/src/search/mod.rs:647`](../../../../packages/project/src/search/mod.rs#L647)

#### Returns

`bool`


***

### filters_path()

```rust
pub fn filters_path(&self) -> bool
```

Defined in: [`packages/project/src/search/mod.rs:651`](../../../../packages/project/src/search/mod.rs#L651)

#### Returns

`bool`


***

### match_full_paths()

```rust
pub fn match_full_paths(&self) -> bool
```

Defined in: [`packages/project/src/search/mod.rs:656`](../../../../packages/project/src/search/mod.rs#L656)

#### Returns

`bool`


***

### match_path()

```rust
pub fn match_path(&self, file_path: &RelPath) -> bool
```

Defined in: [`packages/project/src/search/mod.rs:662`](../../../../packages/project/src/search/mod.rs#L662)

Check match full paths to determine whether you're required to pass a fully qualified
project path (starts with a project root).

#### Parameters

##### file_path

`&RelPath`

#### Returns

`bool`


***

### as_inner()

```rust
pub fn as_inner(&self) -> &SearchInputs
```

Defined in: [`packages/project/src/search/mod.rs:676`](../../../../packages/project/src/search/mod.rs#L676)

#### Returns

`&SearchInputs`


***

### search_str()

```rust
pub fn search_str(&self, text: &str) -> Vec<Range<usize>>
```

Defined in: [`packages/project/src/search/mod.rs:682`](../../../../packages/project/src/search/mod.rs#L682)

#### Parameters

##### text

`&str`

#### Returns

`Vec<Range<usize>>`

## Trait Implementations

- `impl Borrow for SearchQuery`
- `impl BorrowMut for SearchQuery`
- `impl CloneToUninit for SearchQuery`
- `impl Into for SearchQuery`
- `impl From for SearchQuery`
- `impl TryInto for SearchQuery`
- `impl TryFrom for SearchQuery`
- `impl Any for SearchQuery`
- `impl ToOwned for SearchQuery`
- `impl DynClone for SearchQuery`
- `impl VZip for SearchQuery`
- `impl CastableFrom for SearchQuery`
- `impl CastableFrom for SearchQuery`
- `impl Read for SearchQuery`
- `impl IntoEither for SearchQuery`
- `impl ErasedDestructor for SearchQuery`
- `impl Same for SearchQuery`
- `impl Pointable for SearchQuery`
- `impl Instrument for SearchQuery`
- `impl WithSubscriber for SearchQuery`
- `impl FromAngle for SearchQuery`
- `impl IntoAngle for SearchQuery`
- `impl IntoCam16Unclamped for SearchQuery`
- `impl Cam16IntoUnclamped for SearchQuery`
- `impl ArraysFrom for SearchQuery`
- `impl ArraysInto for SearchQuery`
- `impl ComponentsFrom for SearchQuery`
- `impl TryComponentsInto for SearchQuery`
- `impl UintsFrom for SearchQuery`
- `impl UintsInto for SearchQuery`
- `impl AdaptIntoUnclamped for SearchQuery`
- `impl AdaptInto for SearchQuery`
- `impl IntoColor for SearchQuery`
- `impl IntoColorUnclamped for SearchQuery`
- `impl TryIntoColor for SearchQuery`
- `impl FromStimulus for SearchQuery`
- `impl IntoStimulus for SearchQuery`
- `impl Clone for SearchQuery`
- `impl Debug for SearchQuery`

## Auto Trait Implementations

`Freeze` `Send` `Sync` `Unpin` `UnsafeUnpin`

