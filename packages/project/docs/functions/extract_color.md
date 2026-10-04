---
id: extract_color
title: extract_color
---

# Function: extract_color()

```rust
pub fn extract_color(item: &CompletionItem) -> Option<Hsla>
```

Defined in: [`packages/project/src/color_extractor.rs:41`](../../../../packages/project/src/color_extractor.rs#L41)

Extracts a color from an LSP `CompletionItem`.

Adapted from https://github.com/microsoft/vscode/blob/a6870fcb6d79093738c17e8319b760cf1c41764a/src/vs/editor/contrib/suggest/browser/suggestWidgetRenderer.ts#L34-L61

## Parameters

### item

`&CompletionItem`

## Returns

`Option<Hsla>`

