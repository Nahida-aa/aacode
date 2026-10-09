# aacode 相对于 Zed 的修改说明

> 记录 aacode 基线相对 Zed（主要基线 bd747337）所做的有意修改，用于指导 rev→rev 同步时区分「有意改动」vs「需同步上游改动」。

## 1. 命名空间/命名映射

aacode 在重命名层面做了以下映射（fork-sync 归一化时已考虑）：

- `zed_actions` → `aacode_actions`
- `zed_credentials_provider` → `ad_credentials_provider`
- `zed_resource_manager` → `a_resource_manager`
- `zed_application` → `aagent_application`
- ~~`zlog` → `a_log`~~ / ~~`ztracing` → `a_tracing`~~ — 曾经的本地 `a_` 前缀，迁移到 gpui_learn 后**已还原**：
  aacode 源码全局替换 `a_log::` → `zlog::`、`a_tracing::` → `ztracing::`，
  现在与 zed 上游同名（zed 本来就叫 `zlog` / `ztracing`，不是 `log` / `tracing`）

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

同步验收门槛：`cargo check -p aacode -p workspace -p aa_gpui_kit_ui` 必须通过（warnings 可接受）。

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

|            | TOTAL | DISTINCT | DUP                                              |
| ---------- | ----- | -------- | ------------------------------------------------ |
| 保留自引用 | 236   | 235      | `context_server::Restart x2`（两个 fn 指针不同） |
| 移除自引用 | 235   | 235      | 0                                                |

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
  `aacode` 的 `[dependencies]`（**不是** `[dev-dependencies]`，否则不链接进二进制）
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

| Cargo 名字       | 来源                                                       | 引用者                                                                     |
| ---------------- | ---------------------------------------------------------- | -------------------------------------------------------------------------- |
| `http_client`    | zed git `rev = "afecd6d719aad92aecfa2860f49c4f2956708831"` | app / client / anthropic / dap / extension … 20+ crate，**这是实际运行的** |
| `aa_http_client` | `path = "packages/http_client"`                            | **零个 crate**                                                             |

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

## 全量对照现状（非阻塞，仅记录）

aacode 1539 包 / zed 1595 包，**同名版本不一致 485 个**（绝大多数是小版本升级，
如 `cc 1.4.3→1.6.0`、`regex 1.12.3→1.13.1`），不影响正确性。成因同
「无上界依赖陷阱」——任何写 lock 的 cargo 操作都可能重算。

如将来要与 zed 完全对齐，用 `cargo update -p <crate> --precise <zed 版本>` 逐个降级，
不要裸跑不带 `-p` 的 `cargo update`（会重算全图，且可能让同名 crate 分裂成多份）。

## LSP 面板：拆分「停止」与「移除」（aacode 有意改动）

**文件**：`packages/language_tools/src/lsp_button.rs`（改动前与 Zed **零差异**，改动后
`diff` 约 234 行）

### 动机：上游命名与实际行为不符

上游每个 server item 只有一个 `Stop Server`，但它**实际做的是注销/移除**：
`stop_language_servers_for_buffers` → `stop_local_language_server`
（`packages/project/src/lsp_store/mod.rs:12841`）会移除 `language_server_ids`、
清空该 server 的全部诊断、优雅 shutdown、并把 name 写进 `stopped_language_servers`
抑制自动启动；随后 `LspStoreEvent::LanguageServerRemoved` 触发
`remove_server`（`lsp_button.rs:781-788`）清掉面板的 `servers_per_buffer_abs_path`
→ **条目消失**，且**无法单独恢复**，只能走全局 `Restart All Servers`。

即"停止"这个标签下藏着一个**无法单独恢复**的操作（仍可走全局 `Restart All Servers`
恢复，但那会顺带重启所有 server），连维护者都会误解。

> 注意区分：**两者都可通过 `Restart All Servers` 恢复**（`restart_all_language_servers`
> 传空 selector + `clear_stopped=true`，会 `stopped_language_servers.clear()` 清空全部抑制
> 再重新注册，见 `lsp_store/mod.rs:13001` / `13063`）。
> Stop 与 Remove 的差别**只在恢复粒度**：前者可 per-item 单独恢复，后者只能全局恢复。

### 改动（7 项）

1. **改名**
   - per-item：`Stop Server` → `Remove Server`（逻辑一字未动）
   - 全局：`Stop All Servers` → `Remove All Servers`（与 `Restart All Servers` 平行）

2. **`Remove Server` 在「运行中」和「已停止」两种状态都提供**
   - 上游只有一个动作，挂在 `if can_stop`（即 `BinaryStatus::None | Starting`）下就够了；
     拆分后这个继承来的守卫**不完备** —— 已停止的 server 会被漏掉，
     导致 Stop 之后**再也清不掉该条目**（只能靠 Restart All 顺带清）
   - 实现：移出 `if can_stop`，handler 合并两步
     1. `stop_language_servers_for_buffers(Vec::new(), {Id}, cx)` ——
        运行中真正注销；已停止时该 id 已不在 `language_server_ids`，整条链路是
        **静默 no-op**（`stopped_names` 为空、`stop_local_language_server` 取不到 name 直接返回），
        不会 panic 也不会报错
     2. `stopped_server_worktrees.remove(name)` —— 丢弃展示归属，条目消失
   - 已停止时 `stopped_language_servers` 里的抑制**保留**，所以不会自己复活

3. **新增真正的 `Stop Server`**（可恢复）
   - 停掉 server，但**条目保留**，可单独 `Start Server` 恢复
   - 实现：停止前先把该 server 的展示归属记入新字段（见下），再执行与上游
     完全相同的 `stop_language_servers_for_buffers(Vec::new(), {Id}, cx)`

4. **新增 `Start Server`**（仅停止态显示）
   - **必须传全部 buffers**（`lsp_store.buffer_store().read(cx).buffers()`）。
     理由：现有 per-item `Restart Server`（497-581）用的是
     `servers_per_buffer_abs_path` 收集的 buffers，停止后该列表为空 →
     `if !buffers.is_empty()` 不成立 → **静默 no-op**。这是最容易踩的坑。
   - `clear_stopped` 必须传 `true`：`stopped_language_servers` 会拦注册
     （`lsp_store/mod.rs:3200-3204`），只有 `clear_stopped=true` 才会先移除该
     name（`lsp_store/mod.rs:13065-13072`），server 才能起来。

5. **新增 `stopped_server_worktrees` 字段**（`lsp_button.rs` 的 `LanguageServers`）

   ```rust
   stopped_server_worktrees: HashMap<LanguageServerName, (WeakEntity<Worktree>, LanguageServerId)>
   ```
   - **这是 aacode 新增的状态，上游没有**，rev→rev 同步时注意
   - 用弱引用，与 `ServersForPath.worktree` 的 `Option<WeakEntity<Worktree>>` 风格一致
   - **不要**改用"停止时不清理 `servers_per_buffer_abs_path`"来实现条目保留：
     该 map 由更新事件重建，保留死 id 会在重启后产生**幽灵条目**。
     独立记录可彻底回避，且 `remove_server` 无需改动 → 现有
     `remove_server_drops_health_entry_for_id` /
     `remove_server_evicts_id_from_per_buffer_entries_and_drops_empty_entries` /
     `remove_server_does_not_touch_binary_statuses` 三个单测不受影响
   - Restart All / Remove All 两个全局分支都会 `clear()` 该字段

6. **item 构造新增第三来源**
   - 位置紧跟 `binary_statuses` 循环之后，遍历 `stopped_server_worktrees`，
     用 `emitted_server_names` 去重，从 `binary_statuses` 取 `Stopped` 状态
   - 渲染结果：灰色 + `Stopped`（`lsp_button.rs:367-369` 已处理该状态的配色文案）

7. **新增全局 `Stop All Servers`**
   - 上游全局只有 `ToggleServersButton { restart: bool }` 二选一，aacode 改为
     三选一枚举 `ToggleServersAction { StopAll, RemoveAll, RestartAll }`
   - ⚠️ **`StopAll` 不能走 `stop_all_language_servers`**（`lsp_store/mod.rs:12963`）。
     它会设 `all_language_servers_stopped = true` 这个**全局总闸**，在
     `register_buffer_with_language_servers`（3126）和 `refresh_server_tree`（6355）
     两处 early return 拦掉注册；而 `all_language_servers_stopped` 的写入点只有
     `stop_all`（置 true）与 `restart_all`（置 false），中间的
     `restart_language_servers_for_buffers` **不碰它** → 之后每个 per-item
     `Start Server` 都会被拦成**静默 no-op**
   - 正确实现：先把所有 server 的展示归属记入 `stopped_server_worktrees`，再
     `stop_language_servers_for_buffers(Vec::new(), {所有 id}, cx)` 一次停掉，
     **不设**那个总闸
   - `RemoveAll` 仍走 `stop_all_language_servers`（设总闸、条目消失、靠 Restart All 恢复）

### 状态矩阵（改动后）

| server 状态       | 可用动作                                         |
| ----------------- | ------------------------------------------------ |
| 运行中 / Starting | `Restart Server`、`Stop Server`、`Remove Server` |
| 已停止（Stopped） | `Start Server`、`Remove Server`                  |

| 全局（按面板自上而下顺序） | 说明                                   |
| -------------------------- | -------------------------------------- |
| `Restart All Servers`      | 全部拉回（**对齐上游，始终排最上面**） |
| `Stop All Servers`         | 条目保留，可逐个单独 Start 恢复        |
| `Remove All Servers`       | 条目消失，只能 Restart All 恢复        |

（有 server 在运行时三者都显示，顺序如上；全部停止时只显示 `Restart All Servers`。）

> 顺序对齐上游：上游 `if can_stop_all` 先 push `restart: true`（Restart All）再 push
> `restart: false`（Stop All）。新增 `StopAll` 时不要把它排到 Restart 前面。

| 动作            | 条目                 | 恢复粒度                                                         |
| --------------- | -------------------- | ---------------------------------------------------------------- |
| `Stop Server`   | 保留（灰色 Stopped） | **单独**恢复：per-item `Start Server`                            |
| `Remove Server` | 消失                 | **只能全局**恢复：`Restart All Servers`（会顺带重启所有 server） |

### 保留的两个细节（有意为之）

- `can_start` 用**双条件或**：`BinaryStatus::Stopped` **或** name 在
  `stopped_server_worktrees` 中。前者不能省 —— `Stopped` 状态并不只由「移除」产生，
  上游本来就存在该状态且有灰色标识（例如 Stop All 等路径也会置为该状态）。
- 运行中的 server 会同时出现 `Restart Server` + `Stop Server` + `Remove Server` 三项，
  `Stop` 与 `Remove` 的差异已在代码注释中说明（暂未做 tooltip）。

### rev→rev 同步注意

上游若改动以下任一项，需人工合并而非直接覆盖：
`remove_server`（781-788）、item 构造（约 1210-1400）、per-item 子菜单（约 415-715）、
全局按钮分支（约 303-340）。合并后请确认 `stopped_server_worktrees` 的三处引用
（写入 / 移除 / 清空）仍然完整。

## Settings → AI：新增「试听」按钮（aacode 独有，非移植）

`settings_ui/src/page_data.rs` 的 `ai_page()` 里，在 `Play Sound When Agent Done`
之后插入一个 `SettingsPageItem::ActionLink`：

```rust
SettingsPageItem::ActionLink(ActionLink {
    title: "Test Agent Done Sound".into(),
    button_text: "Play".into(),
    on_click: Arc::new(|_settings_window, _window, cx| {
        audio::Audio::play_sound(audio::Sound::AgentDone, cx);
    }),
    files: USER,
}),
```

**上游 zed 没有这一项。** zed 只有 Collaboration 页的 Test Audio
（`page_data.rs:8555`），那走完整回环测试（`audio_test_window.rs` 同时
`open_input_stream` + `open_test_output`）。

差异点：

- 只播提示音，**不碰麦克风**
- 放在 Agent 页而非 Collaboration 页，紧邻对应开关，改完立刻能听到效果
- 用途：排查本机 audio 输出设备是否可用（本机 `alsa::poll()` POLLERR 的排查工具）

`settings_ui` 本就依赖 `audio`（`Cargo.toml:16`），无需新增依赖。

注意 `ai_page` 的 `general_section()` 返回类型写死 `[SettingsPageItem; 8]`，
加项要同步改成 9 —— 该签名是 aacode 自己的简化（zed 是 `Vec`），非移植差异。

## Pane toolbar：2 个 item 未移植（其余 24 个已接）

Zed `crates/zed/src/zed.rs:1457` 的 `initialize_pane` 往每个 pane 挂 26 个
toolbar item。aacode 已接 24 个，**以下 2 个无对应实现，rev→rev 时不必同步**：

| item                          | 原因                                                                                                                                                       |
| ----------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `TelemetryLogToolbarItemView` | zed crate 内部模块（打开遥测日志窗口的入口）。                                                                                                             |
| `BasedPyrightBanner`          | 所属 crate `language_onboarding` **整个未移植**（aacode 有 `packages/languages` 里的 `BasedPyrightLspAdapter`，即 LSP 本身是有的，只缺首次安装引导横幅）。 |

移植时的两个 import 细节（照抄时会踩）：

- `gpui::AppContext as _` 必引 —— `cx.new()` 是该 trait 的方法，漏掉会报
  「no method named new found for Context<Toolbar>」（对应 zed.rs:43）
- `LspLogToolbarItemView` 走模块路径 `language_tools::lsp_log_view::`，
  因 crate 根只 re-export 了 `LspLogView`（对应 zed.rs:53）

挂载点在 `app/src/initialize/pane_toolbar.rs`，对齐 zed L550 的
`observe_new::<Workspace>` + `Event::PaneAdded` 订阅。模块名 `pane_toolbar`
是 aacode 拆出来的（zed 内联在 zed.rs），与 `panels.rs` 的
`initialize_panels`（左侧 Dock Panel）刻意区分 —— 两者在 Zed 里只差复数。

### 已搬入 app crate 的两个 zed 内部组件

原属 zed crate、无独立 crate 的组件，移植到 `app/src/core/` 下（模块名可改，
函数名保留原样）：

| 组件              | aacode 位置                                                                          | 上游位置                                  |
| ----------------- | ------------------------------------------------------------------------------------ | ----------------------------------------- |
| `QuickActionBar`  | `core/quick_action_bar.rs` + `quick_action_bar/{preview,repl_menu}.rs`（共 1460 行） | `crates/zed/src/zed/quick_action_bar*.rs` |
| `MigrationBanner` | `core/migrate/mod.rs`（326 行）                                                      | `crates/zed/src/zed/migrate.rs`           |

两者照抄所需的 workspace 依赖：`picker` / `markdown` / `migrator`
（`MigrationBanner` 用 `migrator::{migrate_keymap, migrate_settings}`；
aacode 的 `packages/migrator` 已含这两个函数）。
`MigrationBanner` 还需在 `initialize_pane` 开头取 `cx.weak_entity()`
（对齐 zed.rs:1463）。

## util → ac_util：app 级函数上提（aacode 独有重构）

**新 crate**：`packages/ac_util/`，从 `packages/util/src/os.rs` 和
`packages/util/src/shell_env.rs` 中提取 app 启动/CLI 专属功能。

### 拆分清单

| 功能                                                                       | 位置                  | 原因                                                                 |
| -------------------------------------------------------------------------- | --------------------- | -------------------------------------------------------------------- |
| `prevent_root_execution`                                                   | **ac_util**           | CLI 入口和 app 启动期安全守门，与通用工具无关                        |
| `get_shell_safe_zed_path`                                                  | **ac_util**           | 格式化当前可执行路径供 shell capture 使用                            |
| `get_zed_cli_path`                                                         | **ac_util**           | 定位同级 `zed-cli`（现在是 aacode 的 CLI 二进制）                    |
| `load_login_shell_environment`                                             | **ac_util**           | 加载用户登录 shell 环境（耗 IO、有副作用）                           |
| `shell_env::capture` / `*_unix` / `*_windows` / `spawn_and_read_fd` 等整套 | **ac_util::shell_env** | 上述 capture 的完整实现，依赖 `command_fds`/`smol::process`/`nix` 等 |
| `set_pre_exec_to_start_new_session`                                        | **留在 util**         | 通用 `Command` 辅助，`terminal`/`ac_util::shell_env` 等多处调用      |
| `increase_open_file_limit`                                                 | **留在 util**         | 通用 rlimit 工具                                                     |
| `parse_os_release`                                                         | **留在 util**         | 通用 os-release 解析                                                 |
| `shell_env::print_env`                                                     | **留在 util**         | `--printenv` 子命令入口，纯打印逻辑                                  |

### 消费者迁移

改完之后，引用 `prevent_root_execution` / `load_login_shell_environment` /
`shell_env::capture` 的 crate 需要改为 `ac_util::`：

| crate           | 旧                                     | 新                                        |
| --------------- | -------------------------------------- | ----------------------------------------- |
| `cli`           | `util::prevent_root_execution()`       | `ac_util::prevent_root_execution()`       |
| `remote_server` | `util::load_login_shell_environment()` | `ac_util::load_login_shell_environment()` |
| `project`       | `util::shell_env::capture(...)`        | `ac_util::shell_env::capture(...)`        |

`set_pre_exec_to_start_new_session` 仍留在 `util`，调用方不改。

### rev→rev 同步注意

Zed 上游的 `crates/util/src/os.rs` 和 `crates/util/src/shell_env.rs` 仍包含
上述所有函数（Zed 没有做这个拆分）。同步时：

1. 对上游这两个文件的改动，先判断**属于哪一类**（ac_util 侧 vs util 侧）
2. ac_util 侧的改动手动 port 到 `packages/ac_util/src/{os,shell_env}.rs`
3. util 侧的改动（`set_pre_exec_to_start_new_session` / `increase_open_file_limit` /
   `parse_os_release` / `shell_env::print_env`）port 到 `packages/util/src/{os,shell_env}.rs`
4. 不要把 Zed 的完整 `shell_env.rs` 直接覆盖 aacode 的 `util/src/shell_env.rs`，
   否则会重新引入已迁走的 capture 系列函数 + 关联依赖（`collections::HashMap`
   已在 util 侧被移除）

### util 侧 Cargo.toml 瘦身

迁走后 `util/Cargo.toml` 的 `[target.'cfg(unix)'.dependencies]` 可移除
`command-fds`；`nix` 只保留 `resource` feature（`user` 已迁到 ac_util）。
`[dependencies]` 里的 `smol` / `which` / `async-fs` / `walkdir` / `dirs`
在 wasm 侧仍被其他模块用到，保留不动。

## 本地 fork crate → gpui_learn 迁移

以下 crate 已从 aacode `packages/` 本地删除，改由 gpui_learn git 依赖供应：

| 旧路径                              | 现在                               | workspace.dependencies key | package 名在 gpui_learn 里 |
| ----------------------------------- | ---------------------------------- | -------------------------- | -------------------------- |
| `packages/git/` （aa_gpui_kit_git）  | gpui_learn `packages/aa_gpui_kit/aa_gpui_kit_git/` | `git`                      | `aa_gpui_kit_git`          |
| `packages/fs/` （aa_gpui_kit_fs）    | gpui_learn `packages/aa_gpui_kit/aa_gpui_kit_fs/`   | `fs`                       | `aa_gpui_kit_fs`           |
| `packages/a_log/`                    | gpui_learn `packages/aa/zlog/`     | `zlog`                     | `zlog`                     |
| `packages/a_tracing/`                | gpui_learn `packages/aa/ztracing/` | `ztracing`                 | `ztracing`                 |
| `packages/a_tracing_macro/`          | gpui_learn `packages/aa/ztracing_macro/` | `ztracing_macro`       | `ztracing_macro`           |

**消费者源码同步**（一次性完成）：

- `a_log::` → `zlog::`，`a_tracing::` → `ztracing::`（全仓库 `.rs` 文件和 Cargo.toml）
- workspace.dependencies 里新增 `zlog`/`ztracing`/`ztracing_macro` git 依赖
- `fs`/`git` 用 `{ package = "aa_gpui_kit_*", git = ..., rev = ... }` 形式

**rev 管理**：gpui_learn 新提交后 bump aacode 的 workspace.dependencies 里对应 rev
（目前 5 个 git 依赖同指向 gpui_learn 主分支最新 rev）。

**仍在本地的同类 crate**（未迁移）：

- `packages/askpass/` — gpui_learn 有 `aa_gpui_kit_askpass`，但 aacode 还留着本地版
- `packages/telemetry/` + `packages/telemetry_events/` — gpui_learn 已有 `zed/telemetry`
- `packages/proto/` — gpui_learn 已有 `zed_proto`
- `packages/paths/` — gpui_learn 已有 `zed_paths`
- `packages/rope/`、`packages/text/` — zed rev 同步，不在 gpui_learn

## Release channel：去掉 RELEASE_CHANNEL 文件，改为「env 或 dev」

**改动**：`packages/release_channel/src/lib.rs` 的 `compile_time_release_channel_name()`。
`packages/app/RELEASE_CHANNEL` 已删除。

### Zed 的做法

```rust
#[cfg(__do_not_set_zed_release_channel)]
fn compile_time_release_channel_name() -> String {
    env!("ZED_RELEASE_CHANNEL").trim().to_string()          // 运行期缺它会 panic
}

#[cfg(not(__do_not_set_zed_release_channel))]
fn compile_time_release_channel_name() -> String {
    include_str!("../../zed/RELEASE_CHANNEL").trim().to_string()
}
```

`build.rs` 在检测到构建期有 `ZED_RELEASE_CHANNEL` 时置
`__do_not_set_zed_release_channel` cfg，保证两条分支严格互斥。

**为什么 Zed 需要文件**（源码注释原文）：

> When a crate in zed is used as a dependency that uses the `crane` nix
> library, it vendors each crate separately and builds it in isolation, which
> makes the `include_str!` fail.

即 crane/Nix 把每个 crate 单独 vendor 到独立 store 路径编译，`include_str!` 的
相对路径在那里不存在 → 编译失败。**这是 Nix 构建的约束，aacode 不用 crane 就不成立。**

**文件在 Zed 里的第二重作用**：CI 靠改写它来切频道，且改完连同 tag 一起提交。

| workflow                              | 操作                                                             |
| ------------------------------------- | ---------------------------------------------------------------- |
| `bump_zed_version.yml:166`            | `echo -n preview > crates/zed/RELEASE_CHANNEL` 后建 preview 分支 |
| `bump_zed_version.yml:229`            | `echo -n stable > ...` 后打 stable tag                           |
| `release_nightly.yml:124/174/292/359` | `echo "nightly" > ...`（Windows 两处用 `Set-Content`）           |

所以 zed 的发布流程是「改文件 + 提交 + 打 tag」，而非注入 env。

### aacode 的做法

```rust
fn compile_time_release_channel_name() -> String {
    option_env!("ZED_RELEASE_CHANNEL").unwrap_or("dev").trim().to_string()
}
```

一条路径覆盖两种情况：有 env 用它（等价 zed 的 `env!` 分支但不 panic），
无 env 则 `dev`。同时删掉 `__do_not_set_zed_release_channel` cfg ——
它原本只为让两个分支互斥，现在只有一个函数，cfg 已无意义，
`packages/release_channel/build.rs` 里的 `rustc-cfg` 一并删除。

### 行为对照

| 构建方式                                            | aacode                               | Zed                                    |
| --------------------------------------------------- | ------------------------------------ | -------------------------------------- |
| `just install`（注入 `ZED_RELEASE_CHANNEL=stable`） | stable，图标无后缀，db 落 `0-stable` | —                                      |
| 裸 `cargo build` / `cargo run`（无 env）            | dev，图标 `-dev`，db 落 `0-dev`      | 同（仓库文件也是 `dev`）               |
| Nix/crane 构建                                      | 不适用                               | 必须注入 env，否则 `include_str!` 失败 |

### 连带改动

`packages/app/build.rs` 的图标后缀取值改为**与 lib.rs 同源**
（`option_env!("ZED_RELEASE_CHANNEL").unwrap_or("dev")`），并移除对文件的读取与
`rerun-if-changed`。两者必须同源，否则会出现「图标是 dev 的、实际行为却是
stable」的不一致。

### rev→rev 同步注意

从 Zed 同步 `release_channel` 时，**不要**把 `#[cfg(__do_not_set_zed_release_channel)`
双分支和 `include_str!` 带回来（那是 crane 专用）。同时留意 Zed 若新增对
`RELEASE_CHANNEL` 文件的其他读取点，那些在 aacode 一并不存在。

若将来 aacode 要发正式版：**必须**在构建期注入 `ZED_RELEASE_CHANNEL`，否则产物
一定是 dev（图标后缀与 db 路径都会暴露出来）。`just install` 已这么做；
`script/bundle-linux` 目前**尚未确认**是否注入 —— 上 CI 前需检查。
