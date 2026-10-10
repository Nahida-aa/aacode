#![allow(clippy::disallowed_methods, reason = "build scripts are exempt")]
use std::process::Command;

const APP_MANIFEST: &str = include_str!("../aacode/Cargo.toml");
const WORKSPACE_MANIFEST: &str = include_str!("../../../Cargo.toml");

/// 从 manifest 里取 `package.version` 的字符串值。
///
/// 兼容两种写法：
/// - `version = "1.2.3"` —— 直接是字符串（Zed 的 crates/zed 就是这样）
/// - `version.workspace = true` —— TOML 点号键，解析出的是表 `{workspace = true}`，
///   真正的值在 workspace 根的 `[workspace.package].version`
fn package_version(doc: &toml_edit::DocumentMut, workspace_doc: Option<&toml_edit::DocumentMut>) -> Option<String> {
    let item = doc.get("package")?.get("version")?;
    if let Some(s) = item.as_str() {
        return Some(s.to_owned());
    }
    // 表继承：转到 workspace 根取
    let ws = workspace_doc?;
    ws.get("workspace")?
        .get("package")?
        .get("version")?
        .as_str()
        .map(|s| s.to_owned())
}

fn main() {
    // 【有意偏离 Zed】Zed 用 toml_edit 解析 crates/zed/Cargo.toml 且要求 version 必须是
    // 字符串字面量（`as_str()` + expect）。aacode 不能照抄：packages/app/Cargo.toml 写的是
    // `version.workspace = true`（点号键 → 表继承），直接 as_str() 会 None 并 expect 失败。
    //
    // 原先这里的写法更糟：逐行字符串切割 + 遇到 "workspace" 返回 None 再 fallback 到
    // 硬编码 "0.62.0"。切割 `version.workspace = true` 得到的是 "true"（既不等于
    // "workspace" 也不像版本号），于是 ZED_PKG_VERSION 被设成字面量 "true" ——
    // remote_server 的 `version` 子命令与 VERSION 都会打印 "true"。
    let workspace_manifest = WORKSPACE_MANIFEST
        .parse::<toml_edit::DocumentMut>()
        .expect("Failed to parse workspace Cargo.toml");

    let app_manifest = APP_MANIFEST
        .parse::<toml_edit::DocumentMut>()
        .expect("Failed to parse packages/app/Cargo.toml");

    let version = package_version(&app_manifest, Some(&workspace_manifest))
        .expect("packages/app/Cargo.toml must declare a resolvable package version");
    println!("cargo:rustc-env=ZED_PKG_VERSION={version}");
    println!(
        "cargo:rustc-env=TARGET={}",
        std::env::var("TARGET").unwrap()
    );

    // Populate git sha environment variable if git is available
    println!("cargo:rerun-if-changed=../../../.git/logs/HEAD");
    if let Some(output) = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .filter(|output| output.status.success())
    {
        let git_sha = String::from_utf8_lossy(&output.stdout);
        let git_sha = git_sha.trim();

        println!("cargo:rustc-env=ZED_COMMIT_SHA={git_sha}");
    }
    if let Some(build_identifier) = option_env!("GITHUB_RUN_NUMBER") {
        println!("cargo:rustc-env=ZED_BUILD_ID={build_identifier}");
    }
}
