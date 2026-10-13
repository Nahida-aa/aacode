//! MultiWorkspace 的 test-support 方法。
//!
//! 对应 Zed `crates/workspace/src/multi_workspace.rs` 里
//! `#[cfg(any(test, feature = "test-support"))]` 标注的那批方法。
//! 该文件整体按 zed 原本门控缺失，本 fork 保留空模块占位，现补回。

// 子模块不继承父模块的私有 use（mod.rs 顶部那批是私有的），需自行导入。
use gpui::{AppContext as _, Context, Entity, Task, Window};
use project::Project;
use util::ResultExt as _;

use super::{MultiWorkspace, Workspace};

#[cfg(any(test, feature = "test-support"))]
impl MultiWorkspace {
    pub fn test_new(project: Entity<Project>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let workspace = cx.new(|cx| Workspace::test_new(project, window, cx));
        // 用全限定名而非 `Self::new`：本文件 `use gpui::Context` 引入的类型与
        // impl 目标同名，`Self` 在此解析到 Context 上。
        MultiWorkspace::new(workspace, window, cx)
    }

    pub fn set_random_database_id(&mut self, cx: &mut Context<Self>) {
        self.workspace().update(cx, |workspace, _cx| {
            workspace.set_random_database_id();
        });
    }

    pub fn test_add_workspace(
        &mut self,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<Workspace> {
        let workspace = cx.new(|cx| Workspace::test_new(project, window, cx));
        self.activate(workspace.clone(), None, window, cx);
        workspace
    }

    pub fn create_test_workspace(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<()> {
        let app_state = self.workspace().read(cx).app_state().clone();
        let project = Project::local(
            app_state.client.clone(),
            app_state.node_runtime.clone(),
            app_state.user_store.clone(),
            app_state.languages.clone(),
            app_state.fs.clone(),
            None,
            project::LocalProjectFlags::default(),
            cx,
        );
        let new_workspace = cx.new(|cx| Workspace::new(None, project, app_state, window, cx));
        self.activate(new_workspace.clone(), None, window, cx);

        let weak_workspace = new_workspace.downgrade();
        let db = crate::persistence::WorkspaceDb::global(cx);
        cx.spawn_in(window, async move |this, cx| {
            let workspace_id = db.next_id().await.unwrap();
            let workspace = weak_workspace.upgrade().unwrap();
            let task: Task<()> = this
                .update_in(cx, |this, window, cx| {
                    let session_id = workspace.read(cx).session_id();
                    let window_id = window.window_handle().window_id().as_u64();
                    workspace.update(cx, |workspace, _cx| {
                        workspace.set_database_id(workspace_id);
                    });
                    this.serialize(cx);
                    let db = db.clone();
                    cx.background_spawn(async move {
                        db.set_session_binding(workspace_id, session_id, Some(window_id))
                            .await
                            .log_err();
                    })
                })
                .unwrap();
            task.await
        })
    }
}