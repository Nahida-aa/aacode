//! aacode 全局常量。
//!
//! 集中管理品牌名、app 标识符、文档 URL 等跨 crate 共享的硬编码值。
//! 当前 bundle 元数据（app/Cargo.toml [package.metadata.bundle-*]）仍用 Zed 品牌，
//! 所以这里的值暂与 Zed 一致。以后换 aacode 品牌只改本文件一处。

/// 品牌显示名，如 `"Zed Dev"` / `"Zed"` 里的 `"Zed"`。
pub const BRAND: &str = "AaCode";

/// 应用标识符：macOS bundle identifier、Wayland app_id、X11 WM_CLASS 共用。
/// Stable 不加后缀，Dev/Nightly/Preview 会拼 `-Dev` / `-Nightly` / `-Preview`。
pub const APP_ID: &str = "dev.zed.aacode";

/// Windows app identifier 前缀，拼频道后缀如 `"Zed-Editor-Dev"`。
#[cfg(target_os = "windows")]
pub const APP_IDENTIFIER_PREFIX: &str = "Aa-Code";

/// 官方文档 base URL。
pub const DOCS_URL: &str = "https://zed.dev/docs";
