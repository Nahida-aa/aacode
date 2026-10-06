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
`copilot_ui` `extension` `extension_host` `debug_adapter_extension` `language_extension`

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

## 仍缺（有意不接）

| crate | 原因 |
|---|---|
| `copilot_chat` | 需 `CredentialsProvider` + `CopilotChatConfiguration`，aacode 无 Copilot 账号体系 |
| `web_search_providers` / `edit_prediction_registry` / `miniprofiler_ui` / `file_finder` / `call_hierarchy` / `journal` / `extensions_ui` / `settings_profile_selector` / `etw_tracing` / `component_preview` | aacode 无对应 crate（整块未移植） |

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

