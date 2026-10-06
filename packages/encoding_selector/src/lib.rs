mod encoding_selector;

// Zed 的 crate root 就是 `src/encoding_selector.rs` 本身（见其 Cargo.toml 的
// `[lib] path`），故 `encoding_selector::init` 在上游是 crate 根的公开函数。
// 本 fork 拆成 `lib.rs` + 子模块后必须显式再导出，否则 app 侧
// `encoding_selector::init(cx)` 找不到 —— 而这种「缺 re-export」不产生任何
// 编译错误提示，只在接线的那一行才炸。
pub use encoding_selector::{init, ActiveBufferEncoding};
