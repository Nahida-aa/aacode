---
id: GetDocumentDiagnostics
title: GetDocumentDiagnostics
---

# Struct: GetDocumentDiagnostics

Defined in: [`packages/project/src/lsp_command/mod.rs:325`](../../../../packages/project/src/lsp_command/mod.rs#L325)

## Definition

```rust
pub struct GetDocumentDiagnostics {
    // We cannot blindly rely on server's capabilities.diagnostic_provider, as they're a singular field, whereas a server can register multiple diagnostic providers post-mortem.
    pub registration_id: Option<SharedString>,
    pub identifier: Option<SharedString>,
    pub previous_result_id: Option<SharedString>,
}
```

## Fields

### registration_id

Defined in: [`packages/project/src/lsp_command/mod.rs:328`](../../../../packages/project/src/lsp_command/mod.rs#L328)

We cannot blindly rely on server's capabilities.diagnostic_provider, as they're a singular field, whereas
a server can register multiple diagnostic providers post-mortem.

## Implementations

### diagnostics_from_proto()

```rust
pub fn diagnostics_from_proto(response: GetDocumentDiagnosticsResponse) -> Vec<LspPullDiagnostics>
```

Defined in: [`packages/project/src/lsp_command/mod.rs:5060`](../../../../packages/project/src/lsp_command/mod.rs#L5060)

#### Parameters

##### response

`GetDocumentDiagnosticsResponse`

#### Returns

`Vec<LspPullDiagnostics>`


***

### deserialize_lsp_diagnostic()

```rust
pub fn deserialize_lsp_diagnostic(diagnostic: LspDiagnostic) -> Result<Diagnostic>
```

Defined in: [`packages/project/src/lsp_command/mod.rs:5094`](../../../../packages/project/src/lsp_command/mod.rs#L5094)

#### Parameters

##### diagnostic

`LspDiagnostic`

#### Returns

`Result<Diagnostic>`


***

### serialize_lsp_diagnostic()

```rust
pub fn serialize_lsp_diagnostic(diagnostic: Diagnostic) -> Result<LspDiagnostic>
```

Defined in: [`packages/project/src/lsp_command/mod.rs:5181`](../../../../packages/project/src/lsp_command/mod.rs#L5181)

#### Parameters

##### diagnostic

`Diagnostic`

#### Returns

`Result<LspDiagnostic>`


***

### deserialize_workspace_diagnostics_report()

```rust
pub fn deserialize_workspace_diagnostics_report(report: WorkspaceDiagnosticReportResult, server_id: LanguageServerId, registration_id: Option<SharedString>) -> Vec<WorkspaceLspPullDiagnostics>
```

Defined in: [`packages/project/src/lsp_command/mod.rs:5258`](../../../../packages/project/src/lsp_command/mod.rs#L5258)

#### Parameters

##### report

`WorkspaceDiagnosticReportResult`

##### server_id

`LanguageServerId`

##### registration_id

`Option<SharedString>`

#### Returns

`Vec<WorkspaceLspPullDiagnostics>`

## Trait Implementations

- `impl Borrow for GetDocumentDiagnostics`
- `impl BorrowMut for GetDocumentDiagnostics`
- `impl CloneToUninit for GetDocumentDiagnostics`
- `impl Into for GetDocumentDiagnostics`
- `impl From for GetDocumentDiagnostics`
- `impl TryInto for GetDocumentDiagnostics`
- `impl TryFrom for GetDocumentDiagnostics`
- `impl Any for GetDocumentDiagnostics`
- `impl ToOwned for GetDocumentDiagnostics`
- `impl DynClone for GetDocumentDiagnostics`
- `impl VZip for GetDocumentDiagnostics`
- `impl CastableFrom for GetDocumentDiagnostics`
- `impl CastableFrom for GetDocumentDiagnostics`
- `impl Read for GetDocumentDiagnostics`
- `impl IntoEither for GetDocumentDiagnostics`
- `impl ErasedDestructor for GetDocumentDiagnostics`
- `impl Same for GetDocumentDiagnostics`
- `impl Pointable for GetDocumentDiagnostics`
- `impl Instrument for GetDocumentDiagnostics`
- `impl WithSubscriber for GetDocumentDiagnostics`
- `impl FromAngle for GetDocumentDiagnostics`
- `impl IntoAngle for GetDocumentDiagnostics`
- `impl IntoCam16Unclamped for GetDocumentDiagnostics`
- `impl Cam16IntoUnclamped for GetDocumentDiagnostics`
- `impl ArraysFrom for GetDocumentDiagnostics`
- `impl ArraysInto for GetDocumentDiagnostics`
- `impl ComponentsFrom for GetDocumentDiagnostics`
- `impl TryComponentsInto for GetDocumentDiagnostics`
- `impl UintsFrom for GetDocumentDiagnostics`
- `impl UintsInto for GetDocumentDiagnostics`
- `impl AdaptIntoUnclamped for GetDocumentDiagnostics`
- `impl AdaptInto for GetDocumentDiagnostics`
- `impl IntoColor for GetDocumentDiagnostics`
- `impl IntoColorUnclamped for GetDocumentDiagnostics`
- `impl TryIntoColor for GetDocumentDiagnostics`
- `impl FromStimulus for GetDocumentDiagnostics`
- `impl IntoStimulus for GetDocumentDiagnostics`
- `impl Clone for GetDocumentDiagnostics`
- `impl Debug for GetDocumentDiagnostics`
- `impl LspCommand for GetDocumentDiagnostics`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

