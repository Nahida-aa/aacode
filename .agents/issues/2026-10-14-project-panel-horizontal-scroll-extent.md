# 项目面板：横向滚动范围由「估算出的最宽项」决定，偶尔不够宽

|                  |                                                    |
| ---------------- | -------------------------------------------------- |
| 状态             | 🟡 未修（影响小，暂不处理）                        |
| 发现             | 2026-10-14                                         |
| 影响范围         | 项目面板横向滚动；默认开启                         |
| 是否上游缺陷     | 是（与 zed `96a1b1b` 逐行一致，见 §3）             |
| 上游 issue       | 未提交                                             |
| 负责人           | —                                                  |

## 1. 现象

- 项目面板开启横向滚动时（`"project_panel.scrollbar.horizontal_scroll": true`，**默认值**），滚到最右侧，仍有部分条目（尤其文件夹）名字露不全 / 超出面板边界。
- **收起若干目录后，算出的宽度变宽，就能看全了**（实测）。这一条是定位的关键线索：可见条目集变了 → 估算选中的候选变了 → 测量出的宽度变了。

## 2. 先排除：不是 ui crate 未对齐

最初怀疑 gpui_learn 的 `packages/ui` 与 zed `crates/ui` 漂移。排查方式：列出 `project_panel` 的 ui 导入，逐个与 zed 对比（过滤 `use` / 文档噪声后看语义差异）：

| ui 文件 | 语义差异 |
|---------|---------|
| `list/list_item.rs` | 0 |
| `context_menu.rs`、`tooltip.rs` | 0 |
| `sticky_items.rs`、`indent_guides.rs` | 0（文件头注明「照搬，原样」） |
| `indicator.rs`、`decorated_icon.rs`、`icon_decoration.rs`、`project_empty_state.rs` | 仅模块文档 / `Icon` 类型私有字段访问适配 |
| `label.rs` | 2 处：缺 `text_center()`（zed #65210）、buffer 字体改走 provider（fork 适配） |
| `scrollbar.rs` | **落后**：缺 zed #64978 的轨道圆角（`inner_corner_radii` / `track_corner_radii`） |

两处实质差异都是**纯外观**，与文字溢出无关。结论：ui 侧对齐，此 Bug 与 ui 无关。

## 3. 真因：只测量「估算选出的那一项」

`packages/project_panel/src/lib.rs` 与 zed `96a1b1b` **逐行一致**：

```rust
// lib.rs:7709-7711（AAgent） / project_panel.rs:7729-7731（zed 96a1b1b）
.when(horizontal_scroll, |list| {
    list.with_width_from_item(self.state.max_width_item_index)
})
```

gpui 侧（`crates/gpui/src/elements/uniform_list.rs:661-683`）：

```rust
fn measure_item(&self, list_width: Option<Pixels>, ...) -> Size<Pixels> {
    let item_ix = cmp::min(self.item_to_measure_index, self.item_count - 1);
    let mut items = (self.render_items)(item_ix..item_ix + 1, window, cx);  // 只渲染这一项
    ...
    item_to_measure.layout_as_root(available_space, window, cx)            // 量它的宽度当列表宽度
}
```

即：**横向滚动范围 = 被选中那一项的真实渲染宽度**。而「选谁」靠估算：

```rust
// lib.rs:4710
let chars = path.as_unix_str().chars().count();
// lib.rs:7275
fn item_width_estimate(depth: usize, item_text_chars: usize, is_symlink: bool) -> usize {
    const ICON_SIZE_FACTOR: usize = 2;
    depth * ICON_SIZE_FACTOR + item_text_chars + if is_symlink { 2 } else { 0 }
}
```

**为什么只能测一项**：候选是在 `cx.spawn_in(window, async move …)` 的后台任务里算的（`lib.rs:4538`），那里没有 `Window`、拿不到文本系统；而 `uniform_list` 是虚拟化的，只有可见项会被布局，无法知道全部条目的真实最大宽度。所以 zed 的取舍是「便宜估算挑候选 → gpui 只真实测那一项」。

**失效点**：估算只用于**排名**。一旦排名与真实渲染宽度不一致，extent 就取到次宽项，缺口 = 两项宽度之差。不一致的来源：

- 比例字形：`iiii` 与 `MMMM` 字符数相同、渲染宽度差很多
- CJK / 全角：记 1，渲染约 2 列宽（**中文路径下最明显**）
- `bold_folder_labels`（默认 false）开启时文件夹是 `SEMIBOLD`，更宽
- 图标、git 状态装饰等未计入估算

## 4. 影响评估

小。`horizontal_scroll` 默认 true，所以路径是默认走的；但只有估算排名错位时才暴露，且缺口仅为两项宽度之差。实测：纯英文项目基本无感；中文路径更明显。收起目录后变准，正是因为候选集变化后选中了更宽的项。

## 5. 可选修法（均未实施）

| # | 方案 | 改动 | 成本 | 备注 |
|---|------|------|------|------|
| 1 | 估算改用显示宽度 | `chars().count()` → `unicode_width::UnicodeWidthStr::width()`（`unicode-width` AAgent 已有，root `Cargo.toml:672`，editor 在用），`project_panel/Cargo.toml` 加 `unicode-width.workspace = true` | 低 | 对上游的主动偏离，需记入 `.agents/zed-modifications.md` |
| 2 | 给 semibold 文件夹加权 | 估算里按 `bold_folder_labels` 给目录项加权重 | 低（调参） | 只在开启该设置时有意义 |
| 3 | 关掉横向滚动 | `"horizontal_scroll": false` → 走 `FitList` + `overflow_hidden` 裁剪 | 零 | 代价：长名看不全，与「能横向滚动」的初衷相反 |
| 4 | gpui 记录渲染中实测的最大宽度 | 动 `uniform_list` | 高 | 最彻底；但 gpui 是 pin 的上游 rev（`c019950`），改它成本与风险都大 |

## 6. 是否向上游提 issue

- AAgent 与 zed `96a1b1b`（v1.23.3）这段逻辑**逐行一致**，且 AAgent 的 gpui 直接 pin 上游 `c019950`，所以 **zed 同样受影响**，不是移植引入的。
- zed 本地 HEAD `26363aab`（2026-10-12）与 pin `c019950`（2026-10-07）之间，`project_panel` / `ui` 没有相关修复（只有 #65210 `text_center` 与 #64978 滚动条圆角）。
- 若要提：先在 96a1b1b 上复现（默认设置 → 项目面板滚到最右 → 长名露不全），再提；指向 `item_width_estimate` 的 `chars().count()`，建议换 `UnicodeWidthStr::width`（zed 自己在 `crates/editor/src/split.rs:1679` 就在用）。
- 当前状态：**未提交**——结论是「架构取舍 + 度量简化」，影响小，先记录。

## 7. 附：已排除的另一解释

曾怀疑粘性条目：`sticky_items.rs:184` 的 `expanded_width = bounds.size.width + scroll_offset.x.abs()`（sticky 行按「视口宽 + 横向滚动量」布局，而 208 行绘制时只减 y、不减 x，即钉在左边缘）。该机制会让长粘性行拿到充裕宽度而**不被裁剪**，理论上也能画出面板外，且 sticky 条目正是父目录（`sticky_scroll` 默认 true）。

但用户实测「收起目录后算出的宽度变宽 → 能看全」，更符合 §3 的候选估算机制（可见条目集变化 → 候选变化 → 测量宽度变化），故以 §3 为准。若日后要复核，可用 `"project_panel": { "sticky_scroll": false }` 开关做对照。
