# aacode 相对于 Zed 的修改说明

> 记录 aacode 基线相对 Zed（主要基线 bd747337）所做的有意修改，用于指导 rev→rev 同步时区分「有意改动」vs「需同步上游改动」。

## 1. 命名空间/命名映射

aacode 在重命名层面做了以下映射（fork-sync 归一化时已考虑）：
- `zlog` → `a_log`
- `zed_actions` → `aacode_actions`
- `zed_credentials_provider` → `ad_credentials_provider`
- `zed_resource_manager` → `a_resource_manager`
- `zed_application` → `aagent_application`

## 2. 结构性重组（单文件 → 多文件拆分）

部分 Zed 单文件 crate/module 在 aacode 中做了拆分，以便维护：

- `crates/workspace/src/status_bar.rs`（~506 行） → `packages/workspace/src/status_bar/`（mod.rs、status_bar.rs、item.rs、render.rs、impls.rs、sidebar_status.rs）+ `packages/workspace/src/workspace/status_bar/`
- `crates/project/src/project.rs`（大文件） → `packages/project/src/project/*.rs`（state.rs、lsp_events.rs、git.rs、diagnostics.rs、ai.rs、dap.rs、init.rs、helpers.rs、test_support.rs、toolchains.rs、buffers.rs、collab.rs、constructors.rs、events.rs、images.rs、lifecycle.rs、lsp.rs、paths.rs、rpc.rs、worktrees.rs 等）——标记为 SPLIT，rev 同步时按需核对区间变更
- `crates/project/src/lsp_store.rs`（单文件） → `packages/project/src/lsp_store/mod.rs` + 多个子模块（code_lens.rs、document_colors.rs、document_links.rs、document_symbols.rs、folding_ranges.rs、semantic_tokens.rs、inlay_hints.rs、rust_analyzer_ext.rs、vue_language_server_ext.rs、clangd_ext.rs、json_language_server_ext.rs 等）
- `crates/conversation_view.rs` → `packages/agent_ui/src/conversation_view/`（mod.rs、thread_view.rs、thread_search_bar.rs、message_queue.rs、elicitation.rs）

## 3. 路径/布局映射

- `packages/*` 下的 fork 源码手动 port（copy-fork），不依赖 Git merge
- `crates/project/src/...` ↔ `packages/project/src/...`，mod.rs 映射到同名 .rs 的情况单独处理
- crate root：`crates/proto/src/proto.rs` ↔ `packages/proto/src/lib.rs`（通过 include! 引入生成消息文件）

## 4. fork-crate 特别说明（project）

`project` 是高复杂度 fork-crate，aacode 有较大本地改动：

- LSP 存储结构重组（lsp_store 拆分为多个模块）
- 终端/远程 shell 处理有本地调整（配合 proto/remote_server）
- 部分功能有意裁剪或调整，导致与 zed old 存在大量归一化后仍不可解释的差异（L2）
- `packages/project/src/project/*` 归类为 SPLIT（结构性拆分），不做文件级逐行比对，区间同步时按需核对
- `packages/project/src/types/*`、`impls.rs`、`path.rs`、`protocol_helpers.rs`、`tests.rs`、`fuzzy.rs`、`event.rs`、`lib.rs` 等在 zed old（bd747337）不存在且区间无变更 → 标记为 `AACODE_LOCAL`

## 5. 主线范围约束

GPUI 桌面（`packages/app` + `packages/workspace` + `packages/ui`）是唯一主线。以下为历史遗留/实验性，不保证编译：

- `packages/cli/`（Ratatui TUI）
- `packages/tui/`（OpenTUI + Solid.js）
- `packages/webui/`（Solid.js Web）
- `packages/ui-solid/`（Kobalte UI）
- `packages/shared/`、`packages/sdk-ts/`

同步验收门槛：`cargo check -p aa-app -p workspace -p aa_gpui_kit_ui` 必须通过（warnings 可接受）。

## 6. 同步原则（rev→rev）

- 每个上游 sha 一个 commit，copy-fork 手动 port
- 不再单纯依赖「路径是否曾被 port 过」判断冲突，改用 L0/L1/L2：
  - L0：逐字节等于上游 old
  - L1：归一化（可见性、已知重命名映射）后等价
  - L2：归一化后仍有不可解释差异 → 人工确认
- 遇到函数修改/删除（有意 vs 遗漏）不确定时，**先暂停并提问**
- project 等高复杂度 crate 优先用 hunk 级局部合并，不整文件替换
- 区间同步以迁移 `bd747337..new` 对上游已有文件的改动为主；上游 old 不存在且区间无变更的文件视为 AACODE_LOCAL
