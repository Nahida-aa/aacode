---
id: ProjectSettings
title: ProjectSettings
---

# Struct: ProjectSettings

Defined in: [`packages/project/src/project_settings.rs:41`](../../../../packages/project/src/project_settings.rs#L41)

## Definition

```rust
pub struct ProjectSettings {
    // Configuration for language servers. The following settings can be overridden for specific language servers: - initialization_options To override settings for a language, add an entry for that language server's name to the lsp value. Default: null
    pub lsp: HashMap<LanguageServerName, LspSettings>,
    // Common language server settings.
    pub global_lsp_settings: GlobalLspSettings,
    // Configuration for Debugger-related features
    pub dap: HashMap<DebugAdapterName, DapSettings>,
    // Settings for context servers used for AI-related features.
    pub context_servers: HashMap<Arc<str>, ContextServerSettings>,
    // Default timeout for context server requests in seconds.
    pub context_server_timeout: u64,
    // Configuration for Diagnostics-related features.
    pub diagnostics: DiagnosticsSettings,
    // Configuration for Git-related features
    pub git: GitSettings,
    // Configuration for Node-related features
    pub node: NodeBinarySettings,
    // Configuration for how direnv configuration should be loaded
    pub load_direnv: DirenvSettings,
    // Configuration for session-related features
    pub session: SessionSettings,
}
```

## Fields

### lsp

Defined in: [`packages/project/src/project_settings.rs:54`](../../../../packages/project/src/project_settings.rs#L54)

Configuration for language servers.

The following settings can be overridden for specific language servers:
- initialization_options

To override settings for a language, add an entry for that language server's
name to the lsp value.
Default: null


***

### global_lsp_settings

Defined in: [`packages/project/src/project_settings.rs:57`](../../../../packages/project/src/project_settings.rs#L57)

Common language server settings.


***

### dap

Defined in: [`packages/project/src/project_settings.rs:60`](../../../../packages/project/src/project_settings.rs#L60)

Configuration for Debugger-related features


***

### context_servers

Defined in: [`packages/project/src/project_settings.rs:63`](../../../../packages/project/src/project_settings.rs#L63)

Settings for context servers used for AI-related features.


***

### context_server_timeout

Defined in: [`packages/project/src/project_settings.rs:66`](../../../../packages/project/src/project_settings.rs#L66)

Default timeout for context server requests in seconds.


***

### diagnostics

Defined in: [`packages/project/src/project_settings.rs:69`](../../../../packages/project/src/project_settings.rs#L69)

Configuration for Diagnostics-related features.


***

### git

Defined in: [`packages/project/src/project_settings.rs:72`](../../../../packages/project/src/project_settings.rs#L72)

Configuration for Git-related features


***

### node

Defined in: [`packages/project/src/project_settings.rs:75`](../../../../packages/project/src/project_settings.rs#L75)

Configuration for Node-related features


***

### load_direnv

Defined in: [`packages/project/src/project_settings.rs:78`](../../../../packages/project/src/project_settings.rs#L78)

Configuration for how direnv configuration should be loaded


***

### session

Defined in: [`packages/project/src/project_settings.rs:81`](../../../../packages/project/src/project_settings.rs#L81)

Configuration for session-related features

## Trait Implementations

- `impl Borrow for ProjectSettings`
- `impl BorrowMut for ProjectSettings`
- `impl CloneToUninit for ProjectSettings`
- `impl Into for ProjectSettings`
- `impl From for ProjectSettings`
- `impl TryInto for ProjectSettings`
- `impl TryFrom for ProjectSettings`
- `impl Any for ProjectSettings`
- `impl ToOwned for ProjectSettings`
- `impl DynClone for ProjectSettings`
- `impl VZip for ProjectSettings`
- `impl CastableFrom for ProjectSettings`
- `impl CastableFrom for ProjectSettings`
- `impl Read for ProjectSettings`
- `impl IntoEither for ProjectSettings`
- `impl ErasedDestructor for ProjectSettings`
- `impl Same for ProjectSettings`
- `impl Pointable for ProjectSettings`
- `impl Instrument for ProjectSettings`
- `impl WithSubscriber for ProjectSettings`
- `impl FromAngle for ProjectSettings`
- `impl IntoAngle for ProjectSettings`
- `impl IntoCam16Unclamped for ProjectSettings`
- `impl Cam16IntoUnclamped for ProjectSettings`
- `impl ArraysFrom for ProjectSettings`
- `impl ArraysInto for ProjectSettings`
- `impl ComponentsFrom for ProjectSettings`
- `impl TryComponentsInto for ProjectSettings`
- `impl UintsFrom for ProjectSettings`
- `impl UintsInto for ProjectSettings`
- `impl AdaptIntoUnclamped for ProjectSettings`
- `impl AdaptInto for ProjectSettings`
- `impl IntoColor for ProjectSettings`
- `impl IntoColorUnclamped for ProjectSettings`
- `impl TryIntoColor for ProjectSettings`
- `impl FromStimulus for ProjectSettings`
- `impl IntoStimulus for ProjectSettings`
- `impl Debug for ProjectSettings`
- `impl Clone for ProjectSettings`
- `impl Settings for ProjectSettings`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

