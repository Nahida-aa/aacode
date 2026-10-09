#![allow(clippy::disallowed_methods, reason = "build scripts are exempt")]
//! 对齐 Zed `crates/zed/build.rs` 的 `prepare_app_icon_x11`（L242-269）与
//! `icon_path`（L221-240）。
//!
//! Zed 在 Linux/FreeBSD 下把 `resources/app-icon{-dev}.png` 缩放到 256x256 写进
//! `OUT_DIR/app_icon.png`，再由 `crates/zed/src/zed.rs` 的 `APP_ICON` 静态
//! （L378-388）以 `include_bytes!` 读入并设为窗口图标（X11 的 _NET_WM_ICON）。
//! 没有这一步，Linux 下窗口没有应用图标，任务栏/Alt-Tab 显示 gpui 的兜底图标。
//!
//! 与 Zed 的差异：
//! - 图标按 channel 区分（app-icon{,-dev,-nightly,-preview}.png），规则与 Zed
//!   icon_path() 相同；取值与 release_channel/src/lib.rs 同源 —— 构建期
//!   `ZED_RELEASE_CHANNEL`（`just install` 注入），没有则 dev。两者若不同源会出现
//!   「图标是 dev 的、实际行为却是 stable」的不一致。
//! - zed 还多一个 `RELEASE_CHANNEL` 文件兜底（它的 channel 取值是
//!   `include_str!` 读仓库文件，crane/Nix 下该路径不可用才有 `env!` 分支）；
//!   aacode 统一为「env 或 dev」，没有那个文件，见 lib.rs 同名函数。
//! - zed build.rs 里还有 pkg-config 收集 libva/egl rpath（webrtc-sys dlopen 用）与
//!   macOS 的 framework / deployment target 设置。前者与 aacode 无关（aacode 不链
//!   webrtc-sys），后者是 macOS 专属且 aacode 的 macOS 路径尚未验证，均未搬。

#[cfg(any(target_os = "linux", target_os = "freebsd"))]
fn prepare_app_icon_x11() {
    use image::{ImageReader, imageops};
    use std::env;
    use std::path::Path;

    let out_dir = env::var("OUT_DIR").unwrap();

    // cargo 把 build script 的 cwd 设为包根（packages/app/），图标与
    // aacode.desktop.in 同级。
    //
    // 按 channel 选图标，后缀规则与 Zed `crates/zed/build.rs` 的 icon_path()
    // （L221-240）一致：stable 无后缀，preview/nightly 带对应后缀，未知一律 -dev。
    //
    // 取值与 release_channel/src/lib.rs 的 compile_time_release_channel_name() 保持
    // 一致：构建期 ZED_RELEASE_CHANNEL → 用它；没有则 dev。这里用 option_env! 而非
    // env::var —— build script 本身就是构建期运行的，env::var 只能读到传给 build script
    // 的那份，两者恰好同源，但 option_env! 更直白地表明「这是编译期决策」。
    // 注意 aacode 已不再有 RELEASE_CHANNEL 文件（改由 lib.rs 里的 "dev" 兜底），
    // 图标与实际频道必须同源，否则会出现「显示 dev 图标却是 stable 行为」的不一致。
    let channel = option_env!("ZED_RELEASE_CHANNEL")
        .unwrap_or("dev")
        .trim()
        .to_string();
    let suffix = match channel.as_str() {
        "stable" => "",
        "preview" => "-preview",
        "nightly" => "-nightly",
        _ => "-dev",
    };

    // 先把路径存成 String 再借给 Path —— 直接写 Path::new(&format!(...)) 会让
    // 临时值在语句结束就析构，而 icon 还要用到下一句，borrow checker 报 E0716。
    let icon_path = format!("resources/app-icon{suffix}.png");
    let icon = Path::new(&icon_path);
    assert!(
        icon.exists(),
        "missing {} —— run `just icons` to generate it from assets/images/aacode{}.svg",
        icon.display(),
        suffix
    );

    let resized_image = ImageReader::open(icon)
        .unwrap()
        .decode()
        .unwrap()
        .resize(256, 256, imageops::FilterType::Lanczos3);

    // name should match the include_bytes! call in src/main.rs
    let icon_out_path = Path::new(&out_dir).join("app_icon.png");
    resized_image.save(&icon_out_path).expect("saving app icon");

    println!("cargo:rerun-if-changed={}", icon.to_string_lossy());
    // channel 变了要重新选图标，故监听这两个来源。
    println!("cargo:rerun-if-env-changed=ZED_RELEASE_CHANNEL");
    // 原先这里还监听 RELEASE_CHANNEL 文件（随该文件一起删除），channel 已改为
    // 「env 或 dev」，故只需监听 env。
}

/// 对齐 Zed `crates/zed/build.rs` 的 commit sha / build id 注入（L50-82）。
///
/// 目的：让 `src/main.rs` 的 `AppVersion::load` 能把 `stable.<build_id>.<sha>`
/// 拼进 semver 的 build metadata，版本号自带出处（对齐 zed L305-308）。
/// 优先读 `ZED_COMMIT_SHA` env（Nix 等确定性构建环境会预注入），否则 `git rev-parse HEAD`。
fn emit_commit_sha() {
    use std::process::Command;

    let git_sha = match std::env::var("ZED_COMMIT_SHA").ok() {
        Some(sha) => Some(sha),
        None => Command::new("git")
            .args(["rev-parse", "HEAD"])
            .output()
            .ok()
            .filter(|output| output.status.success())
            .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string()),
    };

    let Some(git_sha) = git_sha else {
        return;
    };

    println!("cargo:rustc-env=ZED_COMMIT_SHA={git_sha}");

    // Zed 只在 CI（有 GITHUB_RUN_NUMBER）时才带 build id，本地构建不注入。
    if let Some(build_identifier) = option_env!("GITHUB_RUN_NUMBER") {
        println!("cargo:rustc-env=ZED_BUILD_ID={build_identifier}");
    }

    // 这行是 zed 的原注释：release profile 下靠这条 warning 把信息打出来，
    // 因为没有更好的方式让 build script 的输出不被 cargo 的常规输出淹没。
    if std::env::var("PROFILE").is_ok_and(|profile| profile == "release") {
        println!("cargo::warning=Info: using '{git_sha}' hash for ZED_COMMIT_SHA env var");
    }
}

fn main() {
    emit_commit_sha();
    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    prepare_app_icon_x11();
}