# Agent 工具调用输入流截断：`tool input was not fully received`

- 记录：2026-10-22
- 关联会话：Zed session `328aedcb-c3af-437c-b086-8ea91aa0df49`
  （导出在 `AAgent/exports/zed_328a.json`，由 aa_session 导出）

## 现象

Zed agent 会话中工具调用**间歇性失败**，失败时工具结果只有一条错误串
`tool input was not fully received`。全会话分布（已用导出 JSON 核实）：

| 阶段 | 失败 | 特征 |
|---|---|---|
| 会话早期/中段（导出前 64%） | 16 次 terminal 失败，零散 | 与大量成功调用交替 → 间歇性 |
| 会话最后一段（导出后 ~9%） | **17 次 terminal 连发全灭**（含 `echo test`），`read_file` 同轮也开始报同一错误 | 性质从间歇恶化为持续 |

中段 `read_file` 的 4 个报错（路径不存在 / 不在项目内）是正常业务错误，
与传输故障无关，已排除。

## 错误串来源（不是终端沙箱报错）

同一字符串是 zed agent 框架自己的 `ToolInput` 错误：

- zed：`crates/agent/src/thread.rs` `ToolInput::next()` ——
  `self.rx.next().await.ok_or_else(|| anyhow!("tool input was not fully received"))`
- AAgent 移植版：`packages/agent/src/thread.rs:5098-5108` `ToolInput::recv()` 同语义
- 覆盖该语义的既有测试：
  `test_streaming_tool_completes_when_llm_stream_ends_without_final_input`
  （`packages/agent/src/tests/mod.rs:4976`）

触发条件：`ToolInputPayload` 流以 `Partial`（部分 JSON）形式到达，但流在
收到 `Full`（完整输入）之前就结束 → 框架拒绝执行该工具调用，fail-fast。

## 归属判断（⚠️ 尚未定论）

这条错误只说明"**工具调用输入流没收全**"，截断点有候选三处：

1. **LLM 提供商**：provider 侧流中途被截断 / 异常终止（最可疑）
2. **网络层**：连接中途断开、丢包
3. **zed 客户端流处理**：stream 收尾时丢了最后一段 input delta

当前证据支持"**提供商/流层面**"优先于"终端沙箱"：

- 同一轮里**多种不同工具**（terminal 大命令、`echo test` 小命令、
  read_file）报同一条框架错误 → 不是某条命令或某个工具的参数问题；
- 故障在同一轮内**持续全灭**且渐进恶化 → 更像上游流异常，
  而非本地 PTY 卡死（PTY 卡死一般不会让 read_file 跟着挂）；
- 此前会话间歇性复发的 pattern 也符合 provider 侧偶发截断。

但**不能排除客户端流处理 bug**，需要下面的区分手段。

## 区分手段（TODO）

- [ ] 查故障时段会话使用的 provider / 模型（zed 的 llm provider 设置），
      看失败是否与某个 provider 强相关；
- [ ] 看 provider 侧日志 / 响应异常终止方式（是否有 `finish_reason: length`
      或流中途断开）；
- [ ] 同一请求换另一个 provider 网关复现，做 A/B；
- [ ] 审计 zed 客户端 stream 收尾路径（各 provider 客户端把流式 delta
      转 `LanguageModelCompletionEvent` 的 flush 逻辑），确认最后一块
      input 会不会被丢。

## 结论（当前）

"提供商的问题"**是首要嫌疑但未经证实**。在按上面 4 步排除之前，
记录定性为：agent 工具调用输入流（provider → 客户端 → `Thread`）
偶发截断，fail-fast 拒绝执行工具调用；证据倾向于 provider/流层面。
