use super::*;

use futures::Future;
use gpui::{App, Context, Task, Window};

use super::MultiWorkspace;
use crate::persistence::model::MultiWorkspaceState;

impl MultiWorkspace {
    pub fn serialize(&mut self, cx: &mut Context<Self>) {
        self._serialize_task = Some(cx.spawn(async move |this, cx| {
            let Ok(task) = this.update(cx, |this, cx| this.serialize_now(cx)) else {
                return;
            };
            task.await;
        }));
    }

    fn serialize_now(&mut self, cx: &mut Context<Self>) -> impl Future<Output = ()> + use<> {
        let state = MultiWorkspaceState {
            active_workspace_id: self.workspace().read(cx).database_id(),
            project_groups: self
                .project_groups
                .iter()
                .map(|group| {
                    crate::persistence::model::SerializedProjectGroup::from_group(
                        &group.key,
                        group.expanded,
                    )
                })
                .collect::<Vec<_>>(),
            sidebar_open: self.sidebar_open,
            sidebar_state: self.sidebar.as_ref().and_then(|s| s.serialized_state(cx)),
        };
        let window_id = self.window_id;
        let kvp = db::kvp::KeyValueStore::global(cx);
        async move {
            crate::persistence::write_multi_workspace_state(&kvp, window_id, state).await;
        }
    }

    /// Used by the quit handler to ensure pending DB writes
    /// complete before the process exits.
    pub fn flush_serialization(&mut self, cx: &mut Context<Self>) -> Task<()> {
        self._serialize_task.take();
        let serialization = self.serialize_now(cx);
        cx.spawn(async move |_, _| serialization.await)
    }

    pub fn flush_pending_serialization(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Vec<Task<()>> {
        let mut tasks = Vec::new();
        for workspace in self.workspaces() {
            tasks.push(workspace.update(cx, |workspace, cx| {
                workspace.flush_serialization(window, cx)
            }));
        }
        tasks.append(&mut self.take_pending_removal_tasks());
        tasks.push(self.flush_serialization(cx));
        tasks
    }

    /// Assigns random database IDs to all retained workspaces, flushes
    /// workspace serialization (SQLite) and multi-workspace state (KVP),
    /// and writes session bindings so the serialized data can be read
    /// back by `last_session_workspace_locations` +
    /// `read_serialized_multi_workspaces`.
    ///
    /// 对齐 Zed `crates/workspace/src/multi_workspace.rs:1696-1697`。Zed 侧同样带
    /// `#[cfg(any(test, feature = "test-support"))]`：它依赖同样被门控的
    /// `Workspace::set_random_database_id`（workspace.rs:8455），且 zed 全部调用点
    /// （persistence.rs:6139 在 mod tests、markdown_preview_view.rs:3315/3462、
    /// open_listener.rs:2742）都在测试代码里，生产路径无调用方。故此处照搬该 gate ——
    /// 去掉会因 set_random_database_id 不存在而编译失败（E0599）。
    ///
    /// core/open_listener.rs 的测试用它来在删窗口前把 workspace 落库，
    /// 否则 `db.last_workspace()` 读不到数据。
    #[cfg(any(test, feature = "test-support"))]
    pub fn flush_all_serialization(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Vec<Task<()>> {
        for workspace in self.workspaces() {
            workspace.update(cx, |ws, _cx| {
                if ws.database_id().is_none() {
                    ws.set_random_database_id();
                }
            });
        }

        let session_id = self.workspace().read(cx).session_id();
        let window_id_u64 = window.window_handle().window_id().as_u64();

        let mut tasks: Vec<Task<()>> = Vec::new();
        for workspace in self.workspaces() {
            tasks.push(workspace.update(cx, |ws, cx| ws.flush_serialization(window, cx)));
            if let Some(db_id) = workspace.read(cx).database_id() {
                let db = crate::persistence::WorkspaceDb::global(cx);
                let session_id = session_id.clone();
                tasks.push(cx.background_spawn(async move {
                    db.set_session_binding(db_id, session_id, Some(window_id_u64))
                        .await
                        .log_err();
                }));
            }
        }
        tasks.push(self.flush_serialization(cx));
        tasks
    }

    pub fn take_pending_removal_tasks(&mut self) -> Vec<Task<()>> {
        let tasks: Vec<Task<()>> = std::mem::take(&mut self.pending_removal_tasks)
            .into_iter()
            .filter(|task| !task.is_ready())
            .collect();
        tasks
    }
}
