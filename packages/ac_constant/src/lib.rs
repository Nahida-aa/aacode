//! aacode 全局常量。
//!
//! 集中管理品牌名、app 标识符、文档 URL 等跨 crate 共享的硬编码值。
//! 品牌/标识符已于 2026-10-12 改为 `AaCode` / `dev.aa.AaCode`，macOS bundle 元数据
//! （`packages/apps/aacode/Cargo.toml` 的 `[package.metadata.bundle-*]`）已同步为
//! `dev.aa.AaCode{,-Dev,-Nightly,-Preview}`；Linux 侧 `justfile` 的 `_app_id` 与
//! `script/bundle-linux` 的频道分支也应是同一套值。
//! 注意：`DOCS_URL` 仍指向 zed.dev（暂无自有文档站）；分享链接的 `zed://` scheme
//! 亦未改（见 `agent_skills`），这两处是有意保留，改前需先确认消费方。

/// 品牌显示名，如 `"AaCode Dev"` / `"AaCode"` 里的 `"AaCode"`。
pub const BRAND: &str = "AaCode";

/// 应用标识符：macOS bundle identifier、Wayland app_id、X11 WM_CLASS 共用。
/// Stable 不加后缀，Dev/Nightly/Preview 会拼 `-Dev` / `-Nightly` / `-Preview`。
pub const APP_ID: &str = "dev.aa.AaCode";

/// Windows app identifier 前缀，拼频道后缀如 `"Aa-Code-Dev"`。
#[cfg(target_os = "windows")]
pub const APP_IDENTIFIER_PREFIX: &str = "Aa-Code";

/// 官方文档 base URL。
pub const DOCS_URL: &str = "https://zed.dev/docs";
