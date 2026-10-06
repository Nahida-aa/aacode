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

## 待查：可能仍缺失的 init

Zed `crates/zed/src/main.rs:696-706` 还有以下调用，aacode `main.rs` 中**没有**对应。
它们不会导致启动 panic（它们反过来依赖已注册好的东西），但意味着对应功能是缺的：

| Zed 位置 | 调用 | aacode | 影响 |
|---|---|---|---|
| `main.rs:701` | `acp_tools::init(cx)` | ❌ 缺 | ACP 工具集 |
| `main.rs:702` | `zed::telemetry_log::init(cx)` | ❌ 缺 | 遥测日志落盘 |
| `main.rs:703` | `zed::remote_debug::init(cx)` | ❌ 缺 | 远程调试 |
| `main.rs:704` | `edit_prediction_ui::init(cx)` | ❌ 缺 | 编辑预测 UI |
| `main.rs:705` | `web_search::init(cx)` | ❌ 缺 | Web 搜索 |
| `main.rs:706` | `web_search_providers::init(client, user_store, cx)` | ❌ 缺 | Web 搜索 provider |
| `main.rs:707+` | `snippet_provider::init(cx)` | ❌ 缺 | 代码片段 |

> 注：aacode 已移植了 `inspector_ui`（commit `462bab6`）并接上 `dev::ToggleInspector`，走的是自己的路径。
> 上表**未逐个验证**实际影响，只是"上游有、aacode 没有 init 调用"的候选清单 —— 接入前需逐个确认依赖是否已就位。

## 怎么系统排查

新增/移植功能时，把上游的 init 链 diff 一遍，别只补用到的那一个：

```bash
# 1. 列出上游 init 调用
grep -nE "^\s+[a-z_]+::(init|register)\(" <zed>/crates/zed/src/main.rs

# 2. 逐条对照 aacode
grep -nE "^\s+[a-z_]+::(init|register)\(" packages/app/src/main.rs
```

重点关注 Zed 中**同一功能域内成组出现**的调用（如 provider 注册的
`RefreshLlmTokenListener::register` + `language_models::init`）—— 它们通常有顺序依赖，
漏一个的表现往往和漏另一个完全不同（一个是 panic，一个是静默空白）。
