#![allow(clippy::disallowed_methods, reason = "build scripts are exempt")]
use std::process::Command;

const APP_MANIFEST: &str = include_str!("../aacode/Cargo.toml");

fn main() {
    // 对齐 Zed `crates/remote_server/build.rs` L9-14：用 toml_edit 解析并要求 version
    // 必须是字符串字面量（`as_str()` + expect），解析不到直接报错。
    //
    // 这里能直接照抄 Zed，前提是 aacode 的 aacode crate 的 version 是**字面量**
    // （`version = "1.22.1"`）而非 `version.workspace = true`。这不是巧合：
    // version 参与 cargo 的 PackageId，统一继承会导致 bump 版本时全量重编，
    // 故主二进制 crate 刻意独立持有版本号，详见其 Cargo.toml 注释。
    //
    // 移植过程中这里曾写成逐行字符串切割 + 遇到 "workspace" 返回 None 再 fallback 到
    // 硬编码 "0.62.0"。切割 `version.workspace = true` 实际得到 "true"，于是
    // ZED_PKG_VERSION 一直是字面量 "true"，`remote_server version` 子命令一直在打印它。
    let zed_cargo_toml = APP_MANIFEST
        .parse::<toml_edit::DocumentMut>()
        .expect("Failed to parse packages/apps/aacode/Cargo.toml");

    let version = zed_cargo_toml
        .get("package")
        .and_then(|package| package.get("version"))
        .and_then(toml_edit::Item::as_str)
        .expect("aacode Cargo.toml must declare a package version string");
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
