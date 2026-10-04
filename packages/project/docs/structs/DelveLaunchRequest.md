---
id: DelveLaunchRequest
title: DelveLaunchRequest
---

# Struct: DelveLaunchRequest

Defined in: [`packages/project/src/debugger/locators/go.rs:13`](../../../../packages/project/src/debugger/locators/go.rs#L13)

## Definition

```rust
pub struct DelveLaunchRequest
{
    pub request: String,
    pub mode: String,
    pub program: String,
    pub cwd: Option<String>,
    pub args: Vec<String>,
    pub build_flags: Vec<String>,
    pub env: HashMap<String, String>,
}
```

## Trait Implementations

- `impl Borrow for DelveLaunchRequest`
- `impl BorrowMut for DelveLaunchRequest`
- `impl Into for DelveLaunchRequest`
- `impl From for DelveLaunchRequest`
- `impl TryInto for DelveLaunchRequest`
- `impl TryFrom for DelveLaunchRequest`
- `impl Any for DelveLaunchRequest`
- `impl DeserializeOwned for DelveLaunchRequest`
- `impl Equivalent for DelveLaunchRequest`
- `impl Serialize for DelveLaunchRequest`
- `impl VZip for DelveLaunchRequest`
- `impl CastableFrom for DelveLaunchRequest`
- `impl CastableFrom for DelveLaunchRequest`
- `impl Read for DelveLaunchRequest`
- `impl IntoEither for DelveLaunchRequest`
- `impl ErasedDestructor for DelveLaunchRequest`
- `impl Same for DelveLaunchRequest`
- `impl Pointable for DelveLaunchRequest`
- `impl Instrument for DelveLaunchRequest`
- `impl WithSubscriber for DelveLaunchRequest`
- `impl FromAngle for DelveLaunchRequest`
- `impl IntoAngle for DelveLaunchRequest`
- `impl IntoCam16Unclamped for DelveLaunchRequest`
- `impl Cam16IntoUnclamped for DelveLaunchRequest`
- `impl ArraysFrom for DelveLaunchRequest`
- `impl ArraysInto for DelveLaunchRequest`
- `impl ComponentsFrom for DelveLaunchRequest`
- `impl TryComponentsInto for DelveLaunchRequest`
- `impl UintsFrom for DelveLaunchRequest`
- `impl UintsInto for DelveLaunchRequest`
- `impl AdaptIntoUnclamped for DelveLaunchRequest`
- `impl AdaptInto for DelveLaunchRequest`
- `impl IntoColor for DelveLaunchRequest`
- `impl IntoColorUnclamped for DelveLaunchRequest`
- `impl TryIntoColor for DelveLaunchRequest`
- `impl FromStimulus for DelveLaunchRequest`
- `impl IntoStimulus for DelveLaunchRequest`
- `impl Equivalent for DelveLaunchRequest`
- `impl Serialize for DelveLaunchRequest`
- `impl Deserialize for DelveLaunchRequest`
- `impl Debug for DelveLaunchRequest`
- `impl StructuralPartialEq for DelveLaunchRequest`
- `impl PartialEq for DelveLaunchRequest`
- `impl Eq for DelveLaunchRequest`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

