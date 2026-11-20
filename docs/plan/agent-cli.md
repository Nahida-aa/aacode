# 自研 Agent：先 CLI 基本功能，再叠 ACP（计划）

> 日期：2026-11-12
> 背景：修正「太偏向只为 ACP」的思路。先弄清一个 CLI agent 该有什么，ACP 作为输出协议之一后置。
> 代码放 `/home/aa/repos/ide_ls/aa-agent`，库放 `/home/aa/repos/lib_ls/aa-ai`。只支持 ACP v2。

## 1. 核心判断

**先做有基本功能的 CLI agent，ACP 是它的一个可插拔输出层，不是前提。**

ACP 解决的是「agent 怎么和编辑器（Zed 等）对话」。但 agent 自己得先能跑：收消息 → 调 LLM → 调工具 → 出结果。把这个核心循环做稳、用 CLI 验证完，ACP 就只剩「把核心循环的事件翻成 ACP `session/update`，从 ACP `session/prompt` 收输入 + stdio transport」一层的事。

分层（ACP 在最外层，可独立开关）：

```
[输入/输出层]  CLI(TUI/line) │ NDJSON │ ACP(v2, stdio)   ← 可插拔，ACP 最后接
        │
[核心循环]   agent loop：收 → LLM → 工具 → 回填 → … → stop
        │
[LLM 层]    provider 接口 + stub(假流式) / 真模型        ← Zed 的 stream_completion ≈ 这个
[工具层]    读/写文件、跑命令、搜索（带 schema 校验）       ← agent 自己执行，报给 client
[会话/状态]  多 turn 上下文、持久化、（后期）压缩
```

## 2. 一个 CLI agent 的基本功能（v0 → v2）

### v0 最小闭环（5 件，先做这些）
1. **agent loop**：user 消息 → LLM →（可选）工具调用 → 结果回填 → 继续 LLM → 直到 stop
2. **LLM 接口**：provider 抽象 + 一个 **stub provider**（固定文本 / 假流式），不依赖真模型就能跑协议
3. **≥2 个工具**：读文件 + 写文件（或读文件 + 跑 shell 命令），带输入 schema 校验
4. **流式输出**：LLM 增量回显 stdout（不憋完再吐）
5. **取消**：SIGINT / Ctrl-C 中断当前 turn

### v1 能用
- 工具扩：目录列表、grep 搜索、跑命令
- 会话：多 turn 上下文 + 基础持久化（消息落盘）
- 错误处理：LLM 超时/限流/工具失败的**重试 + 降级**
- **权限门**：工具执行前审批（→ ACP 里就是 `session/request_permission`，提前做对 ACP 直接有用）

### v2 好用
- 长对话压缩 / 上下文管理
- 配置（model、工具白名单、env）
- 流式结构化输出、reasoning 显示
- **ACP 输出层**（此时协议风险已被 v0/v1 核心验证兜住）

## 3. 这个清单如何决定 ACP 范围

v0/v1 做的东西**几乎全是 ACP agent 必需的**：

| CLI 功能 | 对应 ACP v2 事件/方法 |
| --- | --- |
| 流式 LLM 输出 | `session/update`：`agent_message_chunk` |
| reasoning | `agent_thought_chunk` |
| 工具调用 + 结果 | `tool_call_update`（`raw_input`/`raw_output`） |
| 权限门 | `session/request_permission` |
| 取消 | `session/cancel` + `stopReason: cancelled` |
| stop | `stopReason: end_turn` |

v2 的「压缩」在 ACP 里是**可选能力**（Zed 要开 flag 才广播），不是必需。

**结论**：做 CLI 基本功能 = 逐个点亮 ACP 需要的能力。到接 ACP 时，**只剩「协议翻译 + stdio transport + `initialize`/`session/new`/`session/prompt` 三个方法」一件事**。ACP 风险被彻底隔离在最后一步。

## 4. 三阶段（修正版）

> 顺序：CLI 核心稳 → 接 ACP（在 Zed 试用）→ 补 TUI；**GUI 不做**（让 Zed 当 client）。

### 阶段 1：CLI 核心（`--print` / line 模式）

v0 内的细粒度顺序（**工具先设计后实现，loop 先行**）：

1. **设计 2 个工具的 schema（不实现）**：读文件 + 写文件（或跑命令）。定死 input/output 契约，loop 才有真实目标。
2. **实现 loop + 工具 1（读文件）+ stub LLM**：跑通 happy path。
   - **关键**：stub LLM 必须能「决定调哪个工具」，否则 loop 的工具路径没被触发，2 个工具白写。
3. **实现工具 2（写文件/命令）**：逼出异常路径（取消中途、工具失败、结果回填）。

**阶段 1 → 阶段 2 的出口判据（5 条，别凭感觉）**：

- [ ] `--print "..."` 能从 CLI 进、流式出、结束于 stopReason
- [ ] 至少 1 个工具被 agent 真正调用并回填结果（不是硬编码）
- [ ] 取消（SIGINT）能干净停住当前 turn
- [ ] stub LLM 不依赖任何真模型
- [ ] 核心循环能在不 import 任何 UI/协议模块的情况下单测

这 5 条过了就接 ACP——核心再长，多接个壳成本很低；核心没稳就接 ACP，协议 bug 会混进核心 bug，很难分。

### 阶段 2：接 ACP v2（stdio）

- `@agentclientprotocol/sdk`（v2）的 `agent()` + `ndJsonStream`，把 v0/v1 事件映射到 `session/update`，实现 `initialize`/`session/new`/`session/prompt`/`session/cancel`。
- **产物不只是"能跑在 Zed 里"**，还有一份 **「ACP 能力清单 + 实测缺什么」**——Zed 这种真实 client 会暴露终端 CLI 里想不到的问题：`agentCapabilities` 声明对不对、`tool_call` 的 `locations`/`raw_input`/`raw_output` 给全没、`session/request_permission` 的选项（allow_once/allow_always/reject）编辑器怎么吃、`loadSession`/`cancel` 在重开/切项目时的行为。
- Zed 验证：`agent_servers` 配 CLI + 开 `acp-beta`。

### 阶段 3：补 TUI（GUI 不做）

- TUI 复用阶段 1 核心 + 加交互输入层，成本低（核心已稳）。
- **GUI 不做**：agent 当库/CLI，Zed 等 editor 当 GUI。你的全景是：
  ```
  核心 agent（库）
    ├── CLI（--print / line / TUI）   ← 你做，给人直接敲
    └── ACP 壳（stdio v2）           ← 你做，给 Zed 驱动
      （GUI 由 Zed 这类 client 提供）
  ```

### 之后：Rust 生产

核心逻辑（loop/工具/持久化）抽成语言无关的接口，Rust 侧重实现，ACP 层用 `agent-client-protocol` crate 的 v2 feature。

## 5. 目录（aa-agent）

```
aa-agent/
├── ts/                          # 实验：TypeScript，v0→v2 全在这里先跑通
│   ├── package.json
│   ├── tsconfig.json
│   └── src/
│       ├── loop.ts              # agent loop（核心）
│       ├── llm.ts               # provider 接口 + stub
│       ├── tools.ts             # 工具（读/写/命令）+ schema
│       ├── session.ts           # 上下文 + 持久化（后补）
│       ├── cli.ts               # bin：--print / line / --acp
│       ├── acp.ts               # ACP v2 输出层（v2 才写）
│       └── verify.ts            # 通用验证脚本（协议层，后补）
└── rust/                       # 生产：Rust（后期，核心逻辑重实现 + ACP v2 feature）
```

## 6. 现状（2026-11-20）

计划里的 v0 已经落地成 Rust（不走 TS 实验）：`aa_agent::AgentLoop`（库）+ `aacode-cli run`（应用）。
第 5 节的 TS 目录只当作早期想法，实际代码在 `packages/apps/cli` + `../../lib_ls/aa-ai/packages/aa_agent`。

### 三种跑法

| 目的 | 命令 | 依赖 |
|---|---|---|
| 验证 loop / 工具接线 | `aacode-cli run --stub --dir <DIR> "..."` | 无（剧本驱动，不读设置、不连网） |
| 真实模型 + 工具 | `aacode-cli run --dir <DIR> <provider> <model> "..."` | 设置里有端点 + 密钥 |
| 纯对话（不带工具） | `aacode-cli chat <provider> <model> "..."` | 同上 |

密钥可以放 `.env`（当前目录或设置文件旁），不用每次 export。

### 测试用哪个端点

- **agnes**（`agnes-3.0-flash`）是首选：工具/图片/并行工具调用都声明支持。
- **minekuai** 也能跑通（`deepseek-v4.1-flash-1017`，带 `reasoning_content`）。
  注意它的鉴权：`/v1/chat/completions` **只认** `Authorization: Bearer`，
  `x-api-key` 只在 `/v1/models` 上被接受（同一个 key：models 200、chat 401）。
- **本地**：Ollama 的 `:cloud` 模型现在全部收费，不算本地。本地测就挑最小可用的模型
  （`qwen2.5:0.5b` / `llama3.2:1b`，`ollama pull` 一次），走
  `openai_compatible.ollama` → `http://localhost:11434/v1`。
- **llama.cpp 不需要单独适配器**：`llama-server` 的 `/v1` 就是 OpenAI 兼容形状
  （`/v1/chat/completions`、`/v1/models`、`/v1/responses`、甚至 Anthropic Messages）。
  配 `openai_compatible.llama.cpp` → `http://localhost:9931/v1`（新版本默认 9931，老版本 8080），
  `auth: {kind: "none"}`。

### 代理

顶层 `proxy`（和 Zed 同名）已经接线：`Settings.proxy` → `Settings::transport()`
（构造一次，`OnceLock` 缓存）→ `build_provider` 把同一个
`Arc<aa_http_client::HttpClient>` 交给每个 provider。语义照 Zed：

- 空串/缺省 = 不显式设置，reqwest 自己读 `ALL_PROXY` / `HTTPS_PROXY` / `HTTP_PROXY`；
- 非空 = 覆盖环境变量代理，但 `NO_PROXY` 的直连列表仍然生效；
- 只接受 `http://` / `https://`；`socks5://` 明确报「需要 reqwest 的 `socks` feature」；
- 解析期就构造传输：代理 URL 语法错误在**加载设置**时报，并带上文件路径
  （代理配错会让所有命令一起失败，越早报错越好）。

传输本身在 `aa-stack/packages/util/aa_http_client`（不在 aa-ai 里）：它不认识任何
LLM 类型，一个进程一个连接池，所有 provider 共享。见 aa-ai
`docs/decisions/0010-transport-object.md`。

顺带把 reqwest 从 `rustls-tls` 换成 `rustls-tls-native-roots`：本地 mitmproxy 的 CA
装在系统信任库里，webpki-roots 不认，换之前走代理会报 `invalid peer certificate: UnknownIssuer`。
实测 `proxy = "https://127.0.0.1:8080"`（mitmweb）时 `models` / `chat` / `run` 三条都通。

### 超时

`aa_http_client` 的默认客户端是 10s 建连 + 120s per-read idle 超时（其余
keep-alive 取值对齐 Zed，见 aa-ai `docs/decisions/0009-http-transport.md`）。
流式响应连续 120s 收不到任何字节会报
`读取超时：连续 120 秒没有收到任何数据 ...`，不再无限挂起；健康的长流只要持续
有增量就不会被中断。CLI 暂不暴露这个值。
