# aacode 的 Agent 家族依赖树

> 分析日期 2026-10-07，基于本仓库当前 workspace。
> 目的：为「把方向放到 agent 上」提供依赖结构的基线 —— 哪些 crate 在哪一层、改动会波及什么。

## 结论

7 个 crate，**内部无循环依赖**（已用拓扑检测验证）。分层清晰，但有两处结构性耦合需要注意
（见文末）。

```
                        acp_tools          ← 最上层，不被本家族任何 crate 依赖
                             │
            ┌────────────────┴──────────────────┐
            │                                   │
        agent_ui  ← 55 个本地依赖                │
            │                                   │
            ├────────────────┬──────────────────┤
            ↓                ↓                  ↓
        acp_thread          agent ──────────→ agent_servers
            │                │                       │
            ↓                └───────────┬───────────┘
    agent_settings  ←──────────── agent_skills
```

## 分层表

| 层 | crate | 家族内依赖 | 本地依赖总数 | 有效代码量 | 职责 |
|---|---|---|---|---|---|
| L4 应用 | `acp_tools` | `agent_servers` `agent_ui` | 10 | 842 行 | ACP 工具的 UI 入口（也是 pane toolbar item） |
| L3 UI | `agent_ui` | `acp_thread` `agent` `agent_servers` `agent_settings` `agent_skills` | **55** | 9.0 万行 | Agent 面板、会话视图、diff 卡、终端卡 |
| L2 逻辑 | `agent` | `acp_thread` `agent_servers` `agent_settings` `agent_skills` | 31 | 5.1 万行 | Agent 主循环、工具调用、上下文管理 |
| L2 逻辑 | `agent_servers` | `acp_thread` | 18 | 7.2 千行 | 外部 agent 服务器接入（google_ai 等） |
| L1 会话 | `acp_thread` | `agent_settings` | 19 | 2.4 万行 | Agent Client Protocol 线程 / 会话 |
| L0 配置 | `agent_settings` | —（家族内无下游） | 6 | 2.8 千行 | 模型配置、profile、工具权限 |
| L0 配置 | `agent_skills` | —（家族内无下游） | **3** | 2.2 千行 | Skills 定义与加载 |

> `agent` 的 8.9 万行原始行数里，2.1 万是 `tools/evals/fixtures/`（测试用的大文件）、
> 9.5 千是 `src/tests/`。剔除非生产代码后约 **5.1 万行**。

## agent 家族内部的依赖边（完整）

```
agent         → acp_thread, agent_servers, agent_settings, agent_skills
agent_ui      → acp_thread, agent, agent_servers, agent_settings, agent_skills
agent_servers → acp_thread
acp_thread    → agent_settings
acp_tools     → agent_servers, agent_ui
agent_settings → （无家族内依赖）
agent_skills   → （无家族内依赖）
```

## 两个结构性耦合

### 1. `acp_tools` 反向依赖 `agent_ui`

`acp_tools` 处在 L4（最上层、不被依赖），却依赖 L3 的 `agent_ui`：

```
acp_tools → agent_servers, agent_ui
```

后果：`acp_tools` 无法独立编译或复用，只要动 `agent_ui` 就会被牵连。若要让 ACP 工具
脱离完整 UI 单独演进，这个边需要断开。

### 2. `agent_ui` 依赖面极宽（55 个本地 crate）

它跨越了 `editor` `workspace` `terminal_view` `search` `picker` `markdown` `git_ui_core`
`lsp` `multi_buffer` `remote` `remote_connection` 等 —— 说明 Agent 面板与编辑器是**深度
耦合**的（内嵌 diff 视图、终端、文件预览、搜索/跳转）。

后果：**精简 Agent UI 的改动面很大**。`agent_ui` 内部最大的三个文件是：

```
agent_panel.rs                14292 行
conversation_view/mod.rs      14034 行
conversation_view/thread_view.rs 13195 行
```

## 各 crate 的关键外部依赖（超出家族的部分）

| crate | 关键依赖 |
|---|---|
| `agent` | `project` `db` `client` `language_models` `cloud_llm_client` `context_server` `sandbox` `streaming_diff` `prompt_store` `web_search` |
| `agent_ui` | 上表全部 + `editor` `workspace` `terminal_view` `search` `picker` `git_ui_core` `lsp` `markdown` `remote` `remote_connection` |
| `acp_thread` | `multi_buffer` `terminal` `buffer_diff` `sandbox` `markdown` `http_proxy` |
| `agent_servers` | `google_ai` `reqwest_client` `remote` `terminal` `ad_credentials_provider` |
| `agent_settings` | `language_model` `project` `settings` `fs` `paths` |
| `agent_skills` | `fs` `paths` `util`（几乎零耦合） |
| `acp_tools` | `project` `workspace` `ui` `markdown` `language` `theme_settings` |

## 切入点建议

按改动成本从低到高：

1. **`agent_skills`** —— 只依赖 `fs` / `paths` / `util`，与 agent 主链路完全解耦。
   要做轻量功能（Skills 管理、校验、浏览）从这里开始代价最低。

2. **`agent_settings`** —— 家族最底层（无家族内下游），改配置结构不会触发上层重编。
   6 个依赖，其中 `settings` / `language_model` 是核心。

3. **`acp_tools`** —— 体积最小（842 行），但因反向依赖 `agent_ui` 而无法独立编译。

4. **`agent_ui`** —— 55 个依赖、9 万行，改动会波及编辑器/工作区/终端/搜索。
   要动这里，建议先明确是「瘦身」还是「加功能」，两者成本差异很大。

## 与平台适配的关系

本家族依赖的 `fs` / `project` / `client` / `http_client` 等基础设施，与平台适配是正交的
—— agent 本身不依赖任何 GUI 平台层（除 `agent_ui` / `acp_tools` 这两个 UI crate）。

也就是说：**若将来考虑移动端，agent 的 L0–L2 层（`agent_settings` / `agent_skills` /
`acp_thread` / `agent` / `agent_servers`）理论上是可复用的**，只有 L3/L4 需要为移动端
重新做交互形态。