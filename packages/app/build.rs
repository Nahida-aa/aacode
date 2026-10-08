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
//!   icon_path() 相同；取值来源改成运行时：先 `ZED_RELEASE_CHANNEL` env（`just install`
//!   注入），否则回退读 `packages/app/RELEASE_CHANNEL`，都没有则 dev。
//! - zed 用 `option_env!("RELEASE_CHANNEL")` 决定图标文件名后缀；aacode 的
//!   release_channel 是运行时读 `<repo>/zed/RELEASE_CHANNEL`，build.rs 里拿不到，
//!   且只有一个图标，故省略这段。
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
    // Zed 用 `option_env!("RELEASE_CHANNEL")` 读编译期 env；这里用运行时
    // `ZED_RELEASE_CHANNEL`（`just install` 注入）或回退读 packages/app/RELEASE_CHANNEL。
    let channel = env::var("ZED_RELEASE_CHANNEL")
        .ok()
        .or_else(|| {
            std::fs::read_to_string("RELEASE_CHANNEL")
                .ok()
                .map(|s| s.trim().to_string())
        })
        .unwrap_or_else(|| "dev".to_string());
    let suffix = match channel.as_str() {
        "stable" => "",
        "preview" => "-preview",
        "nightly" => "-nightly",
        _ => "-dev",
    };

    let icon = Path::new(&format!("resources/app-icon{suffix}.png"));
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
    println!("cargo:rerun-if-changed=RELEASE_CHANNEL");
}

fn main() {
    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    prepare_app_icon_x11();
}