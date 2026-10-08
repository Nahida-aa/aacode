# debug 构建的资源内嵌：为什么 fs_embed! 会 panic

> 记录日期 2026-10-08。症状：debug 构建启动即 panic，release 正常。

## 症状

```
thread 'main' panicked at packages/languages/src/cpp.rs:5:10:
missing cpp/semantic_token_rules.json
```

同类还有 go / python / rust 三个 `semantic_token_rules.json`。**只在 debug 构建出现**，
release 一切正常。`cargo test -p aacode --lib` 全绿（36 passed），因为测试不触发
`languages::init()` 构建内置语言表。

## 根因

`util::fs_embed!`（在 gpui_learn 的 `aa_gpui_kit_util` 里）有两条分支：

| 分支 | 条件 | 行为 |
|---|---|---|
| A | 设了 `ZED_RELEASE_CHANNEL` env | `env!` 编译期取值 |
| B | 没设且无 `debug-embed` feature | `__fs_embed_get` 运行时从 `dev_repo_root() + root_relative` 读文件 |
| C | 有 `debug-embed` feature | `rust_embed` 编译期内嵌 |

zed 的 B 分支读 `crates/grammars/src`（`root_relative`），对 zed 仓库成立。
aacode 仓库里没有 `crates/` 目录，grammars 也不在 aacode 仓库内 → `get_file()` 恒返回
`None` → 调用点的 `.expect()` 直接 panic。

## 真正的解法：让 grammars 解析到带 debug-embed 的 util

`aa_gpui_kit_util` 的 features 段有 `default = ["debug-embed"]`，走 C 分支就与仓库布局无关。

但**光有 `default` 不够** —— 要看 grammars 的 `util::` 解析到哪个 crate：

- grammars 作为 **zed git 依赖** → 解析到 zed 自己的 util，**没有** `default = ["debug-embed"]` → panic
- grammars 指向 **gpui_learn**（`aa_gpui_kit_grammars`）→ `util.workspace = true` 解析到 `aa_gpui_kit_util` → 正常

所以关键不是给 aacode 的 util 开 feature（那只是权宜之计，图里仍有两套 util），
而是让 grammars 本身也来自 gpui_learn。

## 踩过的坑

**1. 曾经修过一次，后来回归**

`1e5e93a`（2026-09-29）就修过这个 panic，做法是
`[patch."…zed-industries/zed"] util = { path = "packages/util" }`。
`090b7a3` 把 util 搬去 gpui_learn 时**顺手删掉了那条 patch**，panic 就回来了。
教训：改 `[patch]` 段时别只顾着改依赖声明，patch 本身失效不会有编译错误，
只表现为运行时 panic。

**2. `[patch]` 对 git 包内部的 path 依赖无效**

zed 的 `util = { path = "crates/util" }` 是 git 包内的 path 依赖。往
`[patch."https://github.com/zed-industries/zed"]` 里加 `util = { path = ... }`
时 cargo 报 `patch ... was not used in the crate graph` —— 该 patch 够不到它。
这也是为什么最终选择 fork grammars 而不是 patch util。

**3. 两套 util 会导致 E0308**

`util` 导出 `Range` / `Cow` / `HashMap` 等类型。若 aacode 自己用 git rev 版、
gpui_learn 的 crate 用本地 path 版，两者 source 不同 ⇒ 编译出两个 crate ⇒
跨 crate 混用即 `E0308 multiple different versions`。

解法是在 `[patch."https://github.com/Nahida-aa/gpui_learn.git"]` 里补
`aa_gpui_kit_util` / `aa_gpui_kit_util_macros` 的 path 条目，让 aacode 自己声明的
git 依赖也落到同一路径。该段里 `ui` / `ui_input` / `component` 本来就这么处理，
`util` 之前漏了。

验证：

```bash
cargo metadata --format-version 1 | python3 -c "
import json,sys
m=json.load(sys.stdin)
seen={}
for p in m['packages']:
    if p['name'] in ('aa_gpui_kit_util','aa_gpui_kit_util_macros'):
        seen.setdefault(p['name'],[]).append(p.get('source') or 'LOCAL')
for k,v in seen.items(): print(k, len(v), v)"
```

各应为 1 份 LOCAL。

## 仍未消除

zed 的 `util` 仍在图里，被 `http_client` / `sqlez` 拉。查过这两个 crate 都不用
`fs_embed!`，只当普通工具库用，所以对本问题无影响。要彻底消除需再 fork 这两个
（`sqlez` 是数据库层，代价较大），暂缓。
