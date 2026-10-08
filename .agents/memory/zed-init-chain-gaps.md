# Zed 移植：上游 init 链缺口

> 记录 aacode 相对 Zed **缺失的初始化调用**。这类缺口的特点是**不报编译错、大部分不 panic，但对应功能静默失效**，只能靠 diff 上游 init 链或看症状反推。

## 为什么单独记这个

搬 Zed 代码时，`init` 链（`packages/app/src/main.rs`）是**最容易被漏掉**的部分：
- 漏掉 → 编译通过、能跑、不 panic，只是**功能静默缺失**
- 比缺 UI 组件更隐蔽：UI 组件缺失会编译报错，init 缺失不会

已确认修好的：

### LLM Provider 注册（已修，commit `5e45a86`）

**症状**：Settings → LLM Providers 子页整页空白，**无任何报错/告警**。General / AI 等其它页面正常。

**根因**：aacode 漏了 Zed `crates/zed/src/main.rs:694-700` 的两步初始化：

```rust
language_model::init(cx);                                    // aacode 已有
RefreshLlmTokenListener::register(                            // 缺失 → 启动 panic
    app_state.client.clone(),
    app_state.user_store.clone(),
    cx,
);
language_models::init(app_state.user_store.clone(), app_state.client.clone(), cx);  // 缺失 → 静默空白
```

两个关键点：

1. **`language_model::init` ≠ `language_models::init`**
   - `language_model`（`packages/language_model`）= 核心 trait / 类型 crate，只负责建 `LanguageModelRegistry` 这个**容器**
   - `language_models`（`packages/language_models`）= 内部调 `register_language_model_providers(...)` 把各 provider **塞进 registry**
   - 看到 `language_model::init` 就以为"初始化过了"是最常见的误判

2. **空 provider 列表会渲染成"正常"的空页面**
   `settings_ui/src/pages/llm_providers_page.rs:24-46` 的 `render_llm_providers_page` 内容 100% 来自
   ```rust
   let providers = LanguageModelRegistry::read_global(cx).visible_providers();
   ```
   为空时 `.children(...)` 就是零子元素 → 一个**没有任何子元素**的 `v_flex` → 全白、零报错。
   Zed 同文件逐字相同，所以**问题一定在 app 侧，不在 settings_ui**。

3. `RefreshLlmTokenListener::register` 是**硬前置**：`language_models/src/provider/cloud.rs:120` 会调
   `RefreshLlmTokenListener::global(cx)` 读这个全局，少这行直接 panic
   （`no state of type client::llm_token::GlobalRefreshLlmTokenListener exists`，`gpui/src/app.rs`）。

## 已修：34 个 crate 已移植但 init 从未接线

**症状**：无编译错误、无 panic，对应功能静默失效。例如 `diagnostics` / `encoding_selector` /
`line_ending_selector` 的 action 已在 status_bar 菜单里点了没反应。

**根因**：这批 crate 全部 fork 进仓、`pub fn init` 也都写好了，但 `packages/app/src/main.rs`
里一次都没调用。多数还**已经在 `[dependencies]` 里**——所以它们被链接了、action 进了 inventory，
只是 handler 永远不注册。

**修复**：按 Zed `main.rs` 的原始顺序补齐调用（无参数的一批插在 `ui_prompt::init` 之后；
需要 `client` / `fs` / `app_state` 的插在对应变量就绪之后），并补 11 个缺失的 `[dependencies]`。

已接线：`git_hosting_providers` `debugger_tools` `command_palette` `acp_tools`
`edit_prediction_ui` `web_search` `snippet_provider` `recent_projects` `dev_container`
`image_viewer` `diagnostics` `audio` `go_to_line` `outline` `tasks_ui` `search`
`lsp_locations` `vim` `encoding_selector` `language_selector` `line_ending_selector`
`toolchain_selector` `language_tools` `feedback` `markdown_preview` `svg_preview`
`edit_prediction` `json_schema_store` `which_key` `client` `auto_update` `dap_adapters`
`copilot_chat` `copilot_ui` `extension` `extension_host` `debug_adapter_extension`
`language_extension` `web_search_providers` `file_finder` `call_hierarchy` `journal`
`extensions_ui` `miniprofiler_ui`

至此事项清零：审计脚本已报不出任何「crate 在 + 有 `pub fn init` + app 未调用」的项。

### 坑：移植 proc 宏 derive 时必须加 `#[register_component(crate = "...")]`

`extensions_ui/src/components/extension_card.rs` 的 `#[derive(RegisterComponent)]` 报
`cannot find module or crate 'aa_gpui_kit_component'`。这是 zed-port.md #12 的情形：
proc 宏展开时看不见 Cargo 别名，只认自己的真名。修法是在消费方那一行加属性
（aacode 已有 5 处既有用法，如 `agent_ui/src/ui/session_notice.rs:72`）：

```rust
#[derive(IntoElement, RegisterComponent)]
#[register_component(crate = "component")]
```

### 坑：拆模块后 workspace 的公开 API 会「少几个符号」

新 crate 照抄 zed 后常直接 `use workspace::{OpenResult, ...}`，但 aacode 把 workspace
从单文件拆成了子模块，这些符号虽 `pub` 却没从 crate 根导出，报 `is private`。
修法是在 `packages/workspace/src/lib.rs` 第 180 行那个 `pub use crate::workspace::{...}`
块里补上（注意别和后面的私有 `use workspace::{...}` 块重复导入，否则 E0252）。

本次补了 `open::local::open_paths`、`open::options::OpenResult`、`nav::MAX_RECENT_SELECTIONS`。

### 坑：零引用 crate 里的悬空引用，只有 `--workspace` 才暴露

`packages/http_client/src/github.rs:32` 写着
`#[serde(default, deserialize_with = "deserialize_sha256_digest")]`，但函数体从未存在
——`Deserializer` 被 import 却无人使用。根因是 `1f370f5` 把「反序列化后遍历 assets 剥离
`sha256:` 前缀」改成 `deserialize_with` 时漏加了函数。

关键背景：该 crate 的 Cargo 名是 `aa_http_client`，**全仓库零引用**（实际跑的是 workspace
里 zed git 依赖的 `http_client`）。也就是说这个 bug 在死代码里，不影响任何运行时行为，
但它让 `cargo check --workspace` 失败。

> 教训一：**检查范围要放到 `--workspace`**。只 check 主线包会漏掉这类问题。
> 教训二：补函数体前先看 zed 上游有没有现成实现——这里 `afecd6d` 早就有，
> 应该照抄而不是自己从被删代码反推（我第一次反推的版本比上游多了 `map` 闭包，
> 虽等价但制造了无谓 diff）。
> 详见 `.agents/zed-modifications.md` 的「http_client：两套并存」一节。

### 坑：不要凭 crate 名猜「需要什么依赖」

`copilot_chat::init` 要 `CredentialsProvider` + `CopilotChatConfiguration`，我一度判定
「aacode 无 Copilot 账号体系，故意不接」。实际两个依赖都齐：

- `packages/ad_credentials_provider/src/lib.rs` **完整实现了** `CredentialsProvider`
  （keychain + development 两种实现），并提供 `global(cx) -> Arc<dyn CredentialsProvider>`。
  它就是 Zed 的 `zed_credentials_provider`，只是改了名。
- `CopilotChatConfiguration` 就在 `copilot_chat/src/lib.rs:28`；其 `enterprise_uri` 的取值路径
  `language::language_settings::all_language_settings(None, cx).edit_predictions.copilot.enterprise_uri`
  在 `packages/language/src/language_settings/mod.rs:607`（`CopilotSettings`）完整存在。

**教训**：判定「无法接线」前必须逐个 grep 验证依赖是否真的不存在。只看 crate 名容易得出
「整块没移植」的错觉——那 10 个真正没移植的 crate（无 `packages/<name>` 目录）反而更容易识别。

## 仍缺（有意不移植）

- `etw_tracing` — Windows-only ETW 追踪，aacode 无此 crate
- `component_preview` — Zed 官网组件预览工具，与产品功能无关

### 顺带发现：扩展子系统整块没接线

补上述 init 时发现 aacode 里**根本没有 `extension_host_proxy` 这个变量**——
`extension::init` + `ExtensionHostProxy::global(cx)`（Zed `main.rs:524-525`）两步都缺。
照抄后 `debug_adapter_extension` / `language_extension` / `extension_host` 才拿得到 proxy。

其中 `ExtensionHostProxy` 是从 **`extension`** crate 导入的（`use extension::ExtensionHostProxy`），
不是 `extension_host`——后者只有私有 re-export。

### 顺带发现：fork 拆模块时漏了 re-export

`encoding_selector` 编译报 `cannot find function 'init' in crate 'encoding_selector'`。
根因：Zed 的 crate root **就是** `src/encoding_selector.rs`（其 `Cargo.toml` 有
`[lib] path = "src/encoding_selector.rs"`），所以上游 `pub fn init` 直接是 crate 根函数。
本 fork 拆成 `lib.rs` + `mod encoding_selector;` 后只 re-export 了 `ActiveBufferEncoding`，
漏了 `init`。**这类错误只在接线那一行才炸**，其余代码照常编译。

## 怎么系统排查

新增/移植功能时，把上游的 init 链 diff 一遍，别只补用到的那一个。

**必须扫整个 app crate，不能只扫 `main.rs`**——`packages/app/src/initialize/` 里还有
36 处 `on_action` / `register_action`（含 `initialize::init`，对应上游 `zed::init`）。
只 grep `main.rs` 会把这些全误判成「上游有、aacode 缺」。

```bash
# 1. 上游 init 调用（90 项）
grep -nE "^\s+[a-z_]+::(init|register)\(" <zed>/crates/zed/src/main.rs

# 2. aacode 侧：扫整个 app crate，且要匹配 init/register 两种形态
grep -rhoE "\b[a-z_0-9]+::(init|register)\(" packages/app/src/ | sort -u

# 3. 对差集逐项确认 crate 是否已移植 + 是否有 pub fn init
#    → 「crate 在 + 有 init」= 真缺口
```

重点关注 Zed 中**同一功能域内成组出现**的调用（如 provider 注册的
`RefreshLlmTokenListener::register` + `language_models::init`）—— 它们通常有顺序依赖，
漏一个的表现往往和漏另一个完全不同（一个是 panic，一个是静默空白）。

另外注意 ` Zed::init` **不是**「统一 init 入口」，它只装 `crates/zed` 私有类型才能实现的
action handler（`Hide` / `OpenLog` / `OpenSettingsFile` 等 12 个）。其余 init 必须住在各自 crate
里（`crates/zed` 依赖几乎所有 crate，反向依赖会成环），「统一」的只是 `main.rs` 里的调用顺序。


## 必须实跑验证：init 顺序错误编译期查不出来

补完 init 链后**一定要 `./target/debug/aacode` 实跑**。编译通过 ≠ 能启动。

本轮实跑抓到 2 个 panic，都是「init 缺前置 set_global」或「init 被插队到依赖方之前」：

### 1. 漏抄 set_global（照抄时只抄了 init 那行）

`git_hosting_providers::init(cx)` 内部读 `GlobalGitHostingProviderRegistry`，但 Zed 里
它前面还有两行必须一起抄：

```rust
// app.run 之前（zed main.rs L423）
let git_hosting_provider_registry = Arc::new(GitHostingProviderRegistry::new());
// init 之前（zed main.rs L519）
GitHostingProviderRegistry::set_global(git_hosting_provider_registry, cx);  // 注意是 move，不是 clone
```

> 用 `.clone()` 会触发 E0373「closure may outlive the current function」——
> Zed 是把 Arc **move** 进 `app.run` 闭包的。

### 2. 旧 init 位置太晚，被新 init 提前触发

`release_channel::init` 原先在第 339 行（app_state 之后），而 Zed 在 L492 极早期。
本来没人读它所以不炸；一旦有 init 在它之前读 `GlobalReleaseChannel` 就 panic。
实际触发者：`ad_credentials_provider::global`（按 Dev/Release 决定系统 keychain vs
development 文件）与 `git_hosting_providers`。

> 教训：把新 init 插到链子前部时，要顺带检查**被它插队的前置全局**是否已就绪。

### 判定某全局是否安全就绪的快捷方法

```bash
# 谁 set_global 它
grep -rn "X::set_global\|set_global(X" packages/*/src/
# 谁读它（读的地方必须在 set_global 之后被调用）
grep -rn "X::global\|global::<X>" packages/*/src/
```

然后确认 app 的调用顺序里 set_global 的位置早于所有读者。

## 环境噪声（不是 init 问题）

实跑日志里的这些 ERROR 可忽略，与 init 链无关：

- `sender was dropped` — oxfmt / prettier 后台任务随进程被杀中断
- `no language server download dir defined` — 未设 LSP 下载目录
- `status error 403 ... API rate limit exceeded` — GitHub 匿名 API 限流（本机出口 IP）
