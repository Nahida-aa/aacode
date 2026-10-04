---
id: CodeLensResolveTask
title: CodeLensResolveTask
---

# Type Alias: CodeLensResolveTask

Defined in: [`packages/project/src/lsp_store/code_lens.rs:48`](../../../../packages/project/src/lsp_store/code_lens.rs#L48)

## Definition

```rust
pub type CodeLensResolveTask = Shared<Task<Option<(CodeLensActionId, CodeAction)>>>
```

