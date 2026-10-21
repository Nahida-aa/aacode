# #65655 fail-fast 变体 review

- 目标：`Nahida-aa/zed` 分支 `fix/65655-openai-tool-call-empty-name-or-id`（commit `ac237f81f8`），
  范围 `language_model_core`（mapper + 错误枚举）+ `agent`（重试策略），UI 改动不在本 review 范围。
- 配套材料：`issues/65655.md`（给上游 PR #65859 的评论草稿，立场一致：fail-fast 是选项之一，不主张合成）。
- review 立场：不采信被审代码自带注释的自证（注释可能有误），逐条对照行为验证。

## 结论

修法机制正确，核心语义（guard 位置、fail-fast、兄弟保留、Stop 抑制）都站得住。
留 2 个开放设计决策（合成 id / 自动重试）到 review 会上裁决，另有 3 处注释/warning 级问题需修。
**不阻塞合并。**

## A. 验证过正确的部分

1. **guard 位置无洞**。缺 id 在流式过程中是合法中间态（首个 delta 才带 id，后续只有 arguments
   碎片），只有 `finish_reason == "tool_calls"` 的 drain 阶段才产生判定。逐 delta 的 L517
   （partial 发射前查 id+name 非空）与 drain 处的终检一一对应，没有第三种缝隙。
2. **fail-fast 优于合成 id**。`ToolCallAccumulator` 的文档已证明 provider 会 reuse index /
   repeat id（#42584），`call_{index}` 合成存在同消息内撞车；且合成会把伪造身份持久化进
   历史，后续 replay 时与 provider 真实下发的 id 冲突。拒绝接受 = 坏 call 不进 `ToolUse`
   事件 = 不持久化 = 下轮请求历史干净。这条因果链是本地可验证的（不依赖服务端行为）。
3. **兄弟保留 + 整响应失败**粒度自洽。assistant 消息声称 N 个调用、实际只回 N-1 个结果，
   本身就是不一致历史；"能救一半"看起来更友好，但把"半成品轮"喂回模型的风险更高。
4. **Stop 抑制正确**（drain 失败路径 `return events` 跳过 `Stop(ToolUse)`）。不抑制的话
   thread 会把该轮判成正常结束，错误事件变成孤儿。
5. **错误分层正确**。`DeserializeResponse` 管 wire 形状（有 `#[source] serde_json::Error`
   可挂）；"读得懂但语义坏"没有 serde 错误可挂，必须独立 variant。`is_transient()`
   已把 `InvalidProviderOutput` 归入不可重试分类，与其他变体一致。
6. **测试覆盖核心语义**：`issue_65655_rejects_tool_calls_with_empty_id_or_name_on_finish`
   （仅缺 id / 仅缺 name / 双缺 / 正常四例）+ `..._keeps_well_formed_tool_calls_alongside_rejected_ones`
   （兄弟保留断言）。
7. **agent 侧无副作用**：`retry_strategy_for` 独立分支；UI 侧 `ThreadError::InvalidProviderOutput`
   是专用 variant → 专属 callout 标题 "Invalid Provider Response"（区别于 "Request Failed"），
   且有 retry 按钮（`show_retry=true && thread.can_retry`，session 级门控）——手动恢复路径存在。

## B. 开放设计决策（review 会裁决，各列利弊）

### D1：自动合成 id 还是 fail-fast

| | fail-fast（现状） | 合成 id |
|---|---|---|
| 轮次 | 整轮失败，用户手动重试/换模型 | 轮次可续 |
| 数据 | 无伪造 | 伪造 id 持久化进历史，撞车风险（index 复用） |
| 适用 | 弱模型普遍场景 | 仅"provider 本身无 tool cell id 概念"才真正需要 |

倾向：**维持 fail-fast**。若未来要支持无 id 协议，单独做协议层合成（用 drain 位置编号
而非 `call_{index}`），不要混进这个 guard。

### D2：自动重试（现状 `InvalidProviderOutput => None`）

- 支持 `None`：主因是确定性的（模型产不出结构），自动重试烧配额且静默；
- 代价：分不清"代理丢了首个 delta 块"这类瞬态成因——一次重试本可救活；
- 缓解：UI 的 retry 按钮覆盖手动恢复，且手动重试重发的是干净历史，模型重新生成，
  对瞬态成因同样有效。

倾向：**维持 `None`**。若要兜底，`FixedDelay { max_attempts: 1 }` 是最小代价，两者都可辩护，
会上定。

## C. 需修正的问题（注释/warning 级，不改行为）

1. **"every subsequent request" 夸大**（mapper drain guard 注释）：空 id 的真实后果精确表述是
   "下一次 replay 这对 tool_call/tool_result 的请求会被服务端拒"，不是此后所有请求。
   后续全新对话不受影响。建议改为 "the next request that replays this tool call is rejected"。
2. **fork 注释漂移**：fork 版写 "drop the call, and fail the response"，zed 原版写
   "keep the well-formed siblings, and fail the response"。行为一致（兄弟保留并 emit），
   但 "drop" 措辞与行为拧着，同步回原版。
3. **log 标签借词**：`log::error!` 里 `raw_input:` 打印的是本地字段 `arguments`——
   dev-facing 日志应用本地字段名，`raw_input` 词汇只属于事件边界（`LanguageModelToolUse`）。
   aa-stack 副本已改，检查 gpui_learn / 本 fork 副本是否同样对齐。

## D. 更深的缺口（后续 issue，不阻塞本次）

**兄弟副作用 vs 结果回传的错位**：N-1 个正常 call 在事件到达时即被 `run_tool` 执行
（副作用已落地：文件改了一半等），随后 turn 死于 `InvalidProviderOutput`，这些工具的
`ToolResult` 不会喂回模型（下一轮 completion 请求不发生）。UI 呈现为"工具都跑完了
+ 一条红错误"，用户容易误判"模型已消化这些结果"。两个方向二选一带上会讨论：

- callout 文案明示"本轮工具结果未回传给模型"；
- 或 malformed 出现时兄弟 call 也转 pending 不执行（副作用一致性换"多执行"的假象）。

注：选项二会同时改变"能救则救"的现有行为，需 agent 团队定夺。
