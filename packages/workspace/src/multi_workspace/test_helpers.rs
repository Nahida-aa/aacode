//! MultiWorkspace 的 test-support 方法。
//!
//! 对应 Zed `crates/workspace/src/multi_workspace.rs` 里
//! `#[cfg(any(test, feature = "test-support"))]` 标注的那批方法。
//! 该文件整体按 zed 原本门控缺失，本 fork 保留空模块占位，现补回。

// 子模块不继承父模块的私有 use（mod.rs 顶部那批是私有的），需自行导入。
use gpui::{AppContext as _, Context, Entity, Window};
use project::Project;

use super::{MultiWorkspace, Workspace};

#[cfg(any(test, feature = "test-support"))]
impl MultiWorkspace {
    pub fn test_new(project: Entity<Project>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let workspace = cx.new(|cx| Workspace::test_new(project, window, cx));
        // 用全限定名而非 `Self::new`：本文件 `use gpui::Context` 引入的类型与
        // impl 目标同名，`Self` 在此解析到 Context 上。
        MultiWorkspace::new(workspace, window, cx)
    }
}