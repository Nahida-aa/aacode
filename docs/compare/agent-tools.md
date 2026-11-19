# Coding Agent 调研：工具集 + 架构 + 性能维度

> 调研日期：2026-11-12
> 目标：为自己 agent（aa-agent / aacode）选型参考。先弄清「一个 coding agent 应该有哪些工具」，再看 4 个开源 agent 怎么做，最后给 agent 性能（质量/速度/成本）该盯哪些榜。
>
> 调研对象（均本地已拉最新代码）：
> - **Pi** — `/home/aa/repos/ai_ls/learn_ls/pi`（earendil-works，TS）
> - **OpenCode** — `/home/aa/repos/ai_ls/learn_ls/opencode`（anomalyco，TS + Effect）
> - **DeepSeek Harness (dsh)** — `/home/aa/repos/ai_ls/learn_ls/deepseek-harness`（DeepSeek，TS + Cordis）
> - **Zed** — `/home/aa/repos/ide_ls/learn_ls/zed`（Rust，agent 是 app 内子系统）

## 1. 工具集横向对比（model-facing 内置工具）

★ = 你 v0 最需要的（读/写/编辑/命令/搜索）。

| 工具 | Pi | OpenCode | dsh | Zed |
|---|---|---|---|---|
| 读文件 | `read` | `read` | `read` | `fs/read_text_file`（client 侧） |
| 写文件 | `write` | `write` | `write` | `fs/write_text_file` |
| 编辑 | `edit`（oldText/newText） | `edit`（old/new） | `edit`（old/new） | —（diff 渲染） |
| patch | — | `patch`（diff 字符串） | `apply_patch`（多文件） | — |
| 命令 | `bash` | `shell`（可后台） | `bash`（可后台转 job） | `terminal/*`（client 侧 PTY） |
| 搜索 | `grep`+`find`(glob)+`ls` | `grep`+`glob` | `grep`+`glob` | — |
| web | — | `websearch`+`webfetch` | `web_search`+`web_fetch` | — |
| todo | — | `todo` | `todo_write` | — |
| **subagent** | 无（最接近是 code-mode） | `subagent`（旧名 `task`，后台子会话） | `subagent`+`list_agents`/`send_message`/`wait_agent`/`interrupt_agent`（树控制） | subagents RFD |
| 提问用户 | — | `question` | `ask_user_question` | `elicitation/create`（form/url） |
| skill | code-mode extension | `skill` | `skill` | — |
| plan | — | — | `plan`/`exit_plan_mode` | `plan`（beta flag） |
| lsp | — | — | `lsp`（goToDefinition 等） | LSP（Zed 核心） |
| 图片 | `read`（图片附件） | `image_generate`（opt-in） | `read_image` | — |
| Code Mode | `codemode`（受限 JS 跑工具） | `execute`（JS 调 tools） | `run_code`（PTC，TS 程序调工具） | — |

**三个 agent 的共同最小工具集（你 v0 照抄）**：
```
read / write / edit / bash / grep / glob
```
这是所有 coding agent 的地基，6 个工具足够跑通 agent loop。差异都在「往上叠」的部分：web、todo、subagent、plan、lsp、skill。

**关键结论**：
1. **读/写/编辑/命令/搜索是 6 件套**，四个 agent 全有，无分歧。
2. **subagent 是 2026 年 coding agent 的分水岭**：dsh 最完整（树控制 + 4 个控制工具 + provider 可插），opencode 次之（后台子会话 + 深度限制），pi 没有（用 code-mode 绕过），Zed 靠 RFD。你产品若要「主 agent 派子 agent 并行」才需要它（对应 TanStack `ai-harness`）。
3. **Code Mode（受限脚本内编排工具）三家都有**（pi `codemode`、opencode `execute`、dsh `run_code`）——本质是「让 LLM 写段代码来循环调工具」，对应 TanStack `ai-code-mode`。这是比 subagent 轻的编排手段。
4. **工具都做成可插拔**：三家都不是写死，而是「内置 = 默认插件，扩展可注册」。**你要学这个机制**——工具注册中心（opencode 的 `Tool.Service` / dsh 的 `ToolsService.register` / pi 的 `registerTool()`），让工具可加可换。

## 2. 工具权限/审批模型（三家各不同，值得你选）

| 模型 | Pi | OpenCode | dsh |
|---|---|---|---|
| 机制 | 事件钩子拦截（`tool_call` 事件可 `block`），默认全放行 | **allow/ask/deny 规则引擎**（`permission.assert`，按 action+resource） | **固定 pipeline**（pre hooks → approval → guards → execute → post hooks） |
| 安全边界 | 外部 sandbox（Docker/Gondolin） | 组织级 policy + 外部目录审批 | 外部 sandbox（`sandbox/sandbox-policy`）+ 升级字段需 approval |
| 特点 | 最简，靠容器兜底 | 最灵活，规则可叠加 | 最严格，审批是一次性的 |

**你的产品**（coding agent，给 Zed 驱动）：
- Zed 侧已有自己的审批 UI（`session/request_permission`：allow_once/allow_always/reject），你的 agent 只需把审批**委托给 ACP**（报 `tool_call` + 等 `request_permission` 结果），**不用自己做 allow/deny 引擎**。
- 但本地 CLI 模式（无 Zed）时，建议学 **opencode 的 allow/ask/deny 规则引擎**（最实用，规则可配置），比 pi 的纯事件拦截（默认全放行）安全。
- 即：**审批模型 = ACP 侧委托给 client，本地侧用 allow/ask/deny 规则 + 外部 sandbox 兜底**（dsh 的 pipeline 顺序是好的参考：审批在 hooks 后、guards 前、execute 前一次性做完）。

## 3. 工具定义/注册机制（你库该怎么设计）

三家共同点 → 你库该抄的接口：

```ts
// 一个工具就是一个注册项（opencode/dsh/pi 都长这样）
ctx.tools.register({
  name: 'read',                 // 给 LLM 的 name
  description: '...',
  parameters: <JSONSchema>,     // input
  output: { schema, render },   // output canonical schema + 给模型看的投影
  execute: async (input, ctx) => ...
})
```

- **registry 按作用域**（opencode 的 `Location` 级 / dsh 的 session 级）：工具列表可变，新增触发 reload 而非重注册。
- **MCP 统一入口**：三家 MCP 工具都扁平化成 `mcp__<server>__<tool>`（dsh 超 64 字符加 hash）。你库若接 MCP（对应 TanStack `ai-mcp`），照这个命名。
- **subagent 是「后台子会话」**（opencode：立即返回 `sessionID`，子会话 idle 时通知父；dsh：树 + 4 控制工具）。你若要，学 opencode 的「立即返回 + 异步通知」模型，比同步阻塞好。

## 4. agent 性能：该盯哪些维度 + 榜单地图

**没有单一权威榜**。按你关心的维度分开（数据需 web 恢复后拉最新，下面给方法论 + 各榜定位）：

| 维度 | 盯哪个榜 | 测什么 | 对你选型意义 |
|---|---|---|---|
| **质量（修 bug）** | **SWE-bench Verified / Pro** | 给 issue 改代码通过测试 | 最主流 coding 质量榜，看你要接的 LLM 模型得分 |
| **质量（终端 agentic）** | **Terminal-Bench** | 终端里多步任务（跑命令/改文件） | **最像你 agent 实际干的活**，比 SWE-bench 贴近 |
| **质量（长程多工具）** | **GAIA / AgentBench / LiveBench Agent** | 长程多工具 agent 任务 | 测「agent 本身」不只模型 |
| **速度** | 厂商 **TTFT + 吞吐（token/s）**（vLLM/SGLang 压测） | 首 token 延迟 + 长任务时长 | 跟着 serving 走，影响交互体验 |
| **成本** | **$/task**（榜上 task 数 × 单价折算） | 解一道 SWE/terminal task 花多少钱 | **性价比 = 质量/成本**，最贴「做产品要省钱」 |

**实操只盯 3 个**：
1. **SWE-bench Verified**（修 bug 质量，主流）
2. **Terminal-Bench**（终端 agentic，最像你的 agent）
3. **$/task 性价比**（同质量谁便宜）

**当前格局（知识截止，需拉最新数据确认）**：
- 质量：Claude 系（Claude 4.x 系）在 SWE-bench Verified 领先；GPT 系次之；DeepSeek V3/R1 系在**成本**上最有竞争力（大参数 + 开源 + 便宜）
- 你的 agent 选型 LLM 时：**质量敏感（修复杂 bug）选 Claude/GPT；成本敏感（大量任务）选 DeepSeek 系**
- 速度：主要看 serving 实现（vLLM/SGLang/厂商 API），不是 agent 框架本身

> ⚠️ 上面模型排名是调研日前的认知，**动手前务必拉一次 SWE-bench Verified + Terminal-Bench 最新榜**（web 恢复后补真实数据）。agent 质量瓶颈在「模型 + 工具设计 + 上下文管理」三者，单看模型榜不够——同样模型下，**工具集质量（6 件套 + subagent/code-mode）和上下文压缩（compaction）直接决定 Terminal-Bench 表现**。

## 5. 对你产品（aa-agent / aacode）的直接结论

1. **v0 工具集 = 6 件套**：`read / write / edit / bash / grep / glob`。够跑 agent loop，和 pi/opencode/dsh 的最小公约数一致。
2. **工具做成可插拔 registry**（学 opencode `Tool.Service` / dsh `ToolsService`），别写死。
3. **审批**：ACP 侧委托 client（`request_permission`），本地侧学 opencode 的 allow/ask/deny 规则 + 外部 sandbox 兜底。
4. **编排手段两个梯度**：轻 = Code Mode（受限脚本调工具，三家都有）；重 = subagent（后台子会话 + 树控制，dsh 最完整）。你产品要「主 agent 派子 agent」才上 subagent。
5. **LLM 选型**：质量敏感选 Claude/GPT，成本敏感选 DeepSeek 系；动手前拉 SWE-bench Verified + Terminal-Bench 最新数据。
6. **上下文压缩（compaction）**：长任务的 Terminal-Bench 表现关键，对应 TanStack `ai-compaction`，v1+ 要接。
