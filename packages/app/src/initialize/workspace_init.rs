use crate::initialize::panels;
use gpui::{App, AppContext, Context, KeyBinding, Window};
use std::sync::Arc;
use ui::{ContextMenu, PopoverMenuHandle};

use settings::Settings as _;
use vim_mode_setting::HelixModeSetting;

/// 注册三个 observe_new：
/// - **MultiWorkspace** → `cx.defer` 里创建 Sidebar + register_sidebar
/// - **Workspace** → 创建 Dock Panel（agent/project/git 等）+ 注册 StatusBar 按钮
/// - **Pane** → 挂 tab 下方那栏 toolbar item（路径栏/搜索框/诊断日志入口等，见 pane_toolbar.rs）
pub fn initialize_workspace(app_state: Arc<workspace::AppState>, cx: &mut App) {
    // 对齐 Zed crates/zed/src/zed.rs L550 的 observe_new（含 PaneAdded 订阅）。
    // 必须在任何 pane 被创建之前注册，否则先建出来的 pane 拿不到 toolbar item。
    super::pane_toolbar::initialize_pane_toolbars(cx);

    // —— MultiWorkspace observe_new → Sidebar ——
    cx.observe_new(|multi_workspace: &mut workspace::MultiWorkspace, window, cx| {
        let Some(window) = window else {
            return;
        };

        let window_handle = window.window_handle();
        let multi_workspace_handle = cx.entity();

        cx.defer(move |cx| {
            window_handle
                .update(cx, |_, window, cx| {
                    let sidebar = cx.new(|cx| {
                        sidebar::Sidebar::new(multi_workspace_handle.clone(), window, cx)
                    });
                    multi_workspace_handle.update(cx, |mw, cx| {
                        mw.register_sidebar(sidebar, cx);
                    });
                })
                .ok();
        });
    })
    .detach();

    // —— Workspace observe_new → StatusBar 按钮 + Dock Panels ——
    let app_state_for_workspace = app_state.clone();
    let edit_prediction_menu_handle = PopoverMenuHandle::default();
    let lsp_button_menu_handle = PopoverMenuHandle::default();
    cx.observe_new({
        let edit_prediction_menu_handle = edit_prediction_menu_handle.clone();
        let lsp_button_menu_handle = lsp_button_menu_handle.clone();
        move |workspace: &mut workspace::Workspace, window, cx| {
            let Some(window) = window else {
                return;
            };

            // 对齐 Zed `crates/zed/src/zed.rs`：EditPredictionButton 的菜单由
            // `ToggleMenu` action 通过 PopoverMenuHandle 切换。
            workspace.register_action({
                let handle = edit_prediction_menu_handle.clone();
                move |_, _: &edit_prediction_ui::ToggleMenu, window, cx| {
                    handle.toggle(window, cx);
                }
            });

            // 同上：LspButton 的菜单由 `lsp_button::ToggleMenu` 切换。
            workspace.register_action({
                let handle = lsp_button_menu_handle.clone();
                move |_, _: &language_tools::lsp_button::ToggleMenu, window, cx| {
                    handle.toggle(window, cx);
                }
            });

            // 对齐 Zed `crates/zed/src/zed.rs L1024-L1041`：Window 菜单的窗口级
            // action 需要 `&mut Window`，所以在 workspace 上注册。
            workspace
                .register_action(|_, _: &crate::app_menus::Minimize, window, _| {
                    window.minimize_window();
                })
                .register_action(|_, _: &crate::app_menus::Zoom, window, _| {
                    window.zoom_window();
                })
                .register_action(|_, _: &crate::app_menus::ToggleFullScreen, window, _| {
                    window.toggle_fullscreen();
                });

            let app_state = app_state_for_workspace.clone();
            register_status_bar_items(
                workspace,
                window,
                cx,
                &app_state,
                edit_prediction_menu_handle.clone(),
                lsp_button_menu_handle.clone(),
            );

        // 对齐 Zed `crates/zed/src/zed.rs L901-L906` — 注册 AgentPanel 相关 action。
        // 放在主应用层而不是 agent_ui crate 的 init 里，避免重复注册。
        workspace
            .register_action(agent_ui::AgentPanel::toggle_focus)
            .register_action(agent_ui::AgentPanel::focus)
            .register_action(agent_ui::AgentPanel::toggle);

        let panels_task = panels::initialize_panels(window, cx);
        workspace.set_panels_task(panels_task);
    }
    })
    .detach();
}

/// 对齐 Zed `crates/zed/src/zed.rs L602-L652`。
/// 在 Workspace 创建后、initialize_panels 前，把所有非 dock 的状态栏按钮注册进 StatusBar。
///
/// 已显示项（aacode 已有 crate，顺序对齐 Zed）：
/// - 左：search_button、lsp_button、diagnostic_summary、active_file_name、git_blame_status、
///   merge_conflict_indicator、activity_indicator
/// - 右：edit_prediction_ui、active_buffer_encoding、active_buffer_language、
///   active_toolchain_language、line_ending_indicator、cursor_position、image_info、
///   vim_mode_indicator、pending_keystrokes_indicator
fn register_status_bar_items(
    workspace: &mut workspace::Workspace,
    window: &mut Window,
    cx: &mut Context<workspace::Workspace>,
    app_state: &workspace::AppState,
    edit_prediction_menu_handle: PopoverMenuHandle<ContextMenu>,
    lsp_button_menu_handle: PopoverMenuHandle<ContextMenu>,
) {
    // —— Left side ——
    let search_button = cx.new(|_| search::search_status_button::SearchButton::new());
    let lsp_button = cx.new(|cx| {
        language_tools::lsp_button::LspButton::new(workspace, lsp_button_menu_handle, window, cx)
    });
    let diagnostic_summary =
        cx.new(|cx| diagnostics::items::DiagnosticIndicator::new(workspace, cx));
    let active_file_name = cx.new(|_| workspace::active_file_name::ActiveFileName::new());
    let activity_indicator =
        activity_indicator::ActivityIndicator::new(workspace, window, cx);
    let git_blame_status = cx.new(|_| git_ui::GitBlameStatus::default());
    let merge_conflict_indicator =
        cx.new(|cx| git_ui::MergeConflictIndicator::new(workspace, cx));

    // —— Right side ——
    let edit_prediction_ui = cx.new(|cx| {
        edit_prediction_ui::EditPredictionButton::new(
            app_state.fs.clone(),
            app_state.user_store.clone(),
            edit_prediction_menu_handle,
            workspace.project().clone(),
            cx,
        )
    });
    let active_buffer_encoding =
        cx.new(|_| encoding_selector::ActiveBufferEncoding::new(workspace));
    let active_buffer_language =
        cx.new(|_| language_selector::ActiveBufferLanguage::new(workspace));
    let active_toolchain_language =
        cx.new(|cx| toolchain_selector::ActiveToolchain::new(workspace, window, cx));
    let line_ending_indicator =
        cx.new(|_| line_ending_selector::LineEndingIndicator::default());
    let cursor_position =
        cx.new(|_| go_to_line::cursor_position::CursorPosition::new(workspace));
    let image_info = cx.new(|_cx| image_viewer::ImageInfo::new(workspace));
    let vim_mode_indicator = cx.new(|cx| vim::ModeIndicator::new(window, cx));
    let pending_keystrokes_indicator =
        cx.new(|cx| which_key::PendingKeystrokesIndicator::new(window, cx));

    // —— 统一注册进 StatusBar（顺序对齐 Zed）——
    let status_bar = workspace.status_bar().clone();
    status_bar.update(cx, |status_bar, cx| {
        status_bar.add_left_item(search_button, window, cx);
        status_bar.add_left_item(lsp_button, window, cx);
        status_bar.add_left_item(diagnostic_summary, window, cx);
        status_bar.add_left_item(active_file_name, window, cx);
        status_bar.add_left_item(git_blame_status, window, cx);
        status_bar.add_left_item(merge_conflict_indicator, window, cx);
        status_bar.add_left_item(activity_indicator, window, cx);

        status_bar.add_right_item(edit_prediction_ui, window, cx);
        status_bar.add_right_item(active_buffer_encoding, window, cx);
        status_bar.add_right_item(active_buffer_language, window, cx);
        status_bar.add_right_item(active_toolchain_language, window, cx);
        status_bar.add_right_item(line_ending_indicator, window, cx);
        status_bar.add_right_item(cursor_position, window, cx);
        status_bar.add_right_item(image_info, window, cx);
        // 保持 vim 模式指示器与 pending_keystrokes 在最右侧（Zed 原版也放在最后）
        status_bar.add_right_item(vim_mode_indicator, window, cx);
        status_bar.add_right_item(pending_keystrokes_indicator, window, cx);
    });
}
