# 排查方法：UI「内容不显示」类 bug

> 一次真实排查的复盘。结论很朴素，但当时没做到，代价是十几轮无效测量。

## 铁律：先验数据源，再碰渲染层

函数入口打**一行**长度/计数日志，确认数据非空。这是最便宜、最可能一击命中的动作。

```
在数据源函数入口：tracing::debug!(count = items.len(), "...");
```

本次真实案例：`render_llm_providers_page` 整页内容 100% 来自
`visible_providers()`，而它因缺 init 返回空 vec → 页面是零子元素的 `v_flex` → 全白零报错。
**一行 `providers.len()` 就能 2 分钟收工**，而我先花了十几轮量 gpui 的视口高度、滚动偏移、可见区间、item 坐标。

## 症状分类：先问「到底哪个坏了」

同一个窗口里，不同区域走**完全不同的渲染路径**，不要假定是同一个 bug。

本次教训：用户报「User / AI / General / LLM Providers」，我假定四个都空，于是把 General/AI
也当排查对象追了十几轮 —— **实际那两页完全正常**（用户能看见才能点到 LLM Providers），只有子页空。

排查前先做两件事：
1. 按渲染路径给症状分类（同一 `Render` impl / 同一列表组件 / 子页函数指针 = 不同路径）
2. 直接确认"哪个真的坏了"，别自己替用户归因

## 指标全正常时，立即停止

本次每一项测量都健康：`visible_count=35`、`list_item_count=36`、视口 633px、滚动正常、
`index 0..8` 每帧正常布局。我却把"正常"读成"正常，但用户说的还没找到"，于是继续加测量。

**当所有指标都正常时，那个「正常」本身就是答案** —— 该做的是回头重读用户到底说了什么，
而不是继续往下挖。

## 埋点要覆盖三层

只测后两层是本次的失误：

| 层 | 问什么 | 手段 |
|---|---|---|
| **数据** | 内容源非空吗？ | 函数入口打 `len()` |
| **布局** | 有尺寸吗？在视口内吗？ | 视口 bounds、item 坐标、滚动偏移 |
| **绘制** | 画出来了吗？没被裁/遮挡吗？ | `bounds_for_item`、ContentMask |

只有数据层是"一行成本、极高命中率"的，优先做它。

## 陷阱：`code = debug!(...)` 在 gpui 的 ListState 回调里会 panic

在 `cx.processor(...)` 闭包**内部**调任何 `ListState` getter 会 panic：

```
panicked at crates/gpui/src/elements/list.rs: RefCell already mutably borrowed
```

原因：`ListState` 内部是 `Rc<RefCell<StateInner>>`，而 gpui 的 `list` 在 `request_layout` /
`prepaint` 持有 `borrow_mut()` 时才回调 `render_item`。

**读法**：把要读的值在 processor 闭包**之外**（render body 内）先取好存成局部变量，
闭包内只用这些快照。Zed 自己也在 `agent_ui/.../thread_view.rs` 留了同样的警告注释。

## 别用局部证据否定整条假设

中途有 worker 报过 `SettingsContent::pick` 返回 `None`（数据层问题），
我因用户说"按钮显示正常"就判为"不是这条路"。

**控件正常只否证「控件渲染有问题」，不能否证「数据为空」。**

## 无对照不下结论

我曾断言 `zed://settings/<path>` 是"死链 bug"，依据只是"aacode 没有消费方" ——
但**从没验证 Zed 是否消费**。用户澄清那只是用来复制的，不是跳转链接。

**只验证了一侧，就不能定性为 bug。** 要么补上另一侧对照，要么明确标注"待确认"。
