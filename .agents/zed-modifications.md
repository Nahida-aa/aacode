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

### terminal_view / terminal 的移植缺口（已补，供后续 port 参考）

`packages/repl` 接入时暴露出四处「repl 按 zed 的 API 写，但 aacode 的 terminal_view
已经改过」的缺口，全部已修：

1. **模块路径**：zed 是 `terminal_view::terminal_element::TerminalElement`（单文件模块），
   aacode 把 `terminal_element.rs` 拆进了 `terminal_view/src/element/` 并从 crate 根
   重导出 → 应写 `terminal_view::TerminalElement`
2. **`layout_grid` 可见性**：zed 是 `pub fn`，aacode 曾收窄成 `pub(super)`
   （`terminal_view/src/element/layout_grid.rs`）
3. **`impl Iterator for RenderableCells<'_>` 缺失**：上游在 `crates/terminal/src/alacritty.rs`
   有这个 impl，aacode 移植 `terminal.rs` 时漏了，已补到
   `terminal/src/alacritty/conversions.rs`（转换函数已在那里，`IndexedCell` /
   `RenderableCells` 也已在同文件的 `use crate::{...}` 中）。
   **这个不只 repl 需要**：`TerminalElement::layout_grid` 的入参是
   `impl Iterator<Item = T>`（zed 与 aacode 签名一致），任何把
   `Terminal::with_renderable_cells(|cells| ...)` 的结果传进去的调用方都会踩到；
   而报错信息（`RenderableCells is not an iterator`）指向**调用点**，极易误判成调用方写错。
4. **`async-tungstenite` 版本分裂**：`repl` 直接用 workspace 的 `0.33`，而它的依赖
   `jupyter-websocket-client 1.1.0` 声明 `async-tungstenite >= 0.29.1`（**无上界**），
   cargo 选了最新的 `0.35`，两端 `WebSocketStream` 类型不通用
   （`repl/src/kernels/remote_kernels.rs`）。上游 `Cargo.lock` 里只有一份 `0.33`。
   修法：`cargo update -p async-tungstenite@0.35.0 --precise 0.33.0`
   （锁到精确版本，勿用无 `-p` 的全量 `cargo update`）。

**接线方式**（对齐 zed crates/zed/Cargo.toml L138/L186 + main.rs L725/L781）：
- 这些 crate 的 action 要进 inventory 才会被内置 keymap 解析，因此必须进
  `aa-app` 的 `[dependencies]`（**不是** `[dev-dependencies]`，否则不链接进二进制）
- `repl::init(fs.clone(), cx)` 需要 `Arc<dyn Fs>`，排在 `set_global(fs.clone(), cx)` 之后
- `tabular_data_preview::init(cx)` / `tab_switcher::init(cx)` /
  `lsp_command_selector::init(cx)` 无参数，直接在 Panel init 段调用

### 无上界依赖约束会因 lock 重算而分裂成多版本

**症状**：`multiple different versions of crate X in the dependency graph` 之类的
编译错误，实际是同一 crate 在图里存在两份，`WebSocketStream` 之类类型不相通。

**机制**：crates.io 上有些包对依赖声明的是**无上界**约束，例如
`jupyter-websocket-client 1.1.0` 对 `async-tungstenite` 声明 `>= 0.29.1`。
cargo 对 `0.x` 版本的 caret 是限定的（`"0.33"` = `>=0.33.0, <0.34.0`），所以我们自己的
声明限死在 0.33；但 `>= 0.29.1` 没有上界，**一旦 lock 被重算，求解器就会给它挑当时
最新的版本**，于是图里同时出现 0.33 与 0.35 两份。

**什么操作会重算 lock**（都会静默改写 Cargo.lock）：
- 裸跑 `cargo metadata`（不带 `--locked`）
- 不带 `-p` 的 `cargo update`
- 新增/删除依赖、改 feature、`[patch]` 变更

**规避**：
- 平时用 `cargo check --locked` / `cargo build --locked`；确实需要重算时用
  `cargo update -p <crate> --precise <version>` 精确到单个包
- 定期跑 `script/check-lock-drift.sh`（等价于上游 zed CI 的
  `cargo update --locked --workspace`，见 .github/workflows/run_tests.yml）
  在提交前确认 lock 没有可更新的漂移
- 上游 zed 从不做会写 lock 的操作，CI 全程 `--locked`；aacode 没有 CI，
  故用该脚本替代

**已发生实例**：`async-tungstenite` 被重算为 0.33 + 0.35 两份，导致 `repl` 报
E0308（`packages/repl/src/kernels/remote_kernels.rs`）。修法
`cargo update -p async-tungstenite@0.35.0 --precise 0.33.0`，与上游 zed
Cargo.lock 一致（上游只有一份 0.33）。


## http_client：两套并存，本地 fork 当前零引用

workspace 里有两个 http_client，**别搞混**：

| Cargo 名字 | 来源 | 引用者 |
|---|---|---|
| `http_client` | zed git `rev = "afecd6d719aad92aecfa2860f49c4f2956708831"` | app / client / anthropic / dap / extension … 20+ crate，**这是实际运行的** |
| `aa_http_client` | `path = "packages/http_client"` | **零个 crate** |

**保留本地 fork 的原因**：将来可能不再用 zed 的 `http_client`——代码里存在
zed 独有字符串。当前无妨，两者并存即可。

**维护约定**：`packages/http_client/` 与 zed `afecd6d` 的 `crates/http_client/`
实质代码**零差异**，只有两处无害偏差：

1. **rustfmt**：aacode `.rustfmt.toml` 开了 `unstable_features = true` +
   `fn_single_line = true`，故短函数压成一行。对 zed 做 diff 时这是噪声，忽略。
2. **crate root 文件名**：zed 是 `[lib] path = "src/http_client.rs"`，fork 用
   `src/lib.rs`（内容一致）。

同步 fork 时改这两点即可；**不要**把 zed 的 `use` 语句改成 aacode 风格
（见 `.agents/zed-port.md` #11：依赖用 Cargo 别名，不改搬来的源码）。

目前代码里唯一的 zed 独有字符串是测试里的
`packages/http_client/src/github.rs:199` ——
`github_api_request("https://api.github.com/repos/zed-industries/zed/releases")`。

## 高危依赖：slotmap（版本必须与 zed 一致）

**现象**（编译通过、不 panic、只在启动时刷 ERROR）：

```
ERROR: Unable to deserialize editor: No entry in database for item_id:
       4294967690 and workspace_id WorkspaceId(2)
ERROR: No keybinding editor to deserialize
```

**根因**：`gpui::EntityId` 是 `slotmap::new_key_type!` 生成的 Key（idx+version 打包位域），
而 workspace 持久化把它的原始 u64 直接当主键写进 `editors` 表：

```
workspace/serialize/pane.rs:36   item_id: handle.item_id().as_u64()
        ↓ 写入
editors 表 (item_id, workspace_id) ← 主键
        ↓ 读取
editor/items.rs:1290  EditorDb::get_serialized_editor(item_id, workspace_id)
        ↓ 查不到行
"Unable to deserialize editor: No entry in database"
```

声明是 `slotmap = "1.0.6"`（caret 语义），lock 却漂到 `1.1.1`（zed 是 `1.0.7`）。
位域布局一变，旧行解码即错。识别特征：日志里出现 `4294967295`（= `0xFFFFFFFF`）——
那正是 `slotmap::KeyData::null()` 的 `idx == u32::MAX` 特征，正常数据不该有。

**修法**：`cargo update -p slotmap --precise 1.0.7`（checksum 应与 zed lock 一致）。

**已排查确认其余无同类风险**：
- `proto` 是 aacode 本地 fork，主线同进程不与 zed 通信 → 无 wire 兼容问题
- `inventory` 0.3.21→0.3.24 是 patch 版本，且 keymap 存的是 action 名字符串 → 无害
- 其余持久化类型（`SerializedEditor` 等）字段全是 `String`/`PathBuf`/`i64`，
  不含第三方紧凑编码；`debugger_ui` 的 `HashMap<EntityId, _>` 是运行时结构，不落库

**防线**：`script/check-lock-drift.sh` 第二重检查会比对高危包与 zed lock 的版本，
不一致直接 `exit 1`。用法 `ZED_LOCK=<path> bash script/check-lock-drift.sh`。

## 全量对照现状（非阻塞，仅记录）

aacode 1539 包 / zed 1595 包，**同名版本不一致 485 个**（绝大多数是小版本升级，
如 `cc 1.4.3→1.6.0`、`regex 1.12.3→1.13.1`），不影响正确性。成因同
「无上界依赖陷阱」——任何写 lock 的 cargo 操作都可能重算。

如将来要与 zed 完全对齐，用 `cargo update -p <crate> --precise <zed 版本>` 逐个降级，
不要裸跑不带 `-p` 的 `cargo update`（会重算全图，且可能让同名 crate 分裂成多份）。
