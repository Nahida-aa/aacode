//! observe_new 注册 — 对齐 Zed `crates/zed/src/zed.rs::initialize_workspace`。
//!
//! 关键 insight：Sidebar / TitleBar / Panels 都不在 open_window 里手动创建。
//! 而是在全局 init 之后，通过 `cx.observe_new` 注册回调。
//! 一旦 MultiWorkspace / Workspace entity 被 new，回调自动触发并注入依赖。
//!
//! 之所以要在 `cx.defer` 里创建，是因为 MultiWorkspace::new / Workspace::new 的订阅
//! 链需要先建立好，再注入子 entity（Sidebar 等）。

use gpui::{App, AppContext, Context, KeyBinding, Window};
use std::sync::Arc;
use ui::{ContextMenu, PopoverMenuHandle};

use settings::Settings as _;
use vim_mode_setting::HelixModeSetting;

pub mod panels;

/// 注册两个 observe_new：
/// - **MultiWorkspace** → `cx.defer` 里创建 Sidebar + register_sidebar
/// - **Workspace** → 创建 Dock Panel（agent/project/git 等）+ 注册 StatusBar 按钮
pub fn initialize_workspace(app_state: Arc<workspace::AppState>, cx: &mut App) {
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

/// 把内置默认快捷键（`assets/keymaps/default-{linux,macos,windows}.json`）绑定到全局
/// `gpui::Keymap`。对齐 Zed `crates/zed/src/zed.rs::load_default_keymap`（L2347）。
///
/// 没有这一步，应用里就**一个默认快捷键都没有**：编辑器内建面板
/// （`cx.bind_keys({...})`）之外的绑定——删除字符、全选、保存、查找——全部静默失效，
/// 且不报任何错。表现为「输入 backspace / ctrl-a 没反应」，同时 KeymapEditor 面板
/// 只有 action 列有内容（action 来自代码注册，绑定列要 keymap 数据才有）。
///
/// 加载顺序与 Zed 一致：default → base_keymap（如 vscode 方案）→ vim 模式。
/// 后者覆盖前者，用户 keymap 最后加载再覆盖它们。
///
/// 调用点见 `main.rs`：必须在 `settings::init(cx)` 之后（要读 `BaseKeymap` 全局）。
pub fn load_default_keymap(cx: &mut App) {
    let base_keymap = *settings::BaseKeymap::get_global(cx);
    if base_keymap == settings::BaseKeymap::None {
        return;
    }

    let default = settings::KeymapFile::load_asset_partial(
        settings::DEFAULT_KEYMAP_PATH,
        Some(settings::KeybindSource::Default),
        cx,
    )
    .unwrap_or_else(|e| {
        panic!("failed to load built-in keymap {}: {e}", settings::DEFAULT_KEYMAP_PATH)
    });
    cx.bind_keys(filter_disabled_ai_bindings(default, cx));

    if let Some(asset_path) = base_keymap.asset_path() {
        let bindings = settings::KeymapFile::load_asset_partial(
            asset_path,
            Some(settings::KeybindSource::Base),
            cx,
        )
        .unwrap_or_else(|e| panic!("failed to load base keymap {asset_path}: {e}"));
        cx.bind_keys(filter_disabled_ai_bindings(bindings, cx));
    }

    if vim_mode_setting::HelixModeSetting::get_global(cx).0 {
        let vim = settings::KeymapFile::load_asset_partial(
            settings::VIM_KEYMAP_PATH,
            Some(settings::KeybindSource::Vim),
            cx,
        )
        .unwrap_or_else(|e| {
            panic!("failed to load built-in keymap {}: {e}", settings::VIM_KEYMAP_PATH)
        });
        cx.bind_keys(filter_disabled_ai_bindings(vim, cx));
    }
}

/// 用户开了 `disable_ai` 时，剥掉 AI 相关的绑定，避免按键被一个「handler 静默
/// no-op」的 action 抢占（`ctrl-enter` 之类就无法正常触发）。对齐 Zed
/// `crates/zed/src/zed.rs::filter_disabled_ai_bindings`（L2403）。
fn filter_disabled_ai_bindings(bindings: Vec<KeyBinding>, cx: &App) -> Vec<KeyBinding> {
    if !project::DisableAiSettings::get_global(cx).disable_ai {
        return bindings;
    }
    bindings
        .into_iter()
        .filter(|binding| !is_ai_keybinding(binding))
        .collect()
}

fn is_ai_keybinding(binding: &KeyBinding) -> bool {
    /// 见 Zed `crates/zed/src/zed.rs::AI_ACTION_NAMESPACES`（L2387）。
    const AI_ACTION_NAMESPACES: &[&str] = &[
        "acp::",
        "agent::",
        "assistant::",
        "edit_prediction::",
        "inline_assistant::",
        "zeta::",
    ];
    let name = binding.action().name();
    AI_ACTION_NAMESPACES
        .iter()
        .any(|namespace| name.starts_with(namespace))
}
