---
id: LspStore
title: LspStore
---

# Struct: LspStore

Defined in: [`packages/project/src/lsp_store/mod.rs:4540`](../../../../packages/project/src/lsp_store/mod.rs#L4540)

## Definition

```rust
pub struct LspStore
{
    pub languages: Arc<LanguageRegistry>,
    pub language_server_statuses: BTreeMap<LanguageServerId, LanguageServerStatus>,
    pub lsp_server_capabilities: HashMap<LanguageServerId, ServerCapabilities>,
}
```

_（存在非公开字段）_

## Implementations

### code_lens_actions()

```rust
pub fn code_lens_actions(&self, buffer: &Entity<Buffer>, cx: &Context<'_, Self>) -> Task<Result<Option<CodeLensActions>>>
```

Defined in: [`packages/project/src/lsp_store/code_lens.rs:120`](../../../../packages/project/src/lsp_store/code_lens.rs#L120)

Fetches all code lenses for the buffer, each tagged with the
[`CodeLensActionId`](CodeLensActionId.md) that callers must pass back to
[`Self::resolve_code_lens`](LspStore.md). Resolution is the caller's job.

#### Parameters

##### buffer

`&Entity<Buffer>`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<Option<CodeLensActions>>>`


***

### resolve_code_lens()

```rust
pub fn resolve_code_lens(&self, buffer: &Entity<Buffer>, server_id: LanguageServerId, lens_id: CodeLensActionId, cx: &Context<'_, Self>) -> CodeLensResolveTask
```

Defined in: [`packages/project/src/lsp_store/code_lens.rs:363`](../../../../packages/project/src/lsp_store/code_lens.rs#L363)

Resolves a single code lens via `codeLens/resolve`, identified by
the [`CodeLensActionId`](CodeLensActionId.md) returned from [`Self::code_lens_actions`](LspStore.md).
The returned task is shared and cached on `CodeLensData::resolving`
keyed by `(server, lens_id)`, so concurrent callers awaiting the
same lens only drive a single LSP request.

`None` is yielded when the lens cannot be resolved (id no longer
cached, server gone, no `resolveProvider`, request failure, etc.).
On success, the cached entry is updated in place before the
`(id, resolved_action)` pair is returned.

All visibility / batching policy lives in the caller. Remote (proto)
resolves are forwarded to the host via [`Self::resolve_code_action`](LspStore.md).

#### Parameters

##### buffer

`&Entity<Buffer>`

##### server_id

`LanguageServerId`

##### lens_id

[`CodeLensActionId`](CodeLensActionId.md)

##### cx

`&Context<'_, Self>`

#### Returns

[`CodeLensResolveTask`](../type-aliases/CodeLensResolveTask.md)


***

### document_colors()

```rust
pub fn document_colors(&self, buffer: Entity<Buffer>, cx: &Context<'_, Self>) -> Option<Shared<Task<Result<DocumentColors, Arc<Error>>>>>
```

Defined in: [`packages/project/src/lsp_store/document_colors.rs:103`](../../../../packages/project/src/lsp_store/document_colors.rs#L103)

#### Parameters

##### buffer

`Entity<Buffer>`

##### cx

`&Context<'_, Self>`

#### Returns

`Option<Shared<Task<Result<DocumentColors, Arc<Error>>>>>`


***

### resolve_color_presentation()

```rust
pub fn resolve_color_presentation(&self, color: DocumentColor, buffer: Entity<Buffer>, server_id: LanguageServerId, cx: &Context<'_, Self>) -> Task<Result<DocumentColor>>
```

Defined in: [`packages/project/src/lsp_store/document_colors.rs:239`](../../../../packages/project/src/lsp_store/document_colors.rs#L239)

#### Parameters

##### color

`DocumentColor`

##### buffer

`Entity<Buffer>`

##### server_id

`LanguageServerId`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<DocumentColor>>`


***

### fetch_document_links()

```rust
pub fn fetch_document_links(&self, buffer: &Entity<Buffer>, cx: &Context<'_, Self>) -> Task<Option<BufferDocumentLinks>>
```

Defined in: [`packages/project/src/lsp_store/document_links.rs:125`](../../../../packages/project/src/lsp_store/document_links.rs#L125)

`Some(..)` means the underlying state was actually refreshed; `None`
means the fetch was skipped or failed, and the caller should keep its
previous data.

#### Parameters

##### buffer

`&Entity<Buffer>`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Option<BufferDocumentLinks>>`


***

### resolved_document_link()

```rust
pub fn resolved_document_link(&self, buffer: &Entity<Buffer>, server_id: LanguageServerId, link_id: DocumentLinkId, cx: &Context<'_, Self>) -> Option<ResolvedDocumentLink>
```

Defined in: [`packages/project/src/lsp_store/document_links.rs:363`](../../../../packages/project/src/lsp_store/document_links.rs#L363)

Returns the resolved state for a cached document link, deduplicating
in-flight `documentLink/resolve` requests across editors via a `Shared`
task stored on `DocumentLinksData`.

`link_id` is the [`DocumentLinkId`](DocumentLinkId.md) stamped on the cached link by
[`Self::fetch_document_links`](LspStore.md); sibling links sharing the same buffer
range are disambiguated by it. `None` is returned when the cache no
longer holds a matching link (likely a version bump in between).

#### Parameters

##### buffer

`&Entity<Buffer>`

##### server_id

`LanguageServerId`

##### link_id

[`DocumentLinkId`](DocumentLinkId.md)

##### cx

`&Context<'_, Self>`

#### Returns

`Option<ResolvedDocumentLink>`


***

### fetch_document_symbols()

```rust
pub fn fetch_document_symbols(&self, buffer: &Entity<Buffer>, cx: &Context<'_, Self>) -> Task<Vec<OutlineItem<Anchor>>>
```

Defined in: [`packages/project/src/lsp_store/document_symbols.rs:102`](../../../../packages/project/src/lsp_store/document_symbols.rs#L102)

Returns a task that resolves to the document symbol outline items for
the given buffer.

Caches results per buffer version so repeated calls for the same version
return immediately. Deduplicates concurrent in-flight requests.

The returned items contain text and ranges but no syntax highlights.
Callers (e.g. the editor) are responsible for applying highlights
via the buffer's tree-sitter data and the active theme.

#### Parameters

##### buffer

`&Entity<Buffer>`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Vec<OutlineItem<Anchor>>>`


***

### fetch_folding_ranges()

```rust
pub fn fetch_folding_ranges(&self, buffer: &Entity<Buffer>, cx: &Context<'_, Self>) -> Task<Vec<LspFoldingRange>>
```

Defined in: [`packages/project/src/lsp_store/folding_ranges.rs:102`](../../../../packages/project/src/lsp_store/folding_ranges.rs#L102)

Returns a task that resolves to the folding ranges for the given buffer.

Caches results per buffer version so repeated calls for the same version
return immediately. Deduplicates concurrent in-flight requests.

#### Parameters

##### buffer

`&Entity<Buffer>`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Vec<LspFoldingRange>>`


***

### semantic_tokens()

```rust
pub fn semantic_tokens(&self, buffer: Entity<Buffer>, cx: &Context<'_, Self>) -> Shared<Task<Result<BufferSemanticTokens, Arc<Error>>>>
```

Defined in: [`packages/project/src/lsp_store/semantic_tokens.rs:90`](../../../../packages/project/src/lsp_store/semantic_tokens.rs#L90)

#### Parameters

##### buffer

`Entity<Buffer>`

##### cx

`&Context<'_, Self>`

#### Returns

`Shared<Task<Result<BufferSemanticTokens, Arc<Error>>>>`


***

### get_or_create_token_stylizer()

```rust
pub fn get_or_create_token_stylizer(&self, server_id: LanguageServerId, language: Option<&LanguageName>, cx: &App) -> Option<&SemanticTokenStylizer>
```

Defined in: [`packages/project/src/lsp_store/semantic_tokens.rs:509`](../../../../packages/project/src/lsp_store/semantic_tokens.rs#L509)

#### Parameters

##### server_id

`LanguageServerId`

##### language

`Option<&LanguageName>`

##### cx

`&App`

#### Returns

`Option<&SemanticTokenStylizer>`


***

### init()

```rust
pub fn init(client: &AnyProtoClient)
```

Defined in: [`packages/project/src/lsp_store/mod.rs:4770`](../../../../packages/project/src/lsp_store/mod.rs#L4770)

#### Parameters

##### client

`&AnyProtoClient`


***

### as_remote()

```rust
pub fn as_remote(&self) -> Option<&RemoteLspStore>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:4836`](../../../../packages/project/src/lsp_store/mod.rs#L4836)

#### Returns

`Option<&RemoteLspStore>`


***

### as_local()

```rust
pub fn as_local(&self) -> Option<&LocalLspStore>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:4843`](../../../../packages/project/src/lsp_store/mod.rs#L4843)

#### Returns

`Option<&LocalLspStore>`


***

### as_local_mut()

```rust
pub fn as_local_mut(&self) -> Option<&LocalLspStore>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:4850`](../../../../packages/project/src/lsp_store/mod.rs#L4850)

#### Returns

`Option<&LocalLspStore>`


***

### upstream_client()

```rust
pub fn upstream_client(&self) -> Option<(AnyProtoClient, u64)>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:4857`](../../../../packages/project/src/lsp_store/mod.rs#L4857)

#### Returns

`Option<(AnyProtoClient, u64)>`


***

### new_local()

```rust
pub fn new_local(buffer_store: Entity<BufferStore>, worktree_store: Entity<WorktreeStore>, prettier_store: Entity<PrettierStore>, toolchain_store: Entity<LocalToolchainStore>, environment: Entity<ProjectEnvironment>, manifest_tree: Entity<ManifestTree>, languages: Arc<LanguageRegistry>, http_client: Arc<dyn HttpClient>, fs: Arc<dyn Fs>, cx: &Context<'_, Self>) -> Self
```

Defined in: [`packages/project/src/lsp_store/mod.rs:4873`](../../../../packages/project/src/lsp_store/mod.rs#L4873)

#### Parameters

##### buffer_store

`Entity<BufferStore>`

##### worktree_store

`Entity<WorktreeStore>`

##### prettier_store

`Entity<PrettierStore>`

##### toolchain_store

`Entity<LocalToolchainStore>`

##### environment

`Entity<ProjectEnvironment>`

##### manifest_tree

`Entity<ManifestTree>`

##### languages

`Arc<LanguageRegistry>`

##### http_client

`Arc<dyn HttpClient>`

##### fs

`Arc<dyn Fs>`

##### cx

`&Context<'_, Self>`

#### Returns

`Self`


***

### prettier_store()

```rust
pub fn prettier_store(&self) -> Option<Entity<PrettierStore>>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:5147`](../../../../packages/project/src/lsp_store/mod.rs#L5147)

#### Returns

`Option<Entity<PrettierStore>>`


***

### refresh_background_diagnostics_for_buffers()

```rust
pub fn refresh_background_diagnostics_for_buffers(&self, buffers: HashSet<BufferId>, cx: &Context<'_, Self>) -> Shared<Task<()>>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:5231`](../../../../packages/project/src/lsp_store/mod.rs#L5231)

#### Parameters

##### buffers

`HashSet<BufferId>`

##### cx

`&Context<'_, Self>`

#### Returns

`Shared<Task<()>>`


***

### buffer_store()

```rust
pub fn buffer_store(&self) -> Entity<BufferStore>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:5729`](../../../../packages/project/src/lsp_store/mod.rs#L5729)

#### Returns

`Entity<BufferStore>`


***

### set_active_entry()

```rust
pub fn set_active_entry(&self, active_entry: Option<ProjectEntryId>)
```

Defined in: [`packages/project/src/lsp_store/mod.rs:5733`](../../../../packages/project/src/lsp_store/mod.rs#L5733)

#### Parameters

##### active_entry

`Option<ProjectEntryId>`


***

### relevant_server_ids_for_capability_check()

```rust
pub fn relevant_server_ids_for_capability_check(&self, buffer: &Entity<Buffer>, cx: &App) -> HashSet<LanguageServerId>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:5776`](../../../../packages/project/src/lsp_store/mod.rs#L5776)

#### Parameters

##### buffer

`&Entity<Buffer>`

##### cx

`&App`

#### Returns

`HashSet<LanguageServerId>`


***

### supports_range_formatting()

```rust
pub fn supports_range_formatting(&self, buffer: &Entity<Buffer>, cx: &App) -> bool
```

Defined in: [`packages/project/src/lsp_store/mod.rs:6068`](../../../../packages/project/src/lsp_store/mod.rs#L6068)

#### Parameters

##### buffer

`&Entity<Buffer>`

##### cx

`&App`

#### Returns

`bool`


***

### request_lsp()

```rust
pub fn request_lsp<R>(&self, buffer: Entity<Buffer>, server: LanguageServerToQuery, request: R, cx: &Context<'_, Self>) -> Task<Result<Response>>
where
    R: [LspCommand](../traits/LspCommand.md),
    Result: Send,
    Params: Send
```

Defined in: [`packages/project/src/lsp_store/mod.rs:6152`](../../../../packages/project/src/lsp_store/mod.rs#L6152)

#### Parameters

##### buffer

`Entity<Buffer>`

##### server

[`LanguageServerToQuery`](../enums/LanguageServerToQuery.md)

##### request

`R`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<Response>>`


***

### apply_code_action()

```rust
pub fn apply_code_action(&self, buffer_handle: Entity<Buffer>, action: CodeAction, push_to_history: bool, cx: &Context<'_, Self>) -> Task<Result<ProjectTransaction>>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:6540`](../../../../packages/project/src/lsp_store/mod.rs#L6540)

#### Parameters

##### buffer_handle

`Entity<Buffer>`

##### action

`CodeAction`

##### push_to_history

`bool`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<ProjectTransaction>>`


***

### execute_lsp_command()

```rust
pub fn execute_lsp_command(&self, server_id: LanguageServerId, command: String, arguments: Vec<Value>, cx: &Context<'_, Self>) -> Task<Result<Option<Value>>>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:6659`](../../../../packages/project/src/lsp_store/mod.rs#L6659)

#### Parameters

##### server_id

`LanguageServerId`

##### command

`String`

##### arguments

`Vec<Value>`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<Option<Value>>>`


***

### resolve_code_action()

```rust
pub fn resolve_code_action(&self, buffer: &Entity<Buffer>, action: CodeAction, cx: &Context<'_, Self>) -> Task<Result<CodeAction>>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:6720`](../../../../packages/project/src/lsp_store/mod.rs#L6720)

#### Parameters

##### buffer

`&Entity<Buffer>`

##### action

`CodeAction`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<CodeAction>>`


***

### apply_code_action_kind()

```rust
pub fn apply_code_action_kind(&self, buffers: HashSet<Entity<Buffer>>, kind: CodeActionKind, push_to_history: bool, cx: &Context<'_, Self>) -> Task<Result<ProjectTransaction>>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:6796`](../../../../packages/project/src/lsp_store/mod.rs#L6796)

#### Parameters

##### buffers

`HashSet<Entity<Buffer>>`

##### kind

`CodeActionKind`

##### push_to_history

`bool`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<ProjectTransaction>>`


***

### resolved_hint()

```rust
pub fn resolved_hint(&self, buffer_id: BufferId, id: InlayId, cx: &Context<'_, Self>) -> Option<ResolvedHint>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:6855`](../../../../packages/project/src/lsp_store/mod.rs#L6855)

#### Parameters

##### buffer_id

`BufferId`

##### id

`InlayId`

##### cx

`&Context<'_, Self>`

#### Returns

`Option<ResolvedHint>`


***

### on_type_format()

```rust
pub fn on_type_format<T: ToPointUtf16>(&self, buffer: Entity<Buffer>, position: T, trigger: String, push_to_history: bool, cx: &Context<'_, Self>) -> Option<Task<Result<Option<Transaction>>>>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:7038`](../../../../packages/project/src/lsp_store/mod.rs#L7038)

#### Parameters

##### buffer

`Entity<Buffer>`

##### position

`T`

##### trigger

`String`

##### push_to_history

`bool`

##### cx

`&Context<'_, Self>`

#### Returns

`Option<Task<Result<Option<Transaction>>>>`


***

### definitions()

```rust
pub fn definitions(&self, buffer: &Entity<Buffer>, position: PointUtf16, cx: &Context<'_, Self>) -> Task<Result<Option<Vec<LocationLink>>>>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:7103`](../../../../packages/project/src/lsp_store/mod.rs#L7103)

#### Parameters

##### buffer

`&Entity<Buffer>`

##### position

`PointUtf16`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<Option<Vec<LocationLink>>>>`


***

### edit_prediction_definitions()

```rust
pub fn edit_prediction_definitions(&self, buffer: &Entity<Buffer>, position: PointUtf16, include_type_definitions: bool, cx: &Context<'_, Self>) -> Task<Result<Vec<EditPredictionDefinition>>>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:7243`](../../../../packages/project/src/lsp_store/mod.rs#L7243)

#### Parameters

##### buffer

`&Entity<Buffer>`

##### position

`PointUtf16`

##### include_type_definitions

`bool`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<Vec<EditPredictionDefinition>>>`


***

### declarations()

```rust
pub fn declarations(&self, buffer: &Entity<Buffer>, position: PointUtf16, cx: &Context<'_, Self>) -> Task<Result<Option<Vec<LocationLink>>>>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:7275`](../../../../packages/project/src/lsp_store/mod.rs#L7275)

#### Parameters

##### buffer

`&Entity<Buffer>`

##### position

`PointUtf16`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<Option<Vec<LocationLink>>>>`


***

### type_definitions()

```rust
pub fn type_definitions(&self, buffer: &Entity<Buffer>, position: PointUtf16, cx: &Context<'_, Self>) -> Task<Result<Option<Vec<LocationLink>>>>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:7344`](../../../../packages/project/src/lsp_store/mod.rs#L7344)

#### Parameters

##### buffer

`&Entity<Buffer>`

##### position

`PointUtf16`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<Option<Vec<LocationLink>>>>`


***

### implementations()

```rust
pub fn implementations(&self, buffer: &Entity<Buffer>, position: PointUtf16, cx: &Context<'_, Self>) -> Task<Result<Option<Vec<LocationLink>>>>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:7413`](../../../../packages/project/src/lsp_store/mod.rs#L7413)

#### Parameters

##### buffer

`&Entity<Buffer>`

##### position

`PointUtf16`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<Option<Vec<LocationLink>>>>`


***

### references()

```rust
pub fn references(&self, buffer: &Entity<Buffer>, position: PointUtf16, cx: &Context<'_, Self>) -> Task<Result<Option<Vec<Location>>>>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:7483`](../../../../packages/project/src/lsp_store/mod.rs#L7483)

#### Parameters

##### buffer

`&Entity<Buffer>`

##### position

`PointUtf16`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<Option<Vec<Location>>>>`


***

### prepare_call_hierarchy()

```rust
pub fn prepare_call_hierarchy(&self, buffer: &Entity<Buffer>, position: PointUtf16, cx: &Context<'_, Self>) -> Task<Result<Option<Vec<CallHierarchyItem>>>>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:7551`](../../../../packages/project/src/lsp_store/mod.rs#L7551)

#### Parameters

##### buffer

`&Entity<Buffer>`

##### position

`PointUtf16`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<Option<Vec<CallHierarchyItem>>>>`


***

### incoming_calls()

```rust
pub fn incoming_calls(&self, item: CallHierarchyItem, cx: &Context<'_, Self>) -> Task<Result<Option<Vec<IncomingCall>>>>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:7614`](../../../../packages/project/src/lsp_store/mod.rs#L7614)

#### Parameters

##### item

[`CallHierarchyItem`](CallHierarchyItem.md)

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<Option<Vec<IncomingCall>>>>`


***

### outgoing_calls()

```rust
pub fn outgoing_calls(&self, item: CallHierarchyItem, cx: &Context<'_, Self>) -> Task<Result<Option<Vec<OutgoingCall>>>>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:7670`](../../../../packages/project/src/lsp_store/mod.rs#L7670)

#### Parameters

##### item

[`CallHierarchyItem`](CallHierarchyItem.md)

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<Option<Vec<OutgoingCall>>>>`


***

### code_actions()

```rust
pub fn code_actions(&self, buffer: &Entity<Buffer>, range: Range<Anchor>, kinds: Option<Vec<CodeActionKind>>, cx: &Context<'_, Self>) -> Task<Result<Option<Vec<CodeAction>>>>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:7726`](../../../../packages/project/src/lsp_store/mod.rs#L7726)

#### Parameters

##### buffer

`&Entity<Buffer>`

##### range

`Range<Anchor>`

##### kinds

`Option<Vec<CodeActionKind>>`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<Option<Vec<CodeAction>>>>`


***

### completions()

```rust
pub fn completions(&self, buffer: &Entity<Buffer>, position: PointUtf16, context: CompletionContext, cx: &Context<'_, Self>) -> Task<Result<Vec<CompletionResponse>>>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:7802`](../../../../packages/project/src/lsp_store/mod.rs#L7802)

#### Parameters

##### buffer

`&Entity<Buffer>`

##### position

`PointUtf16`

##### context

`CompletionContext`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<Vec<CompletionResponse>>>`


***

### resolve_completions()

```rust
pub fn resolve_completions(&self, buffer: Entity<Buffer>, completion_indices: Vec<usize>, completions: Rc<RefCell<Box<[Completion]>>>, cx: &Context<'_, Self>) -> Task<Result<bool>>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:7999`](../../../../packages/project/src/lsp_store/mod.rs#L7999)

#### Parameters

##### buffer

`Entity<Buffer>`

##### completion_indices

`Vec<usize>`

##### completions

`Rc<RefCell<Box<[Completion]>>>`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<bool>>`


***

### apply_additional_edits_for_completion()

```rust
pub fn apply_additional_edits_for_completion(&self, buffer_handle: Entity<Buffer>, completions: Rc<RefCell<Box<[Completion]>>>, completion_index: usize, push_to_history: bool, all_commit_ranges: Vec<Range<Anchor>>, cx: &Context<'_, Self>) -> Task<Result<Option<Transaction>>>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:8385`](../../../../packages/project/src/lsp_store/mod.rs#L8385)

#### Parameters

##### buffer_handle

`Entity<Buffer>`

##### completions

`Rc<RefCell<Box<[Completion]>>>`

##### completion_index

`usize`

##### push_to_history

`bool`

##### all_commit_ranges

`Vec<Range<Anchor>>`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<Option<Transaction>>>`


***

### pull_diagnostics()

```rust
pub fn pull_diagnostics(&self, buffer: Entity<Buffer>, cx: &Context<'_, Self>) -> Task<Result<Option<Vec<LspPullDiagnostics>>>>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:8542`](../../../../packages/project/src/lsp_store/mod.rs#L8542)

#### Parameters

##### buffer

`Entity<Buffer>`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<Option<Vec<LspPullDiagnostics>>>>`


***

### applicable_inlay_chunks()

```rust
pub fn applicable_inlay_chunks(&self, buffer: &Entity<Buffer>, ranges: &[Range<Anchor>], cx: &Context<'_, Self>) -> Vec<Range<BufferRow>>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:8662`](../../../../packages/project/src/lsp_store/mod.rs#L8662)

#### Parameters

##### buffer

`&Entity<Buffer>`

##### ranges

`&[Range<Anchor>]`

##### cx

`&Context<'_, Self>`

#### Returns

`Vec<Range<BufferRow>>`


***

### invalidate_inlay_hints()

```rust
pub fn invalidate_inlay_hints<'a, impl IntoIterator<Item = &'a BufferId> + 'a: IntoIterator + 'a>(&self, for_buffers: impl ? + ?)
```

Defined in: [`packages/project/src/lsp_store/mod.rs:8681`](../../../../packages/project/src/lsp_store/mod.rs#L8681)

#### Parameters

##### for_buffers

`impl ? + ?`


***

### inlay_hints()

```rust
pub fn inlay_hints(&self, invalidate: InvalidationStrategy, buffer: Entity<Buffer>, ranges: Vec<Range<Anchor>>, known_chunks: Option<(Global, HashSet<Range<BufferRow>>)>, cx: &Context<'_, Self>) -> HashMap<Range<BufferRow>, Task<Result<CacheInlayHints>>>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:8692`](../../../../packages/project/src/lsp_store/mod.rs#L8692)

#### Parameters

##### invalidate

[`InvalidationStrategy`](../enums/InvalidationStrategy.md)

##### buffer

`Entity<Buffer>`

##### ranges

`Vec<Range<Anchor>>`

##### known_chunks

`Option<(Global, HashSet<Range<BufferRow>>)>`

##### cx

`&Context<'_, Self>`

#### Returns

`HashMap<Range<BufferRow>, Task<Result<CacheInlayHints>>>`


***

### pull_diagnostics_for_buffer()

```rust
pub fn pull_diagnostics_for_buffer(&self, buffer: Entity<Buffer>, cx: &Context<'_, Self>) -> Task<Result<()>>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:8999`](../../../../packages/project/src/lsp_store/mod.rs#L8999)

#### Parameters

##### buffer

`Entity<Buffer>`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<()>>`


***

### signature_help()

```rust
pub fn signature_help<T: ToPointUtf16>(&self, buffer: &Entity<Buffer>, position: T, cx: &Context<'_, Self>) -> Task<Option<Vec<SignatureHelp>>>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:9122`](../../../../packages/project/src/lsp_store/mod.rs#L9122)

#### Parameters

##### buffer

`&Entity<Buffer>`

##### position

`T`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Option<Vec<SignatureHelp>>>`


***

### hover()

```rust
pub fn hover(&self, buffer: &Entity<Buffer>, position: PointUtf16, cx: &Context<'_, Self>) -> Task<Option<Vec<Hover>>>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:9191`](../../../../packages/project/src/lsp_store/mod.rs#L9191)

#### Parameters

##### buffer

`&Entity<Buffer>`

##### position

`PointUtf16`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Option<Vec<Hover>>>`


***

### symbols()

```rust
pub fn symbols(&self, query: &str, cx: &Context<'_, Self>) -> Task<Result<Vec<Symbol>>>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:9264`](../../../../packages/project/src/lsp_store/mod.rs#L9264)

#### Parameters

##### query

`&str`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<Vec<Symbol>>>`


***

### diagnostic_summary()

```rust
pub fn diagnostic_summary(&self, include_ignored: bool, cx: &App) -> DiagnosticSummary
```

Defined in: [`packages/project/src/lsp_store/mod.rs:9469`](../../../../packages/project/src/lsp_store/mod.rs#L9469)

#### Parameters

##### include_ignored

`bool`

##### cx

`&App`

#### Returns

[`DiagnosticSummary`](DiagnosticSummary.md)


***

### diagnostic_summary_for_path()

```rust
pub fn diagnostic_summary_for_path(&self, project_path: &ProjectPath, _: &App) -> DiagnosticSummary
```

Defined in: [`packages/project/src/lsp_store/mod.rs:9479`](../../../../packages/project/src/lsp_store/mod.rs#L9479)

Returns the diagnostic summary for a specific project path.

#### Parameters

##### project_path

`&ProjectPath`

##### _

`&App`

#### Returns

[`DiagnosticSummary`](DiagnosticSummary.md)


***

### diagnostic_summaries()

```rust
pub fn diagnostic_summaries<'a>(&self, include_ignored: bool, cx: &'a App) -> impl ? + ?
```

Defined in: [`packages/project/src/lsp_store/mod.rs:9508`](../../../../packages/project/src/lsp_store/mod.rs#L9508)

#### Parameters

##### include_ignored

`bool`

##### cx

`&'a App`

#### Returns

`impl ? + ?`


***

### on_buffer_edited()

```rust
pub fn on_buffer_edited(&self, buffer: Entity<Buffer>, cx: &Context<'_, Self>) -> Option<()>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:9545`](../../../../packages/project/src/lsp_store/mod.rs#L9545)

#### Parameters

##### buffer

`Entity<Buffer>`

##### cx

`&Context<'_, Self>`

#### Returns

`Option<()>`


***

### on_buffer_saved()

```rust
pub fn on_buffer_saved(&self, buffer: Entity<Buffer>, cx: &Context<'_, Self>) -> Option<()>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:9671`](../../../../packages/project/src/lsp_store/mod.rs#L9671)

#### Parameters

##### buffer

`Entity<Buffer>`

##### cx

`&Context<'_, Self>`

#### Returns

`Option<()>`


***

### running_language_servers_for_local_buffer()

```rust
pub fn running_language_servers_for_local_buffer<'a>(&self, buffer: &Buffer, cx: &App) -> impl ?
```

Defined in: [`packages/project/src/lsp_store/mod.rs:9820`](../../../../packages/project/src/lsp_store/mod.rs#L9820)

#### Parameters

##### buffer

`&Buffer`

##### cx

`&App`

#### Returns

`impl ?`


***

### language_servers_for_local_buffer()

```rust
pub fn language_servers_for_local_buffer(&self, buffer: &Buffer, cx: &App) -> Vec<LanguageServerId>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:9842`](../../../../packages/project/src/lsp_store/mod.rs#L9842)

#### Parameters

##### buffer

`&Buffer`

##### cx

`&App`

#### Returns

`Vec<LanguageServerId>`


***

### language_server_ids_for_opened_buffer()

```rust
pub fn language_server_ids_for_opened_buffer(&self, buffer_id: BufferId) -> Option<&HashSet<LanguageServerId>>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:9853`](../../../../packages/project/src/lsp_store/mod.rs#L9853)

#### Parameters

##### buffer_id

`BufferId`

#### Returns

`Option<&HashSet<LanguageServerId>>`


***

### language_server_for_local_buffer()

```rust
pub fn language_server_for_local_buffer<'a>(&self, buffer: &'a Buffer, server_id: LanguageServerId, cx: &'a App) -> Option<(&'a Arc<CachedLspAdapter>, &'a Arc<LanguageServer>)>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:9860`](../../../../packages/project/src/lsp_store/mod.rs#L9860)

#### Parameters

##### buffer

`&'a Buffer`

##### server_id

`LanguageServerId`

##### cx

`&'a App`

#### Returns

`Option<(&'a Arc<CachedLspAdapter>, &'a Arc<LanguageServer>)>`


***

### shared()

```rust
pub fn shared(&self, project_id: u64, downstream_client: AnyProtoClient, _: &Context<'_, Self>)
```

Defined in: [`packages/project/src/lsp_store/mod.rs:9966`](../../../../packages/project/src/lsp_store/mod.rs#L9966)

#### Parameters

##### project_id

`u64`

##### downstream_client

`AnyProtoClient`

##### _

`&Context<'_, Self>`


***

### disconnected_from_host()

```rust
pub fn disconnected_from_host(&self)
```

Defined in: [`packages/project/src/lsp_store/mod.rs:9998`](../../../../packages/project/src/lsp_store/mod.rs#L9998)


***

### disconnected_from_ssh_remote()

```rust
pub fn disconnected_from_ssh_remote(&self)
```

Defined in: [`packages/project/src/lsp_store/mod.rs:10002`](../../../../packages/project/src/lsp_store/mod.rs#L10002)


***

### merge_diagnostic_entries()

```rust
pub fn merge_diagnostic_entries<'a, impl Fn(&lsp::Uri, &Diagnostic, &App) -> bool + Clone: Fn(&Uri, &Diagnostic, &App) -> bool + Clone>(&self, diagnostic_updates: Vec<DocumentDiagnosticsUpdate<'a, DocumentDiagnostics>>, merge: impl ? + ?, cx: &Context<'_, Self>) -> Result<()>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:10166`](../../../../packages/project/src/lsp_store/mod.rs#L10166)

#### Parameters

##### diagnostic_updates

`Vec<DocumentDiagnosticsUpdate<'a, DocumentDiagnostics>>`

##### merge

`impl ? + ?`

##### cx

`&Context<'_, Self>`

#### Returns

`Result<()>`


***

### open_buffer_for_symbol()

```rust
pub fn open_buffer_for_symbol(&self, symbol: &Symbol, cx: &Context<'_, Self>) -> Task<Result<Entity<Buffer>>>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:10368`](../../../../packages/project/src/lsp_store/mod.rs#L10368)

#### Parameters

##### symbol

`&Symbol`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<Entity<Buffer>>>`


***

### disk_based_diagnostics_started()

```rust
pub fn disk_based_diagnostics_started(&self, language_server_id: LanguageServerId, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/lsp_store/mod.rs:11647`](../../../../packages/project/src/lsp_store/mod.rs#L11647)

#### Parameters

##### language_server_id

`LanguageServerId`

##### cx

`&Context<'_, Self>`


***

### disk_based_diagnostics_finished()

```rust
pub fn disk_based_diagnostics_finished(&self, language_server_id: LanguageServerId, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/lsp_store/mod.rs:11670`](../../../../packages/project/src/lsp_store/mod.rs#L11670)

#### Parameters

##### language_server_id

`LanguageServerId`

##### cx

`&Context<'_, Self>`


***

### language_server_statuses()

```rust
pub fn language_server_statuses(&self) -> impl ?
```

Defined in: [`packages/project/src/lsp_store/mod.rs:11746`](../../../../packages/project/src/lsp_store/mod.rs#L11746)

#### Returns

`impl ?`


***

### language_server_for_id()

```rust
pub fn language_server_for_id(&self, id: LanguageServerId) -> Option<Arc<LanguageServer>>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:11935`](../../../../packages/project/src/lsp_store/mod.rs#L11935)

#### Parameters

##### id

`LanguageServerId`

#### Returns

`Option<Arc<LanguageServer>>`


***

### handle_resolve_completion_documentation()

```rust
pub async fn handle_resolve_completion_documentation(this: Entity<Self>, envelope: TypedEnvelope<ResolveCompletionDocumentation>, cx: AsyncApp) -> Result<ResolveCompletionDocumentationResponse>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:12163`](../../../../packages/project/src/lsp_store/mod.rs#L12163)

#### Parameters

##### this

`Entity<Self>`

##### envelope

`TypedEnvelope<ResolveCompletionDocumentation>`

##### cx

`AsyncApp`

#### Returns

`Result<ResolveCompletionDocumentationResponse>`


***

### handle_get_project_symbols()

```rust
pub async fn handle_get_project_symbols(this: Entity<Self>, envelope: TypedEnvelope<GetProjectSymbols>, cx: AsyncApp) -> Result<GetProjectSymbolsResponse>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:12369`](../../../../packages/project/src/lsp_store/mod.rs#L12369)

#### Parameters

##### this

`Entity<Self>`

##### envelope

`TypedEnvelope<GetProjectSymbols>`

##### cx

`AsyncApp`

#### Returns

`Result<GetProjectSymbolsResponse>`


***

### handle_restart_language_servers()

```rust
pub async fn handle_restart_language_servers(this: Entity<Self>, envelope: TypedEnvelope<RestartLanguageServers>, cx: AsyncApp) -> Result<Ack>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:12385`](../../../../packages/project/src/lsp_store/mod.rs#L12385)

#### Parameters

##### this

`Entity<Self>`

##### envelope

`TypedEnvelope<RestartLanguageServers>`

##### cx

`AsyncApp`

#### Returns

`Result<Ack>`


***

### handle_stop_language_servers()

```rust
pub async fn handle_stop_language_servers(lsp_store: Entity<Self>, envelope: TypedEnvelope<StopLanguageServers>, cx: AsyncApp) -> Result<Ack>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:12420`](../../../../packages/project/src/lsp_store/mod.rs#L12420)

#### Parameters

##### lsp_store

`Entity<Self>`

##### envelope

`TypedEnvelope<StopLanguageServers>`

##### cx

`AsyncApp`

#### Returns

`Result<Ack>`


***

### handle_cancel_language_server_work()

```rust
pub async fn handle_cancel_language_server_work(lsp_store: Entity<Self>, envelope: TypedEnvelope<CancelLanguageServerWork>, cx: AsyncApp) -> Result<Ack>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:12465`](../../../../packages/project/src/lsp_store/mod.rs#L12465)

#### Parameters

##### lsp_store

`Entity<Self>`

##### envelope

`TypedEnvelope<CancelLanguageServerWork>`

##### cx

`AsyncApp`

#### Returns

`Result<Ack>`


***

### last_formatting_failure()

```rust
pub fn last_formatting_failure(&self) -> Option<&str>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:12564`](../../../../packages/project/src/lsp_store/mod.rs#L12564)

#### Returns

`Option<&str>`


***

### reset_last_formatting_failure()

```rust
pub fn reset_last_formatting_failure(&self)
```

Defined in: [`packages/project/src/lsp_store/mod.rs:12568`](../../../../packages/project/src/lsp_store/mod.rs#L12568)


***

### environment_for_buffer()

```rust
pub fn environment_for_buffer(&self, buffer: &Entity<Buffer>, cx: &Context<'_, Self>) -> Shared<Task<Option<HashMap<String, String>>>>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:12572`](../../../../packages/project/src/lsp_store/mod.rs#L12572)

#### Parameters

##### buffer

`&Entity<Buffer>`

##### cx

`&Context<'_, Self>`

#### Returns

`Shared<Task<Option<HashMap<String, String>>>>`


***

### format()

```rust
pub fn format(&self, buffers: HashSet<Entity<Buffer>>, target: LspFormatTarget, push_to_history: bool, trigger: FormatTrigger, cx: &Context<'_, Self>) -> Task<Result<ProjectTransaction>>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:12586`](../../../../packages/project/src/lsp_store/mod.rs#L12586)

#### Parameters

##### buffers

`HashSet<Entity<Buffer>>`

##### target

[`LspFormatTarget`](../enums/LspFormatTarget.md)

##### push_to_history

`bool`

##### trigger

[`FormatTrigger`](../enums/FormatTrigger.md)

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<ProjectTransaction>>`


***

### stop_all_language_servers()

```rust
pub fn stop_all_language_servers(&self, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/lsp_store/mod.rs:12963`](../../../../packages/project/src/lsp_store/mod.rs#L12963)

#### Parameters

##### cx

`&Context<'_, Self>`


***

### shutdown_all_language_servers()

```rust
pub fn shutdown_all_language_servers(&self, cx: &Context<'_, Self>) -> Task<()>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:12970`](../../../../packages/project/src/lsp_store/mod.rs#L12970)

#### Parameters

##### cx

`&Context<'_, Self>`

#### Returns

`Task<()>`


***

### restart_all_language_servers()

```rust
pub fn restart_all_language_servers(&self, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/lsp_store/mod.rs:13001`](../../../../packages/project/src/lsp_store/mod.rs#L13001)

#### Parameters

##### cx

`&Context<'_, Self>`


***

### restart_language_servers_for_buffers()

```rust
pub fn restart_language_servers_for_buffers(&self, buffers: Vec<Entity<Buffer>>, only_restart_servers: HashSet<LanguageServerSelector>, clear_stopped: bool, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/lsp_store/mod.rs:13011`](../../../../packages/project/src/lsp_store/mod.rs#L13011)

#### Parameters

##### buffers

`Vec<Entity<Buffer>>`

##### only_restart_servers

`HashSet<LanguageServerSelector>`

##### clear_stopped

`bool`

##### cx

`&Context<'_, Self>`


***

### stop_language_servers_for_buffers()

```rust
pub fn stop_language_servers_for_buffers(&self, buffers: Vec<Entity<Buffer>>, also_stop_servers: HashSet<LanguageServerSelector>, cx: &Context<'_, Self>) -> Task<Result<()>>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:13090`](../../../../packages/project/src/lsp_store/mod.rs#L13090)

#### Parameters

##### buffers

`Vec<Entity<Buffer>>`

##### also_stop_servers

`HashSet<LanguageServerSelector>`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<()>>`


***

### merge_lsp_diagnostics()

```rust
pub fn merge_lsp_diagnostics<impl Fn(&lsp::Uri, &Diagnostic, &App) -> bool + Clone: Fn(&Uri, &Diagnostic, &App) -> bool + Clone>(&self, source_kind: DiagnosticSourceKind, lsp_diagnostics: Vec<DocumentDiagnosticsUpdate<'_, PublishDiagnosticsParams>>, merge: impl ? + ?, cx: &Context<'_, Self>) -> Result<()>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:13238`](../../../../packages/project/src/lsp_store/mod.rs#L13238)

#### Parameters

##### source_kind

`DiagnosticSourceKind`

##### lsp_diagnostics

`Vec<DocumentDiagnosticsUpdate<'_, PublishDiagnosticsParams>>`

##### merge

`impl ? + ?`

##### cx

`&Context<'_, Self>`

#### Returns

`Result<()>`


***

### language_servers_running_disk_based_diagnostics()

```rust
pub fn language_servers_running_disk_based_diagnostics(&self) -> impl ? + ?
```

Defined in: [`packages/project/src/lsp_store/mod.rs:13691`](../../../../packages/project/src/lsp_store/mod.rs#L13691)

#### Returns

`impl ? + ?`


***

### language_server_adapter_for_id()

```rust
pub fn language_server_adapter_for_id(&self, id: LanguageServerId) -> Option<Arc<CachedLspAdapter>>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:13832`](../../../../packages/project/src/lsp_store/mod.rs#L13832)

#### Parameters

##### id

`LanguageServerId`

#### Returns

`Option<Arc<CachedLspAdapter>>`


***

### wait_for_remote_buffer()

```rust
pub fn wait_for_remote_buffer(&self, id: BufferId, cx: &Context<'_, Self>) -> Task<Result<Entity<Buffer>>>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:13924`](../../../../packages/project/src/lsp_store/mod.rs#L13924)

#### Parameters

##### id

`BufferId`

##### cx

`&Context<'_, Self>`

#### Returns

`Task<Result<Entity<Buffer>>>`


***

### result_id_for_buffer_pull()

```rust
pub fn result_id_for_buffer_pull(&self, server_id: LanguageServerId, buffer_id: BufferId, registration_id: &Option<SharedString>, cx: &App) -> Option<SharedString>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:14207`](../../../../packages/project/src/lsp_store/mod.rs#L14207)

#### Parameters

##### server_id

`LanguageServerId`

##### buffer_id

`BufferId`

##### registration_id

`&Option<SharedString>`

##### cx

`&App`

#### Returns

`Option<SharedString>`


***

### result_ids_for_workspace_refresh()

```rust
pub fn result_ids_for_workspace_refresh(&self, server_id: LanguageServerId, registration_id: &Option<SharedString>) -> HashMap<PathBuf, SharedString>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:14252`](../../../../packages/project/src/lsp_store/mod.rs#L14252)

#### Parameters

##### server_id

`LanguageServerId`

##### registration_id

`&Option<SharedString>`

#### Returns

`HashMap<PathBuf, SharedString>`


***

### pull_workspace_diagnostics()

```rust
pub fn pull_workspace_diagnostics(&self, server_id: LanguageServerId)
```

Defined in: [`packages/project/src/lsp_store/mod.rs:14281`](../../../../packages/project/src/lsp_store/mod.rs#L14281)

#### Parameters

##### server_id

`LanguageServerId`


***

### pull_workspace_diagnostics_once()

```rust
pub fn pull_workspace_diagnostics_once(&self, cx: &Context<'_, Self>) -> Task<bool>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:14301`](../../../../packages/project/src/lsp_store/mod.rs#L14301)

Triggers a workspace diagnostics pull on all running language servers
and returns a `Task` that resolves once the requests have completed.

This reuses the same background refresh loops as
[`Self::pull_workspace_diagnostics`](LspStore.md), but provides a completion signal
so callers can wait for fresh diagnostics before reading them.

#### Parameters

##### cx

`&Context<'_, Self>`

#### Returns

`Task<bool>`


***

### pull_document_diagnostics_for_server()

```rust
pub fn pull_document_diagnostics_for_server(&self, server_id: LanguageServerId, source_buffer_id: Option<BufferId>, cx: &Context<'_, Self>) -> Shared<Task<()>>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:14332`](../../../../packages/project/src/lsp_store/mod.rs#L14332)

Refreshes `textDocument/diagnostic` for all open buffers associated with the given server.
This is called in response to `workspace/diagnostic/refresh` to comply with the LSP spec,
which requires refreshing both workspace and document diagnostics.

#### Parameters

##### server_id

`LanguageServerId`

##### source_buffer_id

`Option<BufferId>`

##### cx

`&Context<'_, Self>`

#### Returns

`Shared<Task<()>>`


***

### pull_document_diagnostics_for_buffer_edit()

```rust
pub fn pull_document_diagnostics_for_buffer_edit(&self, buffer_id: BufferId, cx: &Context<'_, Self>)
```

Defined in: [`packages/project/src/lsp_store/mod.rs:14351`](../../../../packages/project/src/lsp_store/mod.rs#L14351)

#### Parameters

##### buffer_id

`BufferId`

##### cx

`&Context<'_, Self>`


***

### downstream_client()

```rust
pub fn downstream_client(&self) -> Option<(AnyProtoClient, u64)>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:14841`](../../../../packages/project/src/lsp_store/mod.rs#L14841)

#### Returns

`Option<(AnyProtoClient, u64)>`


***

### worktree_store()

```rust
pub fn worktree_store(&self) -> Entity<WorktreeStore>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:14845`](../../../../packages/project/src/lsp_store/mod.rs#L14845)

#### Returns

`Entity<WorktreeStore>`


***

### current_lsp_data()

```rust
pub fn current_lsp_data(&self, buffer_id: BufferId) -> Option<&BufferLspData>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:14850`](../../../../packages/project/src/lsp_store/mod.rs#L14850)

Gets what's stored in the LSP data for the given buffer.

#### Parameters

##### buffer_id

`BufferId`

#### Returns

`Option<&BufferLspData>`


***

### latest_lsp_data()

```rust
pub fn latest_lsp_data(&self, buffer: &Entity<Buffer>, cx: &App) -> &BufferLspData
```

Defined in: [`packages/project/src/lsp_store/mod.rs:14856`](../../../../packages/project/src/lsp_store/mod.rs#L14856)

Gets the most recent LSP data for the given buffer: if the data is absent or out of date,
new [`BufferLspData`](BufferLspData.md) will be created to replace the previous state.

#### Parameters

##### buffer

`&Entity<Buffer>`

##### cx

`&App`

#### Returns

`&BufferLspData`

## Trait Implementations

- `impl Borrow for LspStore`
- `impl BorrowMut for LspStore`
- `impl Into for LspStore`
- `impl From for LspStore`
- `impl TryInto for LspStore`
- `impl TryFrom for LspStore`
- `impl Any for LspStore`
- `impl VZip for LspStore`
- `impl CastableFrom for LspStore`
- `impl CastableFrom for LspStore`
- `impl Read for LspStore`
- `impl IntoEither for LspStore`
- `impl ErasedDestructor for LspStore`
- `impl Same for LspStore`
- `impl Pointable for LspStore`
- `impl Instrument for LspStore`
- `impl WithSubscriber for LspStore`
- `impl FromAngle for LspStore`
- `impl IntoAngle for LspStore`
- `impl IntoCam16Unclamped for LspStore`
- `impl Cam16IntoUnclamped for LspStore`
- `impl ArraysFrom for LspStore`
- `impl ArraysInto for LspStore`
- `impl ComponentsFrom for LspStore`
- `impl TryComponentsInto for LspStore`
- `impl UintsFrom for LspStore`
- `impl UintsInto for LspStore`
- `impl AdaptIntoUnclamped for LspStore`
- `impl AdaptInto for LspStore`
- `impl IntoColor for LspStore`
- `impl IntoColorUnclamped for LspStore`
- `impl TryIntoColor for LspStore`
- `impl FromStimulus for LspStore`
- `impl IntoStimulus for LspStore`
- `impl EventEmitter for LspStore`

## Auto Trait Implementations

`Freeze` `Unpin` `UnsafeUnpin`

