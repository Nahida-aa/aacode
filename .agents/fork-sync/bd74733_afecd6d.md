# Batch A: bd747337 → afecd6d719aad92aecfa2860f49c4f2956708831

## Baselines

- Zed old: `bd747337d7be138834e20972b9e203c7b239cc47` (2026-09-28)
- Zed new: `afecd6d719aad92aecfa2860f49c4f2956708831` (2026-09-30)
- gpui_learn rev (W1): `4cb19f19b285024d0d2d0359a56f9a78104bb6b8` (after bump)
- aacode baseline before W3 ports: `b9c9c65c19d48e751bfa7532d0df082af6f102cc` (W2 complete: root Zed revs + gpui_learn deps updated)

# 同步 bd74733 → afecd6d（批次 A）

|                    |                                                          |
| ------------------ | -------------------------------------------------------- |
| 状态               | 🟡 计划中                                                |
| from               | `bd747337d7be138834e20972b9e203c7b239cc47`（2026-09-28） |
| to                 | `afecd6d719aad92aecfa2860f49c4f2956708831`（2026-09-30） |
| 区间提交数         | 17                                                       |
| gpui 命中提交      | 2                                                        |
| fork 命中          | 11 个 crate                                              |
| ⚠ 需人工调和（L2） | 5                                                        |
| 负责人             | —                                                        |
| 创建               | 2026-10-03                                               |

## 1. 为什么做这一批

先跑通流程 + 建验收标准，不追求吃到多少代码

## 2. 目标 / 非目标

目标：

-

非目标：

-

## 3. 工作流

rev 同步是原子的（换 rev 会带进区间内全部改动），所以 W1/W2 必须同批次推；
W3 与它们无依赖，可按 crate 多人并行。

### W1 gpui_learn（必须先于 W2）

- [ ] W1.1 根 `Cargo.toml` 10 处 rev → `afecd6d`
- [ ] W1.2 vendored 平台层适配（`gpui-android` 等）
- [ ] W1.3 commit + push，记下 gpui_learn 新 sha

### W2 aacode rev + gpui 适配

- [ ] W2.1 根 `Cargo.toml` 29 处 zed rev + `Cargo.toml:536` 的 `ui` rev
- [ ] W2.2 gpui API 适配（`Platform` trait 新成员等）
- [ ] W2.3 `cargo check -p app -p workspace -p ui` 过
- [ ] W2.4 `cargo check --workspace` 过（下限）
- [ ] W2.5 commit + push

#### 扫描顺序（按依赖优先级，基础→上层）

建议按依赖关系从底层到上层顺序扫描，以便先处理被依赖的 crate。初稿顺序（需人工确认）：

1. `project`
2. `proto`
3. `rpc`（如命中则优先）
4. `task`
5. `terminal`
6. `http_client`
7. `client`
8. `cloud_api_client`
9. `acp_thread`
10. `open_ai`
11. `agent_ui`
12. `which_key`
13. `node_runtime`
14. `remote_server`

> 注：`--full` 全量对比时按此顺序逐 crate 扫描，避免一次性处理过多文件。也可配合 `--crate <name>` 单独处理。

### W3 fork 同步（按 crate 认领，一 crate 一 owner）

命中 11 个 crate：

- [ ] acp_thread（待认领）
- [ ] agent_ui（待认领）
- [ ] client（待认领）
- [ ] cloud_api_client（待认领）
- [ ] http_client（待认领）
- [ ] node_runtime（待认领）
- [ ] open_ai（待认领）
- [ ] project（待认领）
- [ ] proto（待认领）
- [ ] remote_server（待认领）
- [ ] which_key（待认领）

逐条 port 记录（一个上游 sha 一个 commit，便于单独 revert）：

| 上游 sha     | crate                               | aacode 改过该文件?                     | 动作                                                                             | port commit | 认领 |
| ------------ | ----------------------------------- | -------------------------------------- | -------------------------------------------------------------------------------- | ----------- | ---- |
| `afecd6d719` | `node_runtime`                      | ✓（与上游old等价基础上新增SystemNode） | node_runtime: Expose standalone system Node discovery (#64928)                   | `23f4b0d`   | —    |
| `017f9b89aa` | `agent_ui`                          | ✓（1 行，L1 已知变换内）               | agent_ui: Hide wrap guides in the agent message editor (#64886)                  | `69031de`   | —    |
| `5d5963361f` | `project`                           | ✓                                      | Diff LSP format responses that replace the whole buffer (#57269)                 | `0e73083`   | —    |
| `1dc8844439` | `which_key`                         | ✓                                      | which_key: Show task names for task::Spawn bindings (#64937)                     | `7ef1a04`   | —    |
| `c87632ef44` | `project`, `proto`, `remote_server` | ✓                                      | Read remote shell config when creating a terminal shell (#61451)                 | `5097d8a`   | —    |
| `14dd03e896` | `acp_thread`, `agent_ui`            | ✓                                      | agent_ui: Guard follow-up sends from stale send results (#64917)                 |             |      |
| `12f79c0aeb` | `client`, `cloud_api_client`        | ✓                                      | cloud_api_client: Use the platform TLS verifier for the cloud websocket (#63686) |             |      |
| `c32938c34c` | `open_ai`                           | ✓                                      | open_ai: Fix issues with optional arguments when model calls a tool (#64920)     |             |      |
| `ead2d9eac0` | `http_client`                       | ✓                                      | http_client: Ensure GitHub digest prefix is always stripped (#64905)             |             |      |

## 4. ⚠ 冲突预警（按「差异能否被已知移植变换解释」分级）

**不要用「本地有没有提交碰过这个文件」判断冲突** —— copy-fork 里port 动作本身就会让每个
被同步过的文件出现在 `git log` 里，那样判几乎全是误报。本节用的是：把本地文件与
**上游 old 版本**比，看差异能否被已知变换（workspace-ify / `src/x.rs`→`src/lib.rs` /
`zed_actions`→`aacode_actions` / `zlog`→`zlog` 等）解释。

| 级别            | 含义                                                 | 本批 |
| --------------- | ---------------------------------------------------- | ---- |
| **L0 快进**     | 本地 == 上游 old（同路径、已考虑路径映射），干净照搬 | 14   |
| **L1 已知变换** | 差异全是机械移植变换，按例应用即可                   | 2    |
| **L2 需人工**   | 有解释不了的差异                                     | 5    |

L2 只能筛出「与上游 old 不同、且不像机械变换」的文件，**判断不了意图**。
「我们故意删了这些测试」「这个偏离是设计而非疏漏」只有人知道，所以 L2 一律人工确认，
不要自动当成冲突。

| 本地路径                                                 | 上游路径                                               | 级别   | 未解释行 | 样例                                                                                                                                                   |
| -------------------------------------------------------- | ------------------------------------------------------ | ------ | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `packages/acp_thread/src/connection.rs`                  | `crates/acp_thread/src/connection.rs`                  | **L0** | 0        | —                                                                                                                                                      |
| `packages/agent_ui/src/conversation_view.rs`             | `crates/agent_ui/src/conversation_view.rs`             | **L0** | 0        | —                                                                                                                                                      |
| `packages/agent_ui/src/conversation_view/thread_view.rs` | `crates/agent_ui/src/conversation_view/thread_view.rs` | **L1** | 0        | —                                                                                                                                                      |
| `packages/agent_ui/src/message_editor.rs`                | `crates/agent_ui/src/message_editor.rs`                | **L1** | 0        | —                                                                                                                                                      |
| `packages/client/Cargo.toml`                             | `crates/client/Cargo.toml`                             | **L1** | 1        | < workspace = true;                                                                                                                                    |
| `packages/cloud_api_client/Cargo.toml`                   | `crates/cloud_api_client/Cargo.toml`                   | **L1** | 1        | < workspace = true;                                                                                                                                    |
| `packages/cloud_api_client/src/websocket/native.rs`      | `crates/cloud_api_client/src/websocket/native.rs`      | **L0** | 0        | —                                                                                                                                                      |
| `packages/http_client/src/github.rs`                     | `crates/http_client/src/github.rs`                     | **L0** | 0        | —                                                                                                                                                      |
| `packages/node_runtime/Cargo.toml`                       | `crates/node_runtime/Cargo.toml`                       | **L1** | 1        | < workspace = true;                                                                                                                                    |
| `packages/node_runtime/src/node_runtime.rs`              | `crates/node_runtime/src/node_runtime.rs`              | **L0** | 0        | —                                                                                                                                                      |
| `packages/open_ai/src/completion.rs`                     | `crates/open_ai/src/completion.rs`                     | **L0** | 0        | —                                                                                                                                                      |
| `packages/project/src/lsp_store.rs`                      | `crates/project/src/lsp_store.rs`                      | **L0** | 0        | —                                                                                                                                                      |
| `packages/project/src/terminals.rs`                      | `crates/project/src/terminals.rs`                      | **L0** | 0        | —                                                                                                                                                      |
| `packages/project/tests/integration/project_tests.rs`    | `crates/project/tests/integration/project_tests.rs`    | **L0** | 0        | —                                                                                                                                                      |
| `packages/proto/proto/task.proto`                        | `crates/proto/proto/task.proto`                        | **L0** | 0        | —                                                                                                                                                      |
| `packages/proto/proto/zed.proto`                         | `crates/proto/proto/zed.proto`                         | **L0** | 0        | —                                                                                                                                                      |
| `packages/proto/src/proto.rs`                            | `crates/proto/src/proto.rs`                            | **L0** | 0        | —                                                                                                                                                      |
| `packages/remote_server/Cargo.toml`                      | `crates/remote_server/Cargo.toml`                      | **L1** | 2        | < [[bin]];< workspace = true;                                                                                                                          |
| `packages/remote_server/src/headless_project.rs`         | `crates/remote_server/src/headless_project.rs`         | **L0** | 0        | —                                                                                                                                                      |
| `packages/remote_server/src/remote_editing_tests.rs`     | `crates/remote_server/src/remote_editing_tests.rs`     | **L2** | 550      | CompletionSource, LanguageServerLogType, ProgressToken, Project, ProjectPath,; LanguageServerLogType, ProgressToken, Project, ProjectPath,; lsp_store: |
| `packages/which_key/src/which_key.rs`                    | `crates/which_key/src/which_key.rs`                    | **L0** | 0        | —                                                                                                                                                      |

### L2 逐个确认

**remote_editing_tests.rs**（packages/remote_server/src/remote_editing_tests.rs）

- aacode 在 old（bd747337）时就与上游不同：上游版本完整，aacode 版本已做大量裁剪（差异 550 行，主要是删除测试函数）。
- 结论：这是**有意裁剪的本地改动**（aacode 不跑这些 remote editing 集成测试），不属于「冲突」而是「设计偏离」。
- 同步策略：**不要一刀替换整个文件**。遇到上游在这个文件有改动时，按需 cherry-pick/手动合并，优先保留 aacode 现有裁剪，不要把测试全集强行搬回来。

其余 L2（Cargo.toml）已归一化判定为 L1。

```sh
# 通用核对命令
git log --oneline -- <本地路径>
git -C ~/repos/ide_ls/learn_ls/zed diff bd747337d7be138834e20972b9e203c7b239cc47..afecd6d719aad92aecfa2860f49c4f2956708831 -- <上游路径>
diff <(git -C ~/repos/ide_ls/learn_ls/zed show bd747337d7be138834e20972b9e203c7b239cc47:<上游路径>) <本地路径>
```

## 5. 验收

- [ ] `cargo check -p app -p workspace -p ui`
- [ ] `cargo check --workspace`
- [ ] 每条 fork 同步都确认过「该文件 aacode 自己没改动」，有的已在 commit 里说明
- [ ] gpui_learn 侧的 Web/Android 示例仍能 build
- [ ] 回来更新 zed-port.md §16 索引与本文件状态

## 6. 踩的坑

<!-- 做的时候随手记，这里是后来人最需要的东西 -->

## 7. 遗留 / 下一批

-

## 特别说明（project/lsp_store）

- **注意**：`project` 是 fork-crate 的主干包，aacode 对其结构和部分实现有较大本地改动（模块拆分、可见性/接口调整等），不能简单套用 L0。对于涉及函数签名、逻辑改动的提交，应优先按“人工局部合并”（patch+hunk 级别）处理，而不是整文件替换。

- `crates/project/src/lsp_store.rs`（upstream）→ `packages/project/src/lsp_store/mod.rs`（aacode）。路径不同但属于同一模块（workspace-ify 拆分/重组）。
- L0 判定时如直接跨路径比对（不同文件名）容易误判。实际处理：基于上游 old 内容比对 aacode 当前对应文件内容时需考虑路径映射。对于本批次 `5d5963361f`，恢复原文件后按需手动合并应用了上游的 diff-based edits 逻辑（非整文件替换），属于“内容合并”而非单纯 L0 快进。
- 类似的路径映射（mod.rs vs 单文件）在 fork-sync 判定中应作为已知变换（L1）处理，而不是盲目判定为 L0。

## project 全量盘点（crate 内文件级全量）

基线对比：zed old = bd747337d7be138834e20972b9e203c7b239cc47 vs aacode 当前 packages/project

- 路径映射策略：精确匹配 → mod.rs→.rs 同名替换；无法映射的归入 NOMAP
- 归一化：可见性（pub(crate)/pub(super)/pub(in ...)）去除，已知重命名（zlog→zlog、zed_actions→aacode_actions、...）对齐

统计：

- L0（完全一致）：41
- L1（归一化后一致，仅机械变换）：1
- L2（归一化后仍有差异，需人工分析）：24
- NOMAP（路径映射失败）：47
- 总文件：113

### L2 清单（重点关注）

- `packages/project/src/search/mod.rs` ← `crates/project/src/search.rs`
- `packages/project/src/lsp_store/code_lens.rs` ← `crates/project/src/lsp_store/code_lens.rs`
- `packages/project/src/lsp_store/document_colors.rs` ← `crates/project/src/lsp_store/document_colors.rs`
- `packages/project/src/lsp_store/document_links.rs` ← `crates/project/src/lsp_store/document_links.rs`
- `packages/project/src/lsp_store/document_symbols.rs` ← `crates/project/src/lsp_store/document_symbols.rs`
- `packages/project/src/lsp_store/folding_ranges.rs` ← `crates/project/src/lsp_store/folding_ranges.rs`
- `packages/project/src/lsp_store/semantic_tokens.rs` ← `crates/project/src/lsp_store/semantic_tokens.rs`
- `packages/project/src/lsp_store/inlay_hints.rs` ← `crates/project/src/lsp_store/inlay_hints.rs`
- `packages/project/src/lsp_store/mod.rs` ← `crates/project/src/lsp_store.rs`
- `packages/project/src/lsp_command/mod.rs` ← `crates/project/src/lsp_command.rs`
- `packages/project/src/git_store/diff_buffer_list.rs` ← `crates/project/src/git_store/diff_buffer_list.rs`
- `packages/project/src/git_store/mod.rs` ← `crates/project/src/git_store.rs`
- `packages/project/src/debugger/dap_store.rs` ← `crates/project/src/debugger/dap_store.rs`
- `packages/project/src/debugger/breakpoint_store.rs` ← `crates/project/src/debugger/breakpoint_store.rs`
- `packages/project/src/context_server_store/mod.rs` ← `crates/project/src/context_server_store.rs`
- `packages/project/src/worktree_store.rs` ← `crates/project/src/worktree_store.rs`
- `packages/project/src/terminals.rs` ← `crates/project/src/terminals.rs`
- `packages/project/src/task_store.rs` ← `crates/project/src/task_store.rs`
- `packages/project/src/project_settings.rs` ← `crates/project/src/project_settings.rs`
- `packages/project/src/buffer_store.rs` ← `crates/project/src/buffer_store.rs`
- `packages/project/src/project/mod.rs` ← `crates/project/src/project.rs`
- `packages/project/src/agent_server_store.rs` ← `crates/project/src/agent_server_store.rs`
- `packages/project/src/project_search.rs` ← `crates/project/src/project_search.rs`
- `packages/project/src/trusted_worktrees.rs` ← `crates/project/src/trusted_worktrees.rs`

### 路径映射失败（模块拆分导致，需单独评估）

- `packages/project/src/project/state.rs` ← `crates/project/src/project/state.rs`
- `packages/project/src/project/lsp_events.rs` ← `crates/project/src/project/lsp_events.rs`
- `packages/project/src/project/git.rs` ← `crates/project/src/project/git.rs`
- `packages/project/src/project/diagnostics.rs` ← `crates/project/src/project/diagnostics.rs`
- `packages/project/src/project/ai.rs` ← `crates/project/src/project/ai.rs`
- `packages/project/src/project/dap.rs` ← `crates/project/src/project/dap.rs`
- `packages/project/src/project/init.rs` ← `crates/project/src/project/init.rs`
- `packages/project/src/project/helpers.rs` ← `crates/project/src/project/helpers.rs`
- `packages/project/src/project/test_support.rs` ← `crates/project/src/project/test_support.rs`
- `packages/project/src/project/toolchains.rs` ← `crates/project/src/project/toolchains.rs`
- `packages/project/src/project/buffers.rs` ← `crates/project/src/project/buffers.rs`
- `packages/project/src/project/collab.rs` ← `crates/project/src/project/collab.rs`
- `packages/project/src/project/constructors.rs` ← `crates/project/src/project/constructors.rs`
- `packages/project/src/project/events.rs` ← `crates/project/src/project/events.rs`
- `packages/project/src/project/images.rs` ← `crates/project/src/project/images.rs`
- `packages/project/src/project/lifecycle.rs` ← `crates/project/src/project/lifecycle.rs`
- `packages/project/src/project/lsp.rs` ← `crates/project/src/project/lsp.rs`
- `packages/project/src/project/paths.rs` ← `crates/project/src/project/paths.rs`
- `packages/project/src/project/rpc.rs` ← `crates/project/src/project/rpc.rs`
- `packages/project/src/project/worktrees.rs` ← `crates/project/src/project/worktrees.rs`
- `packages/project/src/project/group_key.rs` ← `crates/project/src/project/group_key.rs`
- `packages/project/src/project/env.rs` ← `crates/project/src/project/env.rs`
- `packages/project/src/project/remote.rs` ← `crates/project/src/project/remote.rs`
- `packages/project/src/project/search.rs` ← `crates/project/src/project/search.rs`
- `packages/project/src/project/lsp_rpc.rs` ← `crates/project/src/project/lsp_rpc.rs`
- `packages/project/src/types/mod.rs` ← `crates/project/src/types/mod.rs`
- `packages/project/src/types/code_action.rs` ← `crates/project/src/types/code_action.rs`
- `packages/project/src/types/hover.rs` ← `crates/project/src/types/hover.rs`
- `packages/project/src/types/symbol.rs` ← `crates/project/src/types/symbol.rs`
- `packages/project/src/types/diagnostics.rs` ← `crates/project/src/types/diagnostics.rs`
- ... 其余 17 个

## project 全量盘点（crate 内文件级全量）- v2（修正路径映射）

基线对比：zed old = bd747337d7be138834e20972b9e203c7b239cc47 vs aacode 当前 packages/project
路径映射调整：`packages/project/src/project/*`（含 mod.rs）统一归类为 **SPLIT（结构性拆分，自 crates/project/src/project.rs）**，不与 project.rs 全文比对，避免将结构性重组误判为 L2。

统计：

- L0（完全一致）：41
- L1（归一化后一致，仅机械变换）：1
- L2（归一化后仍有差异，需人工分析）：23
- NOMAP（路径映射失败，未能映射到上游对应文件）：22
- SPLIT（结构性拆分，待区间变更核对）：26
- 总文件：113

### L2 清单（重点关注）

- `packages/project/src/search/mod.rs` ← `crates/project/src/search.rs`
- `packages/project/src/lsp_store/code_lens.rs` ← `crates/project/src/lsp_store/code_lens.rs`
- `packages/project/src/lsp_store/document_colors.rs` ← `crates/project/src/lsp_store/document_colors.rs`
- `packages/project/src/lsp_store/document_links.rs` ← `crates/project/src/lsp_store/document_links.rs`
- `packages/project/src/lsp_store/document_symbols.rs` ← `crates/project/src/lsp_store/document_symbols.rs`
- `packages/project/src/lsp_store/folding_ranges.rs` ← `crates/project/src/lsp_store/folding_ranges.rs`
- `packages/project/src/lsp_store/semantic_tokens.rs` ← `crates/project/src/lsp_store/semantic_tokens.rs`
- `packages/project/src/lsp_store/inlay_hints.rs` ← `crates/project/src/lsp_store/inlay_hints.rs`
- `packages/project/src/lsp_store/mod.rs` ← `crates/project/src/lsp_store.rs`
- `packages/project/src/lsp_command/mod.rs` ← `crates/project/src/lsp_command.rs`
- `packages/project/src/git_store/diff_buffer_list.rs` ← `crates/project/src/git_store/diff_buffer_list.rs`
- `packages/project/src/git_store/mod.rs` ← `crates/project/src/git_store.rs`
- `packages/project/src/debugger/dap_store.rs` ← `crates/project/src/debugger/dap_store.rs`
- `packages/project/src/debugger/breakpoint_store.rs` ← `crates/project/src/debugger/breakpoint_store.rs`
- `packages/project/src/context_server_store/mod.rs` ← `crates/project/src/context_server_store.rs`
- `packages/project/src/worktree_store.rs` ← `crates/project/src/worktree_store.rs`
- `packages/project/src/terminals.rs` ← `crates/project/src/terminals.rs`
- `packages/project/src/task_store.rs` ← `crates/project/src/task_store.rs`
- `packages/project/src/project_settings.rs` ← `crates/project/src/project_settings.rs`
- `packages/project/src/buffer_store.rs` ← `crates/project/src/buffer_store.rs`
- `packages/project/src/agent_server_store.rs` ← `crates/project/src/agent_server_store.rs`
- `packages/project/src/project_search.rs` ← `crates/project/src/project_search.rs`
- `packages/project/src/trusted_worktrees.rs` ← `crates/project/src/trusted_worktrees.rs`

### NOMAP 清单（路径映射失败）

- `packages/project/src/types/mod.rs` ← `crates/project/src/types/mod.rs`
- `packages/project/src/types/code_action.rs` ← `crates/project/src/types/code_action.rs`
- `packages/project/src/types/hover.rs` ← `crates/project/src/types/hover.rs`
- `packages/project/src/types/symbol.rs` ← `crates/project/src/types/symbol.rs`
- `packages/project/src/types/diagnostics.rs` ← `crates/project/src/types/diagnostics.rs`
- `packages/project/src/types/document_color.rs` ← `crates/project/src/types/document_color.rs`
- `packages/project/src/types/location.rs` ← `crates/project/src/types/location.rs`
- `packages/project/src/types/prepare_rename.rs` ← `crates/project/src/types/prepare_rename.rs`
- `packages/project/src/types/completion.rs` ← `crates/project/src/types/completion.rs`
- `packages/project/src/types/inlay_hint.rs` ← `crates/project/src/types/inlay_hint.rs`
- `packages/project/src/settings.rs` ← `crates/project/src/settings.rs`
- `packages/project/src/toast.rs` ← `crates/project/src/toast.rs`
- `packages/project/src/item.rs` ← `crates/project/src/item.rs`
- `packages/project/src/constants.rs` ← `crates/project/src/constants.rs`
- `packages/project/src/directory.rs` ← `crates/project/src/directory.rs`
- `packages/project/src/impls.rs` ← `crates/project/src/impls.rs`
- `packages/project/src/path.rs` ← `crates/project/src/path.rs`
- `packages/project/src/protocol_helpers.rs` ← `crates/project/src/protocol_helpers.rs`
- `packages/project/src/tests.rs` ← `crates/project/src/tests.rs`
- `packages/project/src/fuzzy.rs` ← `crates/project/src/fuzzy.rs`
- `packages/project/src/event.rs` ← `crates/project/src/event.rs`
- `packages/project/src/lib.rs` ← `crates/project/src/lib.rs`

### SPLIT 清单（结构性拆分，自 project.rs）

- `packages/project/src/project/state.rs`
- `packages/project/src/project/lsp_events.rs`
- `packages/project/src/project/git.rs`
- `packages/project/src/project/diagnostics.rs`
- `packages/project/src/project/ai.rs`
- `packages/project/src/project/dap.rs`
- `packages/project/src/project/init.rs`
- `packages/project/src/project/helpers.rs`
- `packages/project/src/project/test_support.rs`
- `packages/project/src/project/toolchains.rs`
- `packages/project/src/project/buffers.rs`
- `packages/project/src/project/collab.rs`
- `packages/project/src/project/constructors.rs`
- `packages/project/src/project/events.rs`
- `packages/project/src/project/images.rs`
- `packages/project/src/project/lifecycle.rs`
- `packages/project/src/project/lsp.rs`
- `packages/project/src/project/paths.rs`
- `packages/project/src/project/rpc.rs`
- `packages/project/src/project/worktrees.rs`
- ... 其余 6 个

## project 盘点结论（v2）

- NOMAP（22 个）：全部在 zed old（bd747337）不存在，且区间 bd747337..afecd6d 内无变更 → **标记为 AACODE_LOCAL**，不参与本批次同步。
- SPLIT（26 个，packages/project/src/project/* 含 mod.rs）：结构性拆分自 `crates/project/src/project.rs`，不做文件级比对。**待区间内 project.rs 有变更时按需核对**。
- L2（23 个）：区间内有上游改动的仅 `packages/project/src/lsp_store/mod.rs`（5d5963361f）和 `packages/project/src/terminals.rs`（c87632ef44）——**已在本批次处理**。其余 21 个 L2 文件在区间内上游无变更，差异为 aacode 本地改动 → **本批次不做额外 port**。

结论：project crate 在本批次 bd747337→afecd6d 范围内的需处理改动，已基本覆盖（lsp_store、terminals）。其余文件差异属 aacode 有意设计偏离。
