# 搁置：Linux 发布构建迁到 GitHub Actions

状态：**用户决定暂缓**（2026-10-09）。原因：aacode 的 `Cargo.toml` 有 16 处
指向 `/home/aa/repos/ide_ls/gpui_learn` 的 path patch，CI 上该路径不存在；
用户打算等时机合适时统一去掉本地 path patch，再上 CI。期间继续本地构建。

## 已完成的本地打包能力（CI 可直接复用）

- `script/bundle-linux` — 已移植并可产出 `target/release/aacode-linux-x86_64.tar.gz`
- `script/get-crate-version`、`script/lib/blob-store.sh`、`script/generate-licenses`
- `script/licenses/{template.md.hbs,aacode-licenses.toml}`
- `.cargo/bundle-config.toml`（rpath / lld / musl）
- channel 图标：`packages/app/resources/app-icon{,-dev,-nightly,-preview}.png`
- justfile：`icons/desktop/install/uninstall/strip`，`_channel` 默认 `stable`

## 上 CI 时要处理的阻碍（按优先级）

1. **16 处 gpui_learn path patch**（`Cargo.toml`）
   指向 `/home/aa/repos/ide_ls/gpui_learn/packages/{aa_cloud_llm_client,
   aa_language_model_core,assets,clock,component,gpui_fuzzy,grammars,icons,
   language_core,theme,ui,ui_input,util,util_macros,zed_cloud_llm_client}`。
   计划：统一改成相对路径（如 `gpui_learn/packages/*`），本地建软链，
   CI 用 `actions/checkout` 直接 checkout 到该目录。
   注：这只解决 aacode 自己的 patch；gpui_learn 内部若也有指向 zed 的绝对路径需一并查。

2. **`script/generate-licenses` 的绝对路径**
   `ICONS_LICENSES` 默认 `/home/aa/repos/ide_ls/gpui_learn/assets/icons/LICENSES`。
   改成相对路径后此默认值也要跟着改，否则 CI 上取不到图标许可。

3. **缺 `script/linux`**（apt/dnf/pacman 等依赖安装脚本）。
   zed 的 `script/linux` 约 300 行，覆盖多发行版；CI runner 固定 ubuntu-2204 的话，
   直接在 workflow 里 apt install 即可，不必移植全量脚本。
   注意 zed 对 ubuntu 20.04 有特殊 clang-18 / libstdc++-11 处理（webrtc-sys 需 C++20）。

4. **gpui_learn 未推送的改动**
   本地 HEAD 有 rust-embed 相关提交未推送，且有未提交改动。
   CI 只能构建已推送内容 —— 上 CI 前需先整理推送并固定 SHA。

5. **无根 `LICENSE` 文件**。不阻塞：`bundle-linux` 用 `script/generate-licenses`
   生成的 `assets/licenses.md`（内含 LICENSE 文本），仓库根不需要。

6. **不需要 `script/download-wasi-sdk`**（除非以后要编 wasm 扩展）。

## zed 的 release.yml 参考要点

`.github/workflows/release.yml` 中 x86_64 Linux：

```yaml
build_server_linux:            # 命名参考，实际 task 名见文件
  runs-on: namespace-profile-32x64-ubuntu-2004   # 32 核 64GB！
  env:
    CARGO_INCREMENTAL: 0
    CC: clang-18
    CXX: clang++-18
  steps:
    - actions/checkout (clean: false)
    - namespacelabs/nscloud-cache-action (cache: rust, path: ~/.rustup)
    - ./script/linux
    - ./script/download-wasi-sdk
    - ./script/bundle-linux
    - actions/upload-artifact → target/release/zed-linux-x86_64.tar.gz
  timeout-minutes: 60
```

关键点：zed 用 32 核 / 64 GB runner，`timeout-minutes: 60`。
aacode 本机 27 GB 内存跑不动 thin-LTO（已因此把 `[profile.release] lto = false`），
迁到 CI 后可以考虑恢复 `lto = "thin"` —— 但那属于独立决策，需重新验证本机可构建性。

## 相关

- 磁盘清理：2026-10-09 `cargo clean` 释放 475.1 GiB（target 曾达 450G，磁盘 100% 满）。
  webrtc 缓存备份在 `~/.cache/aacode/webrtc/`（`cargo clean` 会连带删除 `target/` 下的）。