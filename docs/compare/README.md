# 对比记录：**应用层**（aacode 自己的）

> 只记事实，不做取舍。库层（IR / wire / provider 抽象 / agent loop）的对比在
> **另一个仓库**：`lib_ls/aa-ai/docs/compare/`。这里不复制那边的表，只写一行指针。

## 为什么分两处

1. **复制必然漂移。** 曾经在两边各存一份 `tanstack/acp.md`、`zed/provider-mapping.md`：
   内容相同的两份没人知道哪份是新的，分叉后更没法判断。
2. **读者是「做决定的人」。** 决定发生在哪个仓库，事实就该在哪个仓库，否则 ADR 引用的是
   别人的文件。
3. **aa-ai 是要发布的库。** 把 Claude Code / Codex 的应用层细节塞进库里，会让它的文档
   偏离「库为什么长这样」。

## 划分

| 层 | 放哪 | 典型内容 | 对照对象 |
|---|---|---|---|
| ai / agent **库** | `aa-ai:docs/compare/` | IR、wire 形状、provider 抽象、agent loop、tool trait | rig / genai / TanStack AI / Zed `language_model_core` |
| agent **应用** | 本目录 | CLI 交互、设置形状、凭证、工具集、权限/审批、会话持久化、成本统计 | Claude Code / Codex / opencode / Crush |
| **ADE** | 本目录 | ACP v1/v2、MCP、外部 agent 接入、多 agent 编排 | Zed / opencode / Crush |

### 「既是库又是应用」的项目

**一个应用如果几乎不依赖外部 AI 库**（自己实现 provider / agent loop / 工具执行），
它就应该**同时被当成库和应用**看待——例如 Zed：
`crates/language_model_core` + `crates/open_ai` 是库层，`crates/agent` + `zagent` 是应用层，
`crates/agent_servers` 是 ADE 层。

这类项目**两边都要查**：库层事实记到 `aa-ai:docs/compare/`，应用层事实记在这里，
ADE 事实也记在这里。不要因为「Zed 是个 app」就把它的 wire/IR 事实塞进应用侧。

## 索引

| 文件 | 内容 |
|---|---|
| [`agent-tools.md`](agent-tools.md) | coding agent 的工具集横向对比（Pi / opencode / dsh / Zed）+ 性能榜单该看什么 |
| [`plan/agent-cli.md`](../plan/agent-cli.md) | 自研 agent CLI 的实施计划（v0 → v2，ACP 后置） |

## 待补（还没写的对比）

- CLI 交互形态：流式输出、工具调用的展示、审批流程（Claude Code 的 `/permissions`、
  Codex 的 `approval_policy`、opencode 的 `permission`）
- 设置/凭证：多层设置文件、项目级覆盖、密钥来源
- 会话持久化：历史存哪、怎么压缩、怎么恢复
- ADE：ACP v1/v2 的差异、MCP 的接入点、多 agent 编排

写之前先确认：这件事**是不是库层的**。如果是，去 `aa-ai` 写。