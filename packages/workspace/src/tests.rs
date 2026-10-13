//! 从 zed `crates/workspace/src/workspace.rs` 的 `#[cfg(test)] mod tests` 按需移植。
//!
//! 只搬当前 aacode 用得到的部分：测试初始化、以及 dock 面板尺寸相关的用例。
//! zed 那份模块约 7800 行、依赖大量 pane / autosave / 标题栏等用例的辅助函数，
//! 一次性整份搬入会带进一长串暂时用不到的失败用例，因此按主题分批补。

use crate::{MultiWorkspace, dock};
use gpui::{AppContext as _, TestAppContext};
use settings::SettingsStore;
use theme;

use dock::DockPosition;
use dock::test::TestPanel;
use fs::FakeFs;
use project::Project;

pub fn init_test(cx: &mut TestAppContext) {
    cx.update(|cx| {
        let settings_store = SettingsStore::test(cx);
        cx.set_global(settings_store);
        cx.set_global(db::AppDatabase::test_new());
        theme_settings::init(theme::LoadThemes::JustBase, cx);
    });
}

fn set_share_panel_size_in_dock(
    cx: &mut TestAppContext,
    positions: Vec<settings::DockPosition>,
) {
    cx.update_global(|store: &mut SettingsStore, cx| {
        store.update_user_settings(cx, |settings| {
            settings.workspace.share_panel_size_in_dock = Some(positions);
        });
    });
}


/// 同一水平 dock 内切换面板时，宽度跟随切出面板，而不是各面板各记一份。
#[gpui::test]
async fn test_share_panel_size_in_dock_follows_active_panel(cx: &mut TestAppContext) {
    init_test(cx);
    set_share_panel_size_in_dock(cx, vec![settings::DockPosition::Left]);

    let fs = FakeFs::new(cx.executor());
    let project = Project::test(fs, [], cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
    let workspace =
        multi_workspace.read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone());

    // 面板实体必须在 update_in 之外建：`Context<Workspace>` 上没有 `::new`。
    let first_panel = cx.new(|cx| {
        let mut panel = TestPanel::new(DockPosition::Left, 100, cx);
        panel.default_size = gpui::px(350.);
        panel
    });
    let second_panel = cx.new(|cx| {
        let mut panel = TestPanel::new(DockPosition::Left, 200, cx);
        panel.default_size = gpui::px(500.);
        panel
    });
    workspace.update_in(cx, |workspace, window, cx| {
        workspace.add_panel(first_panel.clone(), window, cx);
        workspace.add_panel(second_panel.clone(), window, cx);
        workspace.left_dock().update(cx, |dock, cx| {
            dock.activate_panel(0, window, cx);
            dock.set_open(true, window, cx);
        });
        let _ = (&first_panel, &second_panel);
    });

    // 让两个面板各自拿到一个互不相同的实测宽度：first=400、second=250。
    // 之后若 first 在切回时变成 250，就说明宽度确实是跟着 dock 走，
    // 而不是各面板保留自己的值。
    workspace.update_in(cx, |workspace, window, cx| {
        workspace.resize_left_dock(gpui::px(400.), window, cx);
        workspace.left_dock().update(cx, |dock, cx| {
            dock.activate_panel(1, window, cx);
        });
        workspace.resize_left_dock(gpui::px(250.), window, cx);
    });
    workspace.update_in(cx, |workspace, window, cx| {
        workspace.left_dock().update(cx, |dock, cx| {
            dock.activate_panel(0, window, cx);
        });
    });
    workspace.update_in(cx, |workspace, window, cx| {
        let dock = workspace.left_dock().read(cx);
        assert_eq!(
            dock.stored_panel_size(&first_panel, window, cx),
            Some(gpui::px(250.)),
            "first 应继承切出面板的 250"
        );
    });

    // 反向再切一次，宽度应保持在 250 而非回到 first 自己的 400。
    workspace.update_in(cx, |workspace, window, cx| {
        workspace.left_dock().update(cx, |dock, cx| {
            dock.activate_panel(1, window, cx);
        });
    });
    workspace.update_in(cx, |workspace, window, cx| {
        let dock = workspace.left_dock().read(cx);
        assert_eq!(
            dock.stored_panel_size(&second_panel, window, cx),
            Some(gpui::px(250.))
        );
    });

    set_share_panel_size_in_dock(cx, vec![]);
}

/// 设置只作用于列出的 dock；left 开了不影响 bottom。
#[gpui::test]
async fn test_share_panel_size_in_dock_is_per_position(cx: &mut TestAppContext) {
    init_test(cx);
    set_share_panel_size_in_dock(cx, vec![settings::DockPosition::Right]);

    let fs = FakeFs::new(cx.executor());
    let project = Project::test(fs, [], cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
    let workspace =
        multi_workspace.read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone());

    let first_panel = cx.new(|cx| TestPanel::new(DockPosition::Left, 100, cx));
    let second_panel = cx.new(|cx| TestPanel::new(DockPosition::Left, 200, cx));
    workspace.update_in(cx, |workspace, window, cx| {
        workspace.add_panel(first_panel.clone(), window, cx);
        workspace.add_panel(second_panel.clone(), window, cx);
        workspace.left_dock().update(cx, |dock, cx| {
            dock.activate_panel(0, window, cx);
            dock.set_open(true, window, cx);
        });
        let _ = (&first_panel, &second_panel);
        // 必须先 resize 一次让两个 entry 的 size_state 落地；否则 first 的
        // size_state 是 None，继承到的也是 None。
        workspace.resize_left_dock(gpui::px(400.), window, cx);
    });

    workspace.update_in(cx, |workspace, window, cx| {
        workspace.left_dock().update(cx, |dock, cx| {
            dock.activate_panel(1, window, cx);
        });
    });
    workspace.update_in(cx, |workspace, window, cx| {
        let dock = workspace.left_dock().read(cx);
        // left 未开启 → second 保留自己的 default_size。
        assert_eq!(
            dock.stored_panel_size(&second_panel, window, cx),
            Some(gpui::px(300.))
        );
    });

    set_share_panel_size_in_dock(cx, vec![]);
}