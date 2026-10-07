//! 全局 init 链 — 对齐 Zed `crates/zed/src/main.rs`。
//!
//! 注意：当前所有初始化步骤直接内联在 `main.rs` 的 `app.run` 闭包内。
//! 此模块暂为占位，后续可把与 AppState 无关的纯 crate init 链提取到此。

pub mod about;
pub mod migrate;
pub mod open_listener;
pub mod quick_action_bar;

pub use about::open_about_window;
pub use migrate::MigrationBanner;
pub use quick_action_bar::QuickActionBar;

// ---------------------------------------------------------------------------
// 以下两个函数原本在 Zed 的 crates/zed/src/main.rs 里，core/open_listener.rs
// 通过 `crate::` 引用它们。Zed 单文件 crate 里 `crate::` 就是 crate 根，天然可达；
// aacode 把 open_listener 拆到 core/ 下后，这两个函数必须一并搬过来。
//
// 名字保持 Zed 原名（handle_open_request / restore_or_create_workspace），
// 只因拆分而换了所在模块。
// ---------------------------------------------------------------------------

use std::sync::Arc;

use anyhow::Result;
use db::kvp::KeyValueStore;
use editor::Editor;
use gpui::{App, AsyncApp, TaskExt};
use onboarding::{FIRST_OPEN, show_onboarding_view};
use settings::Settings as _;
use workspace::{
    AppState, RestoreOnStartupBehavior, SerializedWorkspaceLocation, SessionWorkspace, Toast,
    WorkspaceSettings, notifications::NotificationId,
};

use open_listener::{OpenRequest, OpenRequestKind};

/// 分派一个 [`OpenRequest`]。对齐 Zed `crates/zed/src/main.rs:1008`。
///
/// 五种 kind 里目前只有 `CliConnection` 与 `FocusApp` 有实现 —— 这是让
/// `aacode --new` 生效所需的最小集合：
/// - `CliConnection`：cli 二进制经 `zed-cli://` IPC 把请求转发进来，
///   落到 [`open_listener::handle_cli_connection`]（open_listener.rs:575）。
/// - `FocusApp`：没有已运行实例时的兜底，转 [`restore_or_create_workspace`]。
///
/// 其余 kind（Extension / AgentPanel / InstallSkill / DockMenuAction /
/// BuiltinJsonSchema）依赖 aacode 尚未移植的入口，收到时只记日志不动作。
/// 见 .agents/zed-modifications.md。
pub fn handle_open_request(request: OpenRequest, app_state: Arc<AppState>, cx: &mut App) {
    if let Some(kind) = request.kind {
        match kind {
            OpenRequestKind::CliConnection(connection) => {
                cx.spawn(async move |cx| {
                    open_listener::handle_cli_connection(connection, app_state, cx).await
                })
                .detach();
            }
            OpenRequestKind::FocusApp => {
                cx.spawn(async move |cx| {
                    restore_or_create_workspace(app_state, cx).await
                })
                .detach_and_log_err(cx);
            }
            OpenRequestKind::Extension { extension_id } => {
                log::warn!("Extension({extension_id}) 暂未支持");
            }
            OpenRequestKind::AgentPanel {
                external_source_prompt,
            } => {
                log::warn!("AgentPanel({external_source_prompt:?}) 暂未支持");
            }
            OpenRequestKind::InstallSkill { content } => {
                log::warn!("InstallSkill({} bytes) 暂未支持", content.len());
            }
            OpenRequestKind::DockMenuAction { index } => {
                log::warn!("DockMenuAction({index}) 暂未支持");
            }
            OpenRequestKind::BuiltinJsonSchema { schema_path } => {
                log::warn!("BuiltinJsonSchema({schema_path}) 暂未支持");
            }
            _ => {}
        }
    }
}

/// 启动时的窗口恢复：优先按用户的 `restore_on_startup` 设置恢复上次窗口，
/// 都没有则新建一个。对齐 Zed `crates/zed/src/main.rs:1424`。
///
/// aacode 与 Zed 的差异：Zed 这段还会走 remote 分支（ssh 连接的 workspace），
/// 需要的 `restore_multiworkspace` / `open_remote_project` 虽已存在，但
/// `RemoteSettings::fill_connection_options_from_settings` 等调用点需要
/// 逐个核对，故当前只保留 local 分支 + 新建兜底。remote 恢复待后续补齐。
pub(crate) async fn restore_or_create_workspace(
    app_state: Arc<AppState>,
    cx: &mut AsyncApp,
) -> Result<()> {
    let kvp = cx.update(|cx| KeyValueStore::global(cx));
    if let Some(multi_workspaces) = restorable_workspaces(cx, &app_state).await {
        let mut error_count = 0;
        for multi_workspace in multi_workspaces {
            let result = match &multi_workspace.active_workspace.location {
                SerializedWorkspaceLocation::Local => {
                    workspace::restore_multiworkspace(
                        multi_workspace,
                        app_state.clone(),
                        cx,
                    )
                    .await
                    .map(|_| ())
                }
                SerializedWorkspaceLocation::Remote(_) => {
                    // aacode 暂不支持 remote workspace 恢复，见上方文档注释。
                    log::warn!("remote workspace 恢复暂未支持，跳过");
                    continue;
                }
            };

            if let Err(error) = result {
                log::error!("Failed to restore workspace: {error:#}");
                error_count += 1;
            }
        }

        if error_count > 0 {
            let message = if error_count == 1 {
                "Failed to restore 1 workspace. Check logs for details.".to_string()
            } else {
                format!(
                    "Failed to restore {} workspaces. Check logs for details.",
                    error_count
                )
            };

            // 找一个活动窗口把 toast 显示出来
            let toast_shown = cx.update(|cx| {
                if let Some(window) = cx.active_window()
                    && let Some(multi_workspace) = window.downcast::<workspace::MultiWorkspace>()
                {
                    multi_workspace
                        .update(cx, |multi_workspace, _, cx| {
                            multi_workspace.workspace().update(cx, |workspace, cx| {
                                workspace.show_toast(
                                    Toast::new(NotificationId::unique::<()>(), message.clone()),
                                    cx,
                                )
                            });
                        })
                        .ok();
                    return true;
                }
                false
            });

            // 没有成功打开任何窗口 → 开一个空 workspace 并在那里显示 toast
            if !toast_shown {
                log::error!(
                    "All workspace restorations failed. Opening fallback empty workspace."
                );
                cx.update(|cx| {
                    workspace::open_new(
                        Default::default(),
                        app_state.clone(),
                        cx,
                        |workspace, _window, cx| {
                            workspace.show_toast(
                                Toast::new(NotificationId::unique::<()>(), message),
                                cx,
                            );
                        },
                    )
                })
                .await?;
            }
        }

        // 用户在启动时取消了失败的 remote 连接，open_remote_project 返回 Ok 但移除了
        // 窗口，于是 error_count 仍是 0，上面的 toast 兜底不触发。没有这个检查的话
        // 进程会静默退出。（对齐 Zed main.rs L1544-L1547 的注释）
        if cx.update(|cx| cx.windows().is_empty()) {
            cx.update(|cx| {
                workspace::open_new(
                    Default::default(),
                    app_state.clone(),
                    cx,
                    |workspace, window, cx| {
                        let restore_on_startup = WorkspaceSettings::get_global(cx).restore_on_startup;
                        match restore_on_startup {
                            RestoreOnStartupBehavior::Launchpad => {}
                            _ => {
                                Editor::new_file(workspace, &Default::default(), window, cx);
                            }
                        }
                    },
                )
            })
            .await?;
        }
    } else if matches!(kvp.read_kvp(FIRST_OPEN), Ok(None)) {
        cx.update(|cx| show_onboarding_view(app_state, cx)).await?;
    } else {
        cx.update(|cx| {
            workspace::open_new(
                Default::default(),
                app_state,
                cx,
                |workspace, window, cx| {
                    let restore_on_startup = WorkspaceSettings::get_global(cx).restore_on_startup;
                    match restore_on_startup {
                        RestoreOnStartupBehavior::Launchpad => {}
                        _ => {
                            Editor::new_file(workspace, &Default::default(), window, cx);
                        }
                    }
                },
            )
        })
        .await?;
    }

    Ok(())
}

/// 读取上次退出的 workspace 位置列表。对齐 Zed `crates/zed/src/main.rs:1583`。
async fn restorable_workspaces(
    cx: &mut AsyncApp,
    app_state: &Arc<AppState>,
) -> Option<Vec<workspace::SerializedMultiWorkspace>> {
    let locations = restorable_workspace_locations(cx, app_state).await?;
    Some(cx.update(|cx| workspace::read_serialized_multi_workspaces(locations, cx)))
}

/// 按 `restore_on_startup` 设置算出该恢复哪些 workspace。
/// 对齐 Zed `crates/zed/src/main.rs:1590`。
pub(crate) async fn restorable_workspace_locations(
    cx: &mut AsyncApp,
    app_state: &Arc<AppState>,
) -> Option<Vec<SessionWorkspace>> {
    let (mut restore_behavior, db) = cx.update(|cx| {
        (
            WorkspaceSettings::get(None, cx).restore_on_startup,
            workspace::WorkspaceDb::global(cx),
        )
    });

    let session_handle = app_state.session.clone();
    let (last_session_id, last_session_window_stack) = cx.update(|cx| {
        let session = session_handle.read(cx);

        (
            session.last_session_id().map(|id| id.to_string()),
            session.last_session_window_stack(),
        )
    });

    if last_session_id.is_none()
        && matches!(
            restore_behavior,
            workspace::RestoreOnStartupBehavior::LastSession
        )
    {
        restore_behavior = workspace::RestoreOnStartupBehavior::LastWorkspace;
    }

    match restore_behavior {
        workspace::RestoreOnStartupBehavior::LastWorkspace => {
            workspace::last_opened_workspace_location(&db, app_state.fs.as_ref())
                .await
                .map(|(workspace_id, location, paths)| {
                    vec![SessionWorkspace {
                        workspace_id,
                        location,
                        paths,
                        window_id: None,
                    }]
                })
        }
        workspace::RestoreOnStartupBehavior::LastSession => {
            if let Some(last_session_id) = last_session_id {
                let ordered = last_session_window_stack.is_some();

                let mut locations = workspace::last_session_workspace_locations(
                    &db,
                    &last_session_id,
                    last_session_window_stack,
                    app_state.fs.as_ref(),
                )
                .await
                .filter(|locations| !locations.is_empty());

                // last_session_window_order 返回的窗口是「前到后」排序的，
                // 所以要先打开上次最靠前的窗口。
                if ordered && let Some(locations) = locations.as_mut() {
                    locations.reverse();
                }

                locations
            } else {
                None
            }
        }
        _ => None,
    }
}
