---
id: language_server_settings
title: language_server_settings
---

# Function: language_server_settings()

```rust
pub fn language_server_settings<'a>(delegate: &'a dyn LspAdapterDelegate, language: &LanguageServerName, cx: &'a App) -> Option<&'a LspSettings>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:16379`](../../../../packages/project/src/lsp_store/mod.rs#L16379)

## Parameters

### delegate

`&'a dyn LspAdapterDelegate`

### language

`&LanguageServerName`

### cx

`&'a App`

## Returns

`Option<&'a LspSettings>`

