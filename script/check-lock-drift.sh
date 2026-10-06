#!/usr/bin/env bash
# 检测 Cargo.lock 相对「当前声明能解析出的最新版本」是否漂移。
#
# 背景：某些第三方包（如 jupyter-websocket-client 1.1.0）对依赖声明为
# `>= 0.29.1` 这种**无上界**约束。任何会写 lock 的 cargo 操作（裸跑
# `cargo metadata`、不带 -p 的 `cargo update`、新增依赖、patch 变更…）
# 都会触发 lock 重算，求解器可能给它挑到最新版本，从而在图里产生第二份同名
# crate，导致类型不相通而编译失败。
#
# 实例：`async-tungstenite` 被重算成 0.33 与 0.35 两份后，repl 报
#   E0308: mismatched types ... multiple different versions of crate
#         `async-tungstenite`
#
# 用法：
#   script/check-lock-drift.sh          # 只检测，不修改
#   script/check-lock-drift.sh --fix    # 检测到漂移时用 --precise 逐个降级（需人工确认）
#
# 与上游 zed 的 CI 一致（.github/workflows/run_tests.yml）：
#   cargo update --locked --workspace
# `--locked` 保证只检测不落盘；`update` 保证按最新版本求解而不是沿用现有解。
#
# ─────────────────────────────────────────────────────────────────────────
# 第二重检查：高危包与 zed lock 的版本一致性
#
# 上一重只查「lock 是否落后于最新」，查不出「比 zed 新」。而某些 crate 的
# **版本会改变落库数据的编码格式**，此时必须与 zed 严格一致，否则读旧库即坏。
#
# 实例（已发生）：`slotmap` 声明 `= "1.0.6"`，caret 语义允许 1.1.1。gpui 的
# `EntityId` 就是 `slotmap::new_key_type!` 生成的 Key（idx+version 打包位域），
# 而 workspace 持久化把 `handle.item_id().as_u64()` 原样写进 `editors` 表主键
# （见 packages/workspace/src/workspace/serialize/pane.rs:36）。slotmap 1.0.x
# → 1.1.x 一改布局，旧行解码成 0xFFFFFFxx（`KeyData::null()` 的 idx==u32::MAX
# 特征），表现为启动时刷屏：
#   ERROR: Unable to deserialize editor: No entry in database for
#          item_id: 4294967690 and workspace_id WorkspaceId(2)
#
# 该类 crate 不会导致编译失败，也不会 panic，只在读旧数据时静默出错 —— 靠
# review 抓不住，必须机器比对。
set -euo pipefail

cd "$(dirname "$0")/.."

# ── 高危清单：版本变化会影响落库数据格式 / 跨进程 ABI / wire 协议 ──
# 每行：crate 名称  一句话说明它坏了会怎样
HIGH_RISK_CRATES=(
  "slotmap:EntityId 位域编码，直接决定 editors 表 item_id 能否解回"
)

check_high_risk() {
  local zed_lock="${ZED_LOCK:-}"
  if [[ -z "$zed_lock" || ! -f "$zed_lock" ]]; then
    echo "跳过：未找到 zed 的 Cargo.lock（用 ZED_LOCK=<path> 指定）"
    return 0
  fi

  echo
  echo "==> 检查高危包与 zed lock 的一致性（$zed_lock）"

  local failed=0
  for entry in "${HIGH_RISK_CRATES[@]}"; do
    local crate="${entry%%:*}"
    local why="${entry#*:}"

    local mine theirs
    mine=$(awk -v c="$crate" '
      $0 == "name = \"" c "\"" { found=1; next }
      found && /^version = / { gsub(/"/,"",$3); print $3; exit }
    ' Cargo.lock)
    theirs=$(awk -v c="$crate" '
      $0 == "name = \"" c "\"" { found=1; next }
      found && /^version = / { gsub(/"/,"",$3); print $3; exit }
    ' "$zed_lock")

    if [[ -z "$mine" ]]; then
      echo "  ??  $crate：aacode lock 中不存在（若该功能未启用可忽略）"
    elif [[ "$mine" == "$theirs" ]]; then
      echo "  OK  $crate = $mine"
    else
      echo "  !!  $crate 版本与 zed 不一致：aacode=$mine zed=${theirs:-<无>}"
      echo "      影响：$why"
      echo "      修法：cargo update -p $crate --precise ${theirs}"
      failed=1
    fi
  done

  if [[ $failed -ne 0 ]]; then
    echo
    echo "错误：高危包版本漂移。必须先修再提交 —— 这类问题编译期不报错，"
    echo "      只在读旧数据时静默失效。"
    return 1
  fi
}

echo "==> 检查 Cargo.lock 漂移（cargo update --locked --workspace）"
if out=$(cargo update --locked --workspace 2>&1); then
  if grep -q "Locking 0 packages\|no changes" <<<"$out"; then
    echo "OK：Cargo.lock 与当前声明解析出的最新版本一致，无漂移。"
  else
    echo "警告：Cargo.lock 可以被更新（存在漂移）。详情："
    echo "$out"
    echo
    echo "若确认需要更新，请手动执行并一并提交 Cargo.lock："
    echo "    cargo update --workspace && git add Cargo.lock"
    echo "若因无上界约束导致同名 crate 分裂成多版本，请用 --precise 逐个固定："
    echo "    cargo update -p <crate>@<bad-version> --precise <good-version>"
  fi
else
  echo "错误：lock 与声明不一致（通常意味着 Cargo.toml 改过但 lock 没同步）。"
  echo "$out"
  exit 1
fi

# ── 第二重：高危包与 zed 一致性（在 lock 完整性检查之后）──
check_high_risk
