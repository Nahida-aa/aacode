run-app:
    cargo run -p aacode

# 清理
clean:
    cargo clean

stats:
    scc . --exclude-dir node_modules,dist,build,target,venv,.venv,__pycache__,.git,vendor,out,cmake-build-debug,CMakeFiles --exclude-ext lock,json,md,yaml,yml,toml,ini,conf

# —— 本地安装 ——
# 布局与 zed 的 crates/zed/resources + script/bundle-linux 对齐：
#   packages/app/resources/aacode.desktop.in   .desktop 模板（envsubst 填充）
#   packages/app/resources/app-icon.png        512x512
#   packages/app/resources/app-icon@2x.png     1024x1024（HiDPI）
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
#   RELEASE_CHANNEL=nightly APP_NAME="AACode Nightly" APP_ID=dev.aacode.AACode-Nightly just install
#
# 注意 RELEASE_CHANNEL 只影响 aacode 自己；它与 paths::APP_NAME（决定数据目录
# ~/.local/share/<name>）是两件事，见 .agents/zed-modifications.md。
_channel      := env_var_or_default("RELEASE_CHANNEL", "dev")
_app_name    := env_var_or_default("APP_NAME", "AACode")
_app_id      := env_var_or_default("APP_ID", "dev.aacode.AACode")
_app_icon    := env_var_or_default("APP_ICON", "aacode")
_app_args    := env_var_or_default("APP_ARGS", "%F")

# 生成应用图标（512 + 1024，产物随仓库提交）。依赖 resvg。
# 改了 assets/images/aacode.svg 后重跑。
icons:
    #!/usr/bin/env bash
    set -euo pipefail
    resvg -w 512  -h 512  assets/images/aacode.svg packages/app/resources/app-icon.png
    resvg -w 1024 -h 1024 assets/images/aacode.svg packages/app/resources/app-icon@2x.png
    echo "生成完成：packages/app/resources/app-icon{,\\@2x}.png"

# 渲染 .desktop 模板到标准输出（调试用，不安装）。
# channel 变体：APP_NAME=AACode\ Nightly APP_ID=dev.aacode.AACode-Nightly just desktop
desktop:
    #!/usr/bin/env bash
    set -euo pipefail
    export DO_STARTUP_NOTIFY="true" APP_CLI="aacode" APP_ICON="{{_app_icon}}" APP_ARGS="{{_app_args}}"
    export APP_NAME="{{_app_name}}" APP_ID="{{_app_id}}"
    envsubst < packages/app/resources/aacode.desktop.in

# 把调试符号拆到 .dbg，再 strip 掉本体里的 debug info 与局部符号。
# 对齐 Zed `script/bundle-linux` L119-129（含其注释里的 objcopy 选型理由）。
#
# 实测效果：521 MB → 366 MB（省 155 MB）。省的是**符号表**不是 debuginfo ——
# aacode 没有 [profile.release] 段，用 cargo 默认 debug = false，二进制里本来
# 就没有 .debug_* section；真正的体积在 .symtab + .strtab（约 303 MB），
# `--discard-all` 把局部符号也丢掉了，符号表于是几乎归零。
# 注意 llvm-size -A 不显示 .symtab/.strtab，只看它会误判"没有可 strip 的东西"。
#
# .dbg 留在 target/release/ 不安装 —— 与 zed 一致（它的 .dbg 也不进 tarball）。
strip:
    #!/usr/bin/env bash
    set -euo pipefail
    bin="target/release/aacode"
    # 用 llvm-objcopy 而非 objcopy：老发行版（如 Ubuntu 20.04）的 GNU objcopy
    # 不认新版 LLVM 产生的 CREL section。此段理由抄自 zed bundle-linux L123-124。
    llvm-objcopy --only-keep-debug "$bin" "$bin.dbg"
    llvm-objcopy --strip-debug --discard-all "$bin"
    ls -la "$bin" "$bin.dbg"

# 本地真实安装（默认用户级 ~/.local，无需 root；可传 PREFIX 覆盖，如 `just install /usr`）。
#
# Exec/TryExec 用绝对路径：用户级安装时 ~/.local/bin 往往不在 GUI 会话 PATH 里，
# TryExec 解析失败会让启动器直接隐藏该应用。zed 的 tarball 面向解压即用故用裸名，
# 这里 install 到 PREFIX 必须绝对化 —— 与 aa-player 的做法一致。
install prefix="$HOME/.local":
    #!/usr/bin/env bash
    set -euo pipefail
    # RELEASE_CHANNEL 在这里注入 —— 必须在 cargo build 之前，release_channel 的
    # build.rs 是构建期读 env 的（对齐 zed：CI 在编译时注入 ZED_RELEASE_CHANNEL）。
    ZED_RELEASE_CHANNEL="{{_channel}}" cargo build --release -p aacode
    # 与 zed bundle-linux 同样的顺序：先 strip 产物，再把瘦身后的本体装进 PREFIX。
    just strip
    install -Dm755 target/release/aacode "{{prefix}}/bin/aacode"

    # 512 + 1024 两个尺寸，对齐 zed bundle-linux L174-177。
    install -Dm644 packages/app/resources/app-icon.png       "{{prefix}}/share/icons/hicolor/512x512/apps/{{_app_icon}}.png"
    install -Dm644 packages/app/resources/app-icon@2x.png       "{{prefix}}/share/icons/hicolor/1024x1024/apps/{{_app_icon}}.png"

    # .desktop 由模板 envsubst 生成，再把 APP_CLI 绝对化。
    export DO_STARTUP_NOTIFY="true" APP_ICON="{{_app_icon}}" APP_ARGS="{{_app_args}}"
    export APP_NAME="{{_app_name}}" APP_ID="{{_app_id}}"
    export APP_CLI="{{prefix}}/bin/aacode"
    mkdir -p "{{prefix}}/share/applications"
    envsubst < packages/app/resources/aacode.desktop.in \
      > "{{prefix}}/share/applications/{{_app_id}}.desktop"
    chmod +x "{{prefix}}/share/applications/{{_app_id}}.desktop"

    # 这三个是"有则刷新、无则跳过"——缺了不影响安装，只是桌面数据库/图标缓存不更新。
    # 注意不能用 just 的 `-` 行前缀来容错：那只对非 shebang recipe 有效，而本 recipe
    # 是 `#!/usr/bin/env bash` 块，整块按一个脚本执行，`-foo` 会被 shell 当成命令名
    # （"未找到命令"）。故显式 `|| true`。
    command -v update-desktop-database >/dev/null \
      && update-desktop-database "{{prefix}}/share/applications" || true
    command -v gtk-update-icon-cache >/dev/null \
      && gtk-update-icon-cache -q -t -f "{{prefix}}/share/icons/hicolor" || true
    command -v kbuildsycoca6 >/dev/null && kbuildsycoca6 || true
    echo "已安装到 {{prefix}}（二进制：{{prefix}}/bin/aacode，desktop：{{_app_id}}.desktop）"

# 卸载本地安装（数据目录 ~/.local/share/zed 保留，见 paths::APP_NAME）
uninstall prefix="$HOME/.local":
    #!/usr/bin/env bash
    set -euo pipefail
    rm -f "{{prefix}}/bin/aacode" "{{prefix}}/share/applications/{{_app_id}}.desktop"
    find "{{prefix}}/share/icons/hicolor" -name "{{_app_icon}}.png" -delete 2>/dev/null || true
    command -v update-desktop-database >/dev/null \
      && update-desktop-database "{{prefix}}/share/applications" || true
    command -v gtk-update-icon-cache >/dev/null \
      && gtk-update-icon-cache -q -t -f "{{prefix}}/share/icons/hicolor" || true
    echo "已从 {{prefix}} 卸载"

# cargo tree -e no-dev -i -p terminal_view    # 谁依赖 terminal_view（反向）
# cargo tree -e no-dev -p terminal_view       # terminal_view 依赖谁（正向）
# cargo tree --prefix none -p terminal_view   # 简洁版
