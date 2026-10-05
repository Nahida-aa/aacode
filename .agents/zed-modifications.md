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
- `packages/project/src/types/*`、`impls.rs`、`path.rs`、`protocol_helpers.rs`、`tests.rs`、`fuzzy.rs`、`event.rs`、`lib.rs` 等：源自结构性提取/拆分（aacode 基线重组时从原始大文件提取形成），在 zed old（bd747337）中无直接同名文件，且区间内未变更。同步时需结合原始大文件（如 `crates/project/src/project.rs`）按符号核对，而不是简单视为全新特性。

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

### project 特别说明（语义层面）

除了结构性拆分外，`packages/project` 相对 `crates/project` 还存在语义层面的有意改动，包括函数签名调整、主动删除部分函数、接口/trait 实现差异等。这些差异需要结合符号级比对来判断，而不是靠行文比对。

**已核对的文件（符号一致性）**：
- `packages/project/src/lsp_store/mod.rs` ↔ `crates/project/src/lsp_store.rs`：函数名集合完全一致（159 个）
- `packages/project/src/terminals.rs` ↔ `crates/project/src/terminals.rs`：函数名集合完全一致（11 个）

**rev→rev 同步要点**：
- 对 `packages/project/src/project/*`（SPLIT）文件，不能简单整文件替换。区间内若 `crates/project/src/project.rs` 有改动，**必须先用 ast-grep 提取受影响的符号（函数/方法/struct/enum/trait impl 等）清单**，再逐一核对 aacode 对应拆分文件中是否存在同名符号、签名是否一致、是否已实现。
- 符号级比对优先于全文 diff。对于 SPLIT 组尤其需要按符号清单逐项核对，而不是假设文件内容对应。
- 遇到符号在 aacode 中不存在但在上游区间改动中涉及时：**先暂停并提问**，判断是「主动删除」（有意）还是「漏实现」（需补充）。
- 遇到签名不一致时：**先暂停并提问**，判断是否为有意调整。
- 对 L2 文件（归一化后仍有差异）也要优先考虑语义变动而非行文差异。
- 具体的符号差异清单**按需生成**（处理区间变更时针对受影响范围分析），避免事先穷尽列出所有差异。

### 不要照搬 zed 的 dev-dependencies 自引用（已移除）

`crates/project/Cargo.toml` 的 `[dev-dependencies]` 里有

```toml
project = {workspace = true, features = ["test-support"]}   # 指向自己
```

**这一行在 aacode 里已被移除，且不要在同步时加回来。**

原因：rustc 遇到 crate 的 dev-dependency 指向自身时，会为 unit test 构建**两份**该crate——一份带 `cfg(test)`（给 lib 内测试访问私有 API），一份库版（给 dev-dep 用公开 API）。两份各自带一份 `inventory` section，同名 action 就被注册两次：

```
ActionRegistry::load_actions() -> insert_action()
-> panic: Action with name `context_server::Restart` already registered
```

panic 发生在 `TestAppContext::build()` 构造 `ActionRegistry` 阶段，所以 `packages/project` 里**每一个** `#[gpui::test]` 都会 panic（不是某个测试的问题）。

实测（用 `gpui::generate_list_of_all_registered_actions()` / `inventory::iter::<gpui::MacroActionBuilder>` 计数）：

| | TOTAL | DISTINCT | DUP |
|---|---|---|---|
| 保留自引用 | 236 | 235 | `context_server::Restart x2`（两个 fn 指针不同） |
| 移除自引用 | 235 | 235 | 0 |

**为什么 aacode 可以安全移除，而 zed 需要它**：
- zed 的 `crates/project/tests/integration/` 存在，integration test 需要以库的公开 API 链接一份带 test-support 的 project，因此需要自引用来统一 feature
- aacode 的 `packages/project` **没有 `tests/`、`examples/`、`benches/`** 目录（integration 测试套件在同步时按「有意裁剪」处理掉了），自引用没有任何消费方

**排查这类问题的可复用手法**（下次遇到 action/registry 重复注册直接照搬）：
1. `gpui::generate_list_of_all_registered_actions()` 数 TOTAL / DISTINCT，确认是否真有重名
2. `inventory::iter::<gpui::MacroActionBuilder>` 取每个 builder 的 `fn` 指针地址，`same_code` 判断是「同一份代码被调用两次」还是「两处独立声明」
3. `grep -c '^name = "<crate>"' Cargo.lock` 与 `cargo tree -d` 排除「同名 crate 两个来源」（`-d` 里出现 workspace member 或 gpui_learn 包才算异常；第三方 crate 多版本属正常）
4. 若确认是同一 crate 两份编译实例，检查该 crate 的 `[dev-dependencies]` 有没有指向自己

### action 的 namespace 必须是 zed，不能改名

`actions!` 宏的**首参是用户可见的 action 名**，会出现在 keybindings.json、用户
`keymap.json`、命令面板搜索与 keybindings schema 里——它是公开接口，不是内部命名。

**保持上游的值，不要改成 `app` / `aacode` / 产品名。** 上游 zed 没有 `app`、
`aacode` 这类 namespace；aacode 的做法是照搬上游（`aacode_actions` 里 9 处
`actions!(zed, ...)`、`title_bar` 的 `actions!(app_menu, ...)` 都与上游一致）。

唯一一次偏离是 `packages/app/src/app_menus.rs` 曾把 zed `crates/zed/src/zed.rs:121`
那一块 app 级 action（`Hide` / `HideOthers` / `ShowAll` / `Minimize` / `Zoom` /
`OpenDefaultSettings` / `ShowDefaultSemanticTokenRules` / `ToggleFullScreen`）搬进本文件时
顺手把首参改成了 `app`。后果（已修）：

- `assets/keymaps/default-linux.json` 的 `"zed::ToggleFullScreen"`（f11 全屏）解析失败
- `default-macos.json` 的 `zed::Hide` / `HideOthers` / `Minimize` / `ToggleFullScreen` 同样失效
- 从 zed 迁移过来的用户 keymap.json 也全部失效

代价对比：改 20 个 keymap json + 破坏配置迁移，换的只是「换个更贴合产品名的命名」，
不划算。Rust 侧引用不受影响（都是 `crate::app_menus::Minimize` 这类 crate 内类型路径）。

> 注意 `aacode_actions` 里大量 action 带
> `#[action(deprecated_aliases = ["aacode_actions::Xxx"])]` —— 那是 **Rust crate 名**
> 从 `zed_actions` 改名的兼容措施（crate 改名不涉及用户配置），与 action 名的
> namespace 是两件事，不要混淆。

