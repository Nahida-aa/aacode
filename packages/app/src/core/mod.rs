//! 全局 init 链 — 对齐 Zed `crates/zed/src/main.rs`。
//!
//! 注意：当前所有初始化步骤直接内联在 `main.rs` 的 `app.run` 闭包内。
//! 此模块暂为占位，后续可把与 AppState 无关的纯 crate init 链提取到此。

pub mod about;
pub mod migrate;
pub mod quick_action_bar;

pub use about::open_about_window;
pub use migrate::MigrationBanner;
pub use quick_action_bar::QuickActionBar;
