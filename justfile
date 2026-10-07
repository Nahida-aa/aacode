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
# APP_ID 用 reverse-dns 同 zed（dev.zed.Zed），使各 channel 的 desktop 可并存。
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

# 本地真实安装（默认用户级 ~/.local，无需 root；可传 PREFIX 覆盖，如 `just install /usr`）。
#
# Exec/TryExec 用绝对路径：用户级安装时 ~/.local/bin 往往不在 GUI 会话 PATH 里，
# TryExec 解析失败会让启动器直接隐藏该应用。zed 的 tarball 面向解压即用故用裸名，
# 这里 install 到 PREFIX 必须绝对化 —— 与 aa-player 的做法一致。
install prefix="$HOME/.local":
    #!/usr/bin/env bash
    set -euo pipefail
    cargo build --release -p aacode
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
    -update-desktop-database "{{prefix}}/share/applications"
    -gtk-update-icon-cache -q -t -f "{{prefix}}/share/icons/hicolor"
    -kbuildsycoca6
    echo "已安装到 {{prefix}}（二进制：{{prefix}}/bin/aacode，desktop：{{_app_id}}.desktop）"

# 卸载本地安装（数据目录 ~/.local/share/zed 保留，见 paths::APP_NAME）
uninstall prefix="$HOME/.local":
    #!/usr/bin/env bash
    set -euo pipefail
    rm -f "{{prefix}}/bin/aacode" "{{prefix}}/share/applications/{{_app_id}}.desktop"
    find "{{prefix}}/share/icons/hicolor" -name "{{_app_icon}}.png" -delete 2>/dev/null || true
    -update-desktop-database "{{prefix}}/share/applications"
    -gtk-update-icon-cache -q -t -f "{{prefix}}/share/icons/hicolor"
    echo "已从 {{prefix}} 卸载"

# cargo tree -e no-dev -i -p terminal_view    # 谁依赖 terminal_view（反向）
# cargo tree -e no-dev -p terminal_view       # terminal_view 依赖谁（正向）
# cargo tree --prefix none -p terminal_view   # 简洁版
