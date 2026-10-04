---
id: language_server_settings_for
title: language_server_settings_for
---

# Function: language_server_settings_for()

```rust
pub fn language_server_settings_for<'a>(location: SettingsLocation<'a>, language: &LanguageServerName, cx: &'a App) -> Option<&'a LspSettings>
```

Defined in: [`packages/project/src/lsp_store/mod.rs:16394`](../../../../packages/project/src/lsp_store/mod.rs#L16394)

## Parameters

### location

`SettingsLocation<'a>`

### language

`&LanguageServerName`

### cx

`&'a App`

## Returns

`Option<&'a LspSettings>`

