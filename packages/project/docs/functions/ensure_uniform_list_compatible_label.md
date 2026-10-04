---
id: ensure_uniform_list_compatible_label
title: ensure_uniform_list_compatible_label
---

# Function: ensure_uniform_list_compatible_label()

```rust
pub fn ensure_uniform_list_compatible_label(label: &CodeLabel)
```

Defined in: [`packages/project/src/lsp_store/mod.rs:16745`](../../../../packages/project/src/lsp_store/mod.rs#L16745)

Completion items are displayed in a `UniformList`.
Usually, those items are single-line strings, but in LSP responses,
completion items `label`, `detail` and `label_details.description` may contain newlines or long spaces.
Many language plugins construct these items by joining these parts together, and we may use `CodeLabel::fallback_for_completion` that uses `label` at least.
All that may lead to a newline being inserted into resulting `CodeLabel.text`, which will force `UniformList` to bloat each entry to occupy more space,
breaking the completions menu presentation.

Sanitize the text to ensure there are no newlines, or, if there are some, remove them and also remove long space sequences if there were newlines.

## Parameters

### label

`&CodeLabel`

