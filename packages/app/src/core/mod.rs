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
        // Zed main.rs L1258：kind 有值时走完上面的定向动作就结束，不再落到下面的
        // 打开路径逻辑。少了这个 return 会与上面各臂的 `app_state` move 冲突，
        // 且语义也不对（kind 与路径打开是互斥的两条路）。
        return;
    }

    // —— 以下对齐 Zed crates/zed/src/main.rs L1277-1310 ——
    //
    // 只有 kind 为 None 时才会到这里，即"没有定向动作、只打开路径"。file:// /
    // zed://file 这类 url 由 OpenRequest::parse 填进 open_paths 而不动 kind
    // （open_listener.rs:163），所以 `aacode --new <file>` 正是走这条路径。
    //
    // 最初移植时漏了整个 L1277-1372 的函数尾部，导致
    // test_e2e_explicit_new_flag_with_file_url_opens_new_window 失败：
    // `aacode --new <file>` 收到后 open_paths 被丢弃，窗口数停在 1。
    let mut task = None;
    let dev_container = request.dev_container;
    if !request.open_paths.is_empty() || !request.diff_paths.is_empty() {
        let app_state = app_state.clone();
        let base_open_options = open_listener::open_options_for_request(
            request.open_behavior,
            &workspace::SerializedWorkspaceLocation::Local,
            cx,
        );
        task = Some(cx.spawn(async move |cx| {
            let paths_with_position = open_listener::derive_paths_with_position(
                app_state.fs.as_ref(),
                request.open_paths,
            )
            .await;
            let (_window, results) = open_listener::open_paths_with_positions(
                &paths_with_position,
                &request.diff_paths,
                request.diff_all,
                app_state,
                workspace::OpenOptions {
                    open_in_dev_container: dev_container,
                    ..base_open_options
                },
                cx,
            )
            .await?;
            for result in results.into_iter().flatten() {
                if let Err(err) = result {
                    log::error!("Error opening path: {err:#}");
                }
            }
            anyhow::Ok(())
        }));
    }

    // Zed 同处还有 open_channel_notes / join_channel 分支（main.rs L1308-1360），
    // aacode 的 open_listener 未移植该功能，故此处省略而非留空实现。
    if let Some(task) = task {
        cx.spawn(async move |cx| {
            if let Err(err) = task.await {
                fail_to_open_window_async(err, cx);
            }
        })
        .detach();
    }
}

/// 开窗口失败时的用户提示。对齐 Zed `crates/zed/src/main.rs:153-190`。
///
/// Linux/freebsd 走 ashpd 桌面通知（XDG 桌面门户），其余平台直接退出 —— 开不出窗口时
/// 继续跑一个没有窗口的进程没有意义，故两条路最终都 `process::exit(1)`。
fn fail_to_open_window_async(e: anyhow::Error, cx: &mut AsyncApp) {
    cx.update(|cx| fail_to_open_window(e, cx));
}

fn fail_to_open_window(e: anyhow::Error, _cx: &mut App) {
    eprintln!(
        "aacode failed to open a window: {e:?}. See https://aacode.dev/docs/linux for troubleshooting steps."
    );
    #[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
    {
        std::process::exit(1);
    }

    // Maybe unify this with gpui::platform::linux::platform::ResultExt::notify_err(..)?
    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    {
        use ashpd::desktop::notification::{Notification, NotificationProxy, Priority};
        _cx.spawn(async move |_cx| {
            let Ok(proxy) = NotificationProxy::new().await else {
                std::process::exit(1);
            };

            proxy
                .add_notification(
                    // 与 Zed 的 "dev.zed.Oops" 同理：用固定 id 避免同一失败刷多条通知。
                    "dev.aacode.Oops",
                    Notification::new("aacode failed to launch")
                        .body(Some(
                            format!(
                                "{e:?}. See https://aacode.dev/docs/linux for troubleshooting steps."
                            )
                            .as_str(),
                        ))
                        .priority(Priority::High)
                        .icon(ashpd::desktop::Icon::with_names(&[
                            "dialog-question-symbolic",
                        ])),
                )
                .await
                .ok();

            std::process::exit(1);
        })
        .detach();
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
