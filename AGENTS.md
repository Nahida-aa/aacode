<!-- intent-skills:start -->

## Skill Loading

Before editing files for a substantial task:

- Run `bunx @tanstack/intent@latest list` from the workspace root to see available local skills.
- If a listed skill matches the task, run `bunx @tanstack/intent@latest load <package>#<skill>` before changing files.
- Use the loaded `SKILL.md` guidance while making the change.
- Monorepos: when working across packages, run the skill check from the workspace root and prefer the local skill for the package being changed.
- Multiple matches: prefer the most specific local skill for the package or concern you are changing; load additional skills only when the task spans multiple packages or concerns.

<!-- intent-skills:end -->

- 修改代码后, 如果认为适合提交, 就自行提交
- 禁止删除或手动修改 `Cargo.lock`
## Debug

- 使用 tracing

## Code Navigation

Use `ast-grep outline` before reading full source files when exploring code.

- Use `ast-grep outline <file>` to inspect a candidate file.
- Use `ast-grep outline <dir> --items exports` to find public entry points.

After finding candidate files, use `ast-grep outline` to decide which source
range to read. If your agent supports skills, use the `ast-grep-outline`
skill for more detailed guidance.

## ⚠️ 主线声明

**GPUI 桌面（packages/app + packages/workspace + packages/ui）是唯一主线。**

以下前端方案是历史遗留或实验性的，**不保证能编译、能运行或与主线兼容**：

- `packages/cli/` — Ratatui TUI（旧 CLI 入口）
- `packages/tui/` — OpenTUI + Solid.js TUI（Bun）
- `packages/webui/` — Solid.js Web 前端（TanStack Router）
- `packages/ui-solid/` — Kobalte UI 组件库
- `packages/shared/` — TS 共享 lib

`cargo check --workspace` 只要求主线相关包通过（`kernel`、`core`、`llm`、`extensions`、`extension-sdk`、`extension-mcp`、`function-tools`、`config`、`session`、`server`、`app`、`workspace`、`ui`）。其他包即使编译失败也不阻塞提交。

---

所有代码统一放在 `packages/` 下，不分 Rust/TS：


**非主线（仅参考/历史，不保证编译）：**

- `packages/cli/` 旧 CLI 入口（`aa run` 行模式、Ratatui TUI）
- `packages/tui/` OpenTUI + Solid.js TUI（独立 Bun 进程）
- `packages/webui/` Solid.js + TanStack Router Web 前端
- `packages/ui-solid/` Kobalte 共享 UI 组件库
- `packages/shared/` TS 共享 lib（utils、i18n）
- `packages/sdk-ts/` 自动生成的 TypeScript SDK（openapi-ts）

**Rust edition**: 2024（`set_var`/`remove_var` 需 `unsafe` 块）

📁 **目录名 vs Cargo.toml name 规则**：目录名用简洁形式（如 `kernel/`），`package.name` 用 `aa-` 前缀（如 `aa-kernel`）。不要混用。

## 架构决策

- **Server 层不单独拆 C/S**，内嵌在 `packages/server/` 作为同进程调用边界
- **主线（GPUI 桌面）直接调 `session::run_turn()`**，不走网络、不走 SSE
- `packages/server/` 可选暴露 HTTP/SSE（`aa serve`）给远程客户端（非主线前端用）

## GPUI 桌面架构（主线）

- `packages/app/` — 应用入口：主窗口、标题栏（SSD/CSD 条件渲染）、Agent 面板
- `packages/workspace/` — Workspace 框架：StatusBar（left/right/hidden items）、DockSide、面板切换
- `packages/ui/` — 共享 GPUI UI 组件
- **GPUI 来源**：Zed git rev `bd747337`（pin 在根 Cargo.toml，共 6 个 `gpui*` + `collections` 等 29 个 zed crate 一起锁）
- **UI 组件来源**：`ui` = `aa_gpui_kit_ui`，从 `gpui_learn` 的 git rev 拉（`Cargo.toml` 约 536 行）→ 改 gpui rev 后要同步这个 rev
- **WindowControlArea**：`Drag` + `Close/Minimize/Maximize` 按钮布局（吸收自 aa-player）
- **服务器调用**：同进程 `session::run_turn()`，不经过 HTTP

## 非主线架构（仅供参考，不保证工作）

### 旧 CLI（Ratatui）

- `aa run` — 行模式对话
- `aa` — Ratatui TUI

### OpenTUI + Solid.js TUI

- 独立 Bun 进程，localhost HTTP/SSE 连 Rust server
- `@opentui/*` 必须 0.3.4（0.4.1 Solid context 不传播）
- JSX 通过 Babel 插件转换，需 `bunfig.toml` + `--conditions=browser`

### WebUI（Solid.js + TanStack）

- 通过 HTTP/SSE 连接 Rust server

## 已验证

- **Ollama 端到端**：`aa run --provider ollama --model gemma4:31b-cloud` 成功返回 "Hello"（462 prompt + 2 completion tokens）
- **TUI 编译通过**（无终端时 panic 属预期）
- **`Config::resolve()` 正确性**：cli 传 `Option`，不覆盖 provider 默认值（ollama→`http://localhost:11434`）
- **`RunArgs` 改为 `Option<String>`**：避免 clap default 覆盖 provider 特定默认值

纠错记录见 `.agents/CORRECTIONS.md`。

## 从 zed 搬代码

**搬 zed 的包之前，先读 `.agents/zed-port.md`。**

zed 参照仓库在 `~/repos/ide_ls/learn_ls/zed`，单文件 crate 拆分后的可见性转发、
`collections::` vs `std::collections::`、`RelPath` vs `std::path::Path`、
模块遮蔽要用 `::rpc::` 绝对路径等坑都记在那儿，照搬时直接套用，别重新推演。

## 查上游版本/release：优先用 `tools/gh-releases.ts`

问「zed 最新到哪个版本 / 某 rev 对应什么 / 某功能在哪个版本进的」时，
**不要用网页搜索**，直接跑：

```bash
bun tools/gh-releases.ts zed-industries/zed --latest      # 最新稳定版 tag + sha
bun tools/gh-releases.ts zed-industries/zed --limit 15     # 按时间倒序列表
```

它走 GitHub GraphQL 拿 tag / commit sha / publishedAt，**比网页搜索准确**：
搜索结果常是缓存的旧快照，曾据此误判「不存在 1.23」，而实际 1.23.2 已于
2026-10-07 发布。核对 rev 时还能直接对着 sha 走本地 zed 仓库。

网页搜索只用于「release notes 说了什么」「官方公告/定价」这类 GraphQL 拿不到的内容。

## 排查经验（`.agents/memory/`）

- **UI「内容不显示」类 bug**：先在数据源函数入口打一行 `len()` 确认数据非空，再碰渲染层。
  指标全正常时立即停止、回头复核用户诉求。详见 `.agents/memory/debug-ui-empty.md`。
- **Zed init 链缺口**：`init` 链缺失不报编译错、大多不 panic，只表现为功能静默失效
  （如缺 provider 注册 → 某个设置子页全白且零报错）。移植新功能时 diff 一遍上游
  `zed/src/main.rs` 的 init 链。已知缺口与排查法见 `.agents/memory/zed-init-chain-gaps.md`。
