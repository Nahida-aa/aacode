run-app:
    cargo run -p aacode

# 清理
clean:
    cargo clean

stats:
    scc . --exclude-dir node_modules,dist,build,target,venv,.venv,__pycache__,.git,vendor,out,cmake-build-debug,CMakeFiles --exclude-ext lock,json,md,yaml,yml,toml,ini,conf

# —— 本地安装 ——
# 布局与 zed 的 crates/zed/resources + script/bundle-linux 对齐：
#   packages/apps/aacode/resources/aacode.desktop.in       .desktop 模板（envsubst 填充）
#   packages/apps/aacode/resources/app-icon<suffix>.png     512x512，按 channel 取（stable 无后缀）
#   packages/apps/aacode/resources/app-icon<suffix>@2x.png  1024x1024（HiDPI）
# 全部走 zed 的注入方式，值在本 recipe 执行时由环境变量决定，不写进源码：
#   RELEASE_CHANNEL  同 zed 的 ZED_RELEASE_CHANNEL —— release_channel crate 在
#                    **构建期**读它（build.rs 检测到该 env 就置
#                    __do_not_set_zed_release_channel cfg，lib.rs 走 env! 分支）。
#                    它决定 db 目录（0-<channel>）与 cli 的 socket 名
#                    （zed-<channel>.sock）。改它必须重新构建。
#   APP_NAME/APP_ID/APP_ICON/APP_ARGS  zed 的 script/bundle-linux L180-199 同样是
#                    export 一组变量再 envsubst 渲染 .desktop.in，这里照搬。
#
# 例（各 channel 并存，desktop 文件名不同故互不覆盖）：
#   just install
#   RELEASE_CHANNEL=nightly APP_NAME="AaCode Nightly" APP_ID=dev.aa.AaCode-Nightly just install
#
# 注意 RELEASE_CHANNEL 只影响 aacode 自己；它与 paths::APP_NAME（决定数据目录
# ~/.local/share/<name>）是两件事，见 .agents/zed-modifications.md。
#
# 默认 stable：本地安装产物是正式版（图标无后缀、db 落在 0-stable）。
#
# 与「不设 env 时的默认」的关系：aacode 规定「设置环境变量，否则 dev」
# （packages/release_channel/src/lib.rs 的 compile_time_release_channel_name，
# 用 option_env! 而非 zed 的 env! + include_str! 双分支，也不再有 RELEASE_CHANNEL
# 文件）。所以：
#   just install                 → stable（本文档这条默认，env 注入）
#   裸 cargo build / cargo run   → dev   （无 env，代码内兜底）
# 这样调试时不至于和正式数据混在同一个 0-stable 库里。
_channel      := env_var_or_default("RELEASE_CHANNEL", "stable")

# 生成应用图标，512 + 1024 两个尺寸 × 4 个 channel，产物随仓库提交。依赖 resvg。
#
# 命名规则照 Zed `crates/zed/build.rs` 的 icon_path()（L221-240）：stable 无后缀，
# 其余带 -<channel>，未知 channel 一律当 -dev。
#   aacode.svg         → app-icon.png        （stable）
#   aacode-dev.svg     → app-icon-dev.png
#   aacode-nightly.svg → app-icon-nightly.png
#   aacode-preview.svg → app-icon-preview.png
# 改了任一 assets/images/*.svg 后重跑。
icons:
    #!/usr/bin/env bash
    set -euo pipefail
    # 用 cut 而不是 shell 的 ${var%%:*} —— 后者的 %{ 会被 just 的插值语法吃掉。
    for pair in "stable:aacode" "dev:aacode-dev" "nightly:aacode-nightly" "preview:aacode-preview"; do
      # stable 的源文件是 aacode.svg（无后缀），其余带 -<channel>。
      suffix=$(echo "$pair" | cut -d: -f1)
      src="assets/images/$(echo "$pair" | cut -d: -f2).svg"
      base="packages/apps/aacode/resources/app-icon"
      if [ "$suffix" != "stable" ]; then base="$base-$suffix"; fi
      resvg -w 512  -h 512  "$src" "$base.png"
      resvg -w 1024 -h 1024 "$src" "$base@2x.png"
    done
    echo "生成完成：packages/apps/aacode/resources/app-icon{,-dev,-nightly,-preview}{,@2x}.png"

# 本地真实安装：aa-bundle 生成 PKGBUILD → makepkg 构建并安装 → pacman 管理。
#
# 与旧实现的区别：此前这里是 ~60 行手写拷贝（cargo build + strip + install
# 二进制/图标 + envsubst 渲染 .desktop + 刷桌面数据库）。现在这些全在
# aa-bundle 里（packages/apps/aa-bundle，装在 ~/.cargo/bin/aa-bundle）：
# - .desktop 由 aa-bundle 按 cargo-bundle 的方式自动生成，不需模板
# - 身份/图标/依赖来自 Cargo.toml 的 [package.metadata.bundle-shared]（各 channel
#   共享）+ [package.metadata.bundle-stable]（本 channel 差异），两层分层合并
# - 卸载变成 pacman -R aacode，不再需要 script/uninstall.sh
#
# 安装前确保 aa-bundle 可用（gpui_learn 侧改过 aa-bundle 后要重跑才生效，
# 因为 cargo install 是快照式的）：
#   just -C ~/repos/ide_ls/gpui_learn install-cli
install:
    #!/usr/bin/env bash
    set -euo pipefail
    # RELEASE_CHANNEL 在 cargo build 之前注入 —— aacode 的取值规则是「env 或 dev」
    # （release_channel/src/lib.rs 用 option_env! 读构建期值），这条注入就是 stable
    # 与 dev 的唯一区别来源。对齐 zed：CI 也是在编译时注入 ZED_RELEASE_CHANNEL。
    # 注意 aa-bundle 不认 channel——想打 nightly 就把它指向 bundle-nightly 表。
    # LK_CUSTOM_WEBRTC：webrtc-sys（git rev 0a1c519）的 download_webrtc() **没有缓存
    # 判断**，开头就 remove_dir_all(prebuilt_dir()) 然后无条件下载（registry 版
    # webrtc-sys-build 0.3.19 才有 `if exists { return Ok }`）。而它的 prebuilt_dir()
    # 落在 OUT_DIR 下 —— profile 一改指纹就变 → OUT_DIR 变 → 每次全量重编都要重新
    # 下载 575 MB 的 libwebrtc.a，GitHub 那边握手失败就直接 build failed。
    # 设 LK_CUSTOM_WEBRTC 后 download_webrtc() 第一行就 return Ok，改从固定目录读。
    # 该目录在 ~/.cache 下，cargo clean 不会波及。
    #
    # 若要重新拉取：删掉该目录并去掉此 env var，下次构建会自动下载。
    #
    # RUSTFLAGS 里的 -fuse-ld=lld：产物 350MB+ 时 GNU ld 的内存开销是 lld 的数倍，
    # 与 lto="thin"（bitcode 全量载入内存）叠加会撑爆 27G 内存的机器，最后链接被
    # SIGTERM 杀掉（"Compile terminated by signal 15"）。换成 lld 即可，行为不变。
    # 注意 bundle-linux 走 .cargo/bundle-config.toml，那边也加了同一条。
    #
    # 这里不能用 --config 传 rustflags：RUSTFLAGS 环境变量会覆盖 [target.*] rustflags，
    # 而 bundle-config 的 rpath=$ORIGIN/../lib 正是靠那条生效的。
    export LK_CUSTOM_WEBRTC="$HOME/.cache/aacode/webrtc/webrtc-0001d84-4-linux-x64-release"
    export ZED_RELEASE_CHANNEL="{{_channel}}" CARGO_BUILD_WARNINGS=allow
    export RUSTFLAGS="-C link-arg=-fuse-ld=lld"

    # 输出在 workspace 根的 target/ 下（workspace 成员的 target 由根决定，
    # crate 目录下的 target/ cargo 不用），按 bin 名分目录
    out="target/aa-bundle/pkgbuild/aacode"
    rm -rf "$out"
    aa-bundle --format pkgbuild -p packages/apps/aacode \
      --bundle-meta "Cargo.toml#[package, metadata, bundle-shared]" \
      --bundle-meta "Cargo.toml#[package, metadata, bundle-stable]" \
      --out "$out"
    # strip 由 PKGBUILD 的 options=(strip)（Arch 默认）负责，与旧实现一致
    (cd "$out" && makepkg -sif --noconfirm)
    command -v kbuildsycoca6 >/dev/null && kbuildsycoca6 || true
    echo "已通过 pacman 安装 aacode（查询：pacman -Ql aacode）"

# 卸载：交给 pacman。包内文件（二进制/图标/.desktop）由包管理器记录，
# 不再需要手写 rm + 刷桌面数据库——那套 script/uninstall.sh 可以删了。
# 数据目录 ~/.local/share/... 不在包里，pacman 不会动它（与旧实现一致）。
uninstall:
    pacman -R aacode

# cargo tree -e no-dev -i -p terminal_view    # 谁依赖 terminal_view（反向）
# cargo tree -e no-dev -p terminal_view       # terminal_view 依赖谁（正向）
# cargo tree --prefix none -p terminal_view   # 简洁版
