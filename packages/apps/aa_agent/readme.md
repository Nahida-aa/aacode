# aa_agent

独立运行的 agent 二进制，与 `aacode`（GPUI 桌面）/ `cli`（行模式/TUI）/
`remote_server`（SSE/HTTP 服务端）并列。

## 定位

未来可能考虑开发。目标形态暂定：

- **无 UI 的 agent runtime**：从 stdin / socket / MCP transport 接收任务，
  调用 `session::run_turn()` 执行，输出结构化结果。
- **daemon 模式**：长期驻留，管理多 session、tool call 权限、通知分发，
  可被 `aacode` / 外部客户端通过 unix socket 或 HTTP 调用。
- **MCP server**：直接把 agent capability 暴露为 MCP tools，供
  其他 agent（如 opencode）编排调用。

## 现状

占位 crate，`[dependencies]` 为空，`src/` 下仅有模板文件。
在 workspace 里不承担任何构建职责，不阻塞主线编译。
