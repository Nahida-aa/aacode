//! Pane toolbar 初始化 —— 对齐 Zed `crates/zed/src/zed.rs` 的 `initialize_pane`。
//!
//! Zed 把 tab 下方那一整栏（路径导航、搜索框、诊断/日志入口、diff 工具条…）
//! 逐个 `toolbar.add_item(...)` 挂到**每个 pane** 上。aacode 此前完全没有这段，
//! 于是 `packages/breadcrumbs` 等 crate 都在仓里、editor 也已设好
//! `RenderBreadcrumbText` 全局（`editor/src/lib.rs:362`，与 zed 同号），
//! 但永远显示不出来 —— 不报编译错、不 panic，纯静默缺失。
//!
//! 挂载点对齐 Zed：`observe_new::<Workspace>` 里对 `active_pane` 执行一次，
//! 再订阅 `Event::PaneAdded` 给后续新增的 pane 补上（分屏/新窗口用）。
//!
//! 注意与同目录 `panels.rs` 的 `initialize_panels`（**复数**）区分：那是 Zed
//! `zed.rs:777` 的左侧 Dock Panel（project / git / agent 面板），本文件对应的是
//! `zed.rs:1457` 的 `initialize_pane`（**单数**），负责 tab 下方那栏 toolbar item。
//! Zed 两者只差一个 s，本 fork 拆成文件后特意把本文件命名为 `pane_toolbar`
//! 以免与 `panels.rs` 混淆。

use breadcrumbs::Breadcrumbs;
use diagnostics::ToolbarControls as DiagnosticEditorControls;
// `AppContext` 必须引入：`cx.new()` 是它的方法，漏掉会报
// 「no method named `new` found for Context<Toolbar>」—— 与 zed.rs:43 的 import 对齐。
use gpui::{App, AppContext as _, Context, Entity, Window};
use image_viewer::ImageViewToolbarControls;
use language_tools::{
    HighlightsTreeToolbarItemView, SyntaxTreeToolbarItemView, lsp_log_view::LspLogToolbarItemView,
};
use onboarding::multibuffer_hint::MultibufferHint;
use search::{BufferSearchBar, project_search::ProjectSearchBar};
use workspace::{Event, Pane, Workspace};

/// 注册「每个 pane 都挂上 toolbar item」的 observe_new。
///
/// 对齐 Zed `crates/zed/src/zed.rs:550-563`。Zed 那边这段是内联在
/// `observe_new::<Workspace>` 里的，没有独立包装函数；本 fork 拆到独立模块，
/// 故起名 `initialize_pane_toolbars`（区别于同模块的 `initialize_pane`，后者是
/// Zed 原名、逐字保留）。
pub fn initialize_pane_toolbars(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, window, cx| {
        let Some(window) = window else {
            return;
        };

        let workspace_handle = cx.entity();
        initialize_pane(workspace, &workspace.active_pane().clone(), window, cx);

        cx.subscribe_in(&workspace_handle, window, {
            move |workspace, _, event, window, cx| match event {
                Event::PaneAdded(pane) => {
                    initialize_pane(workspace, pane, window, cx);
                }
                _ => {}
            }
        })
        .detach();
    })
    .detach();
}

/// 对齐 Zed `crates/zed/src/zed.rs:1457` 的 `initialize_pane`。
///
/// 与上游的差异：跳过 4 个 aacode 没有实现的 item，各自的缺失原因见下方注释。
fn initialize_pane(
    workspace: &Workspace,
    pane: &Entity<Pane>,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    pane.update(cx, |pane, cx| {
        pane.toolbar().update(cx, |toolbar, cx| {
            let multibuffer_hint = cx.new(|_| MultibufferHint::new());
            toolbar.add_item(multibuffer_hint, window, cx);
            let solo_diff_style_toolbar = cx.new(|cx| {
                git_ui::solo_diff_view::SoloDiffStyleToolbar::new(cx)
            });
            toolbar.add_item(solo_diff_style_toolbar, window, cx);
            let file_diff_style_toolbar = cx.new(|cx| {
                git_ui_core::file_diff_view::FileDiffStyleToolbar::new(cx)
            });
            toolbar.add_item(file_diff_style_toolbar, window, cx);
            // 路径导航栏 —— 用户报「tab 下方不显示路径」就是缺这一项。
            let breadcrumbs = cx.new(|_| Breadcrumbs::new());
            toolbar.add_item(breadcrumbs, window, cx);
            let buffer_search_bar = cx.new(|cx| {
                BufferSearchBar::new(
                    Some(workspace.project().read(cx).languages().clone()),
                    window,
                    cx,
                )
            });
            toolbar.add_item(buffer_search_bar, window, cx);
            // 上游紧接着是
            //   let quick_action_bar = cx.new(|cx| QuickActionBar::new(buffer_search_bar, workspace, cx));
            // 但 QuickActionBar 是 zed crate 内的组件（crates/zed/src/zed/quick_action_bar.rs，
            // 连同 preview.rs 共 957 行），aacode 无对应 crate。BufferSearchBar 本身不依赖它
            // （zed 只是把 Entity 传进去做包装），故此处只加裸的 BufferSearchBar。
            let diagnostic_editor_controls = cx.new(|_| DiagnosticEditorControls::new());
            toolbar.add_item(diagnostic_editor_controls, window, cx);
            let project_search_bar = cx.new(|_| ProjectSearchBar::new());
            toolbar.add_item(project_search_bar, window, cx);
            let lsp_log_item = cx.new(|_| LspLogToolbarItemView::new());
            toolbar.add_item(lsp_log_item, window, cx);
            let dap_log_item = cx.new(|_| debugger_tools::DapLogToolbarItemView::new());
            toolbar.add_item(dap_log_item, window, cx);
            let acp_tools_item = cx.new(|_| acp_tools::AcpToolsToolbarItemView::new());
            toolbar.add_item(acp_tools_item, window, cx);
            // 上游此处有 `telemetry_log::TelemetryLogToolbarItemView`，属 zed crate 内部模块
            // （打开遥测日志窗口的入口），aacode 无对应物，跳过。
            let syntax_tree_item = cx.new(|_| SyntaxTreeToolbarItemView::new());
            toolbar.add_item(syntax_tree_item, window, cx);
            // 上游此处有 `MigrationBanner` —— 数据库迁移期间的提示横幅，aacode 跳过。
            let highlights_tree_item = cx.new(|_| HighlightsTreeToolbarItemView::new());
            toolbar.add_item(highlights_tree_item, window, cx);
            let project_diff_toolbar = cx.new(|cx| git_ui::project_diff::ProjectDiffToolbar::new(workspace, cx));
            toolbar.add_item(project_diff_toolbar, window, cx);
            let staged_diff_toolbar = cx.new(|cx| git_ui::staged_diff::StagedDiffToolbar::new(workspace, cx));
            toolbar.add_item(staged_diff_toolbar, window, cx);
            let unstaged_diff_toolbar = cx.new(|cx| git_ui::unstaged_diff::UnstagedDiffToolbar::new(workspace, cx));
            toolbar.add_item(unstaged_diff_toolbar, window, cx);
            let branch_diff_toolbar = cx.new(git_ui::branch_diff::BranchDiffToolbar::new);
            toolbar.add_item(branch_diff_toolbar, window, cx);
            let solo_diff_git_toolbar = cx.new(git_ui::solo_diff_view::SoloDiffGitToolbar::new);
            toolbar.add_item(solo_diff_git_toolbar, window, cx);
            let commit_view_toolbar = cx.new(|_| git_ui::commit_view::CommitViewToolbar::new());
            toolbar.add_item(commit_view_toolbar, window, cx);
            let agent_diff_toolbar = cx.new(agent_ui::AgentDiffToolbar::new);
            toolbar.add_item(agent_diff_toolbar, window, cx);
            // 上游此处有 `BasedPyrightBanner`（python LSP 提示），aacode 跳过。
            let image_view_toolbar = cx.new(|_| ImageViewToolbarControls::new());
            toolbar.add_item(image_view_toolbar, window, cx);
        })
    });
}