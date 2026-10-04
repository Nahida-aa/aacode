---
id: SemanticTokenStylizer
title: SemanticTokenStylizer
---

# Struct: SemanticTokenStylizer

Defined in: [`packages/project/src/lsp_store/semantic_tokens.rs:652`](../../../../packages/project/src/lsp_store/semantic_tokens.rs#L652)

## Definition

```rust
pub struct SemanticTokenStylizer
```

_（存在非公开字段）_

## Implementations

### new()

```rust
pub fn new(server_id: LanguageServerId, legend: &SemanticTokensLegend, language_rules: Option<&SemanticTokenRules>, cx: &App) -> Self
```

Defined in: [`packages/project/src/lsp_store/semantic_tokens.rs:660`](../../../../packages/project/src/lsp_store/semantic_tokens.rs#L660)

#### Parameters

##### server_id

`LanguageServerId`

##### legend

`&SemanticTokensLegend`

##### language_rules

`Option<&SemanticTokenRules>`

##### cx

`&App`

#### Returns

`Self`


***

### server_id()

```rust
pub fn server_id(&self) -> LanguageServerId
```

Defined in: [`packages/project/src/lsp_store/semantic_tokens.rs:718`](../../../../packages/project/src/lsp_store/semantic_tokens.rs#L718)

#### Returns

`LanguageServerId`


***

### token_type_name()

```rust
pub fn token_type_name(&self, token_type: TokenType) -> Option<&SharedString>
```

Defined in: [`packages/project/src/lsp_store/semantic_tokens.rs:722`](../../../../packages/project/src/lsp_store/semantic_tokens.rs#L722)

#### Parameters

##### token_type

[`TokenType`](TokenType.md)

#### Returns

`Option<&SharedString>`


***

### has_modifier()

```rust
pub fn has_modifier(&self, token_modifiers: u32, modifier: &str) -> bool
```

Defined in: [`packages/project/src/lsp_store/semantic_tokens.rs:726`](../../../../packages/project/src/lsp_store/semantic_tokens.rs#L726)

#### Parameters

##### token_modifiers

`u32`

##### modifier

`&str`

#### Returns

`bool`


***

### token_modifiers()

```rust
pub fn token_modifiers(&self, token_modifiers: u32) -> Option<String>
```

Defined in: [`packages/project/src/lsp_store/semantic_tokens.rs:733`](../../../../packages/project/src/lsp_store/semantic_tokens.rs#L733)

#### Parameters

##### token_modifiers

`u32`

#### Returns

`Option<String>`


***

### rules_for_token()

```rust
pub fn rules_for_token(&self, token_type: TokenType) -> Option<&[SemanticTokenRule]>
```

Defined in: [`packages/project/src/lsp_store/semantic_tokens.rs:747`](../../../../packages/project/src/lsp_store/semantic_tokens.rs#L747)

#### Parameters

##### token_type

[`TokenType`](TokenType.md)

#### Returns

`Option<&[SemanticTokenRule]>`

## Trait Implementations

- `impl Borrow for SemanticTokenStylizer`
- `impl BorrowMut for SemanticTokenStylizer`
- `impl Into for SemanticTokenStylizer`
- `impl From for SemanticTokenStylizer`
- `impl TryInto for SemanticTokenStylizer`
- `impl TryFrom for SemanticTokenStylizer`
- `impl Any for SemanticTokenStylizer`
- `impl VZip for SemanticTokenStylizer`
- `impl CastableFrom for SemanticTokenStylizer`
- `impl CastableFrom for SemanticTokenStylizer`
- `impl Read for SemanticTokenStylizer`
- `impl IntoEither for SemanticTokenStylizer`
- `impl ErasedDestructor for SemanticTokenStylizer`
- `impl Same for SemanticTokenStylizer`
- `impl Pointable for SemanticTokenStylizer`
- `impl Instrument for SemanticTokenStylizer`
- `impl WithSubscriber for SemanticTokenStylizer`
- `impl FromAngle for SemanticTokenStylizer`
- `impl IntoAngle for SemanticTokenStylizer`
- `impl IntoCam16Unclamped for SemanticTokenStylizer`
- `impl Cam16IntoUnclamped for SemanticTokenStylizer`
- `impl ArraysFrom for SemanticTokenStylizer`
- `impl ArraysInto for SemanticTokenStylizer`
- `impl ComponentsFrom for SemanticTokenStylizer`
- `impl TryComponentsInto for SemanticTokenStylizer`
- `impl UintsFrom for SemanticTokenStylizer`
- `impl UintsInto for SemanticTokenStylizer`
- `impl AdaptIntoUnclamped for SemanticTokenStylizer`
- `impl AdaptInto for SemanticTokenStylizer`
- `impl IntoColor for SemanticTokenStylizer`
- `impl IntoColorUnclamped for SemanticTokenStylizer`
- `impl TryIntoColor for SemanticTokenStylizer`
- `impl FromStimulus for SemanticTokenStylizer`
- `impl IntoStimulus for SemanticTokenStylizer`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

