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
set -euo pipefail

cd "$(dirname "$0")/.."

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
