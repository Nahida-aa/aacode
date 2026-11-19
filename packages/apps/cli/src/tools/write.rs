//! `write`：写入（创建或整体覆盖）一个文件。

use std::sync::Arc;

use aa_agent::tool::{AgentTool, ToolEvents};
use serde::{Deserialize, Serialize};

use crate::tools::Workspace;

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct WriteInput {
    /// 相对工作目录的路径；父目录不存在会自动创建。
    pub path: String,
    /// 完整的新内容（整体覆盖，不是追加）。
    pub content: String,
}

pub struct WriteFile {
    pub workspace: Arc<Workspace>,
}

impl AgentTool for WriteFile {
    type Input = WriteInput;
    type Output = String;
    const NAME: &'static str = "write";

    async fn run(
        self: Arc<Self>,
        input: WriteInput,
        _events: &dyn ToolEvents,
    ) -> Result<String, String> {
        self.workspace.require_write()?;
        let path = self.workspace.resolve(&input.path)?;
        if path.is_dir() {
            return Err(format!("`{}` 是一个目录", input.path));
        }
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("创建目录 `{}` 失败：{e}", parent.display()))?;
        }
        std::fs::write(&path, input.content.as_bytes())
            .map_err(|e| format!("写入 `{}` 失败：{e}", input.path))?;

        Ok(format!(
            "已写入 {} 字节到 {}",
            input.content.len(),
            input.path
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct NoEvents;

    impl ToolEvents for NoEvents {
        fn progress(&self, _msg: &str) {}
        fn cancelled(&self) -> bool {
            false
        }
    }

    fn temp_dir() -> std::path::PathBuf {
        // 并行测试必须各自一个目录：一个用例会断言文件不存在。
        let dir = std::env::temp_dir().join(format!(
            "aacode-write-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[tokio::test]
    async fn creates_missing_parent_directories() {
        let dir = temp_dir();
        let tool = Arc::new(WriteFile {
            workspace: Arc::new(Workspace::new(&dir, true)),
        });
        tool.run(
            WriteInput {
                path: "deep/nested/a.txt".to_owned(),
                content: "hi".to_owned(),
            },
            &NoEvents,
        )
        .await
        .unwrap();
        assert_eq!(
            std::fs::read_to_string(dir.join("deep/nested/a.txt")).unwrap(),
            "hi"
        );
        std::fs::remove_dir_all(&dir).ok();
    }

    #[tokio::test]
    async fn read_only_workspace_refuses_to_write() {
        let dir = temp_dir();
        let tool = Arc::new(WriteFile {
            workspace: Arc::new(Workspace::new(&dir, false)),
        });
        let err = tool
            .run(
                WriteInput {
                    path: "a.txt".to_owned(),
                    content: "hi".to_owned(),
                },
                &NoEvents,
            )
            .await
            .unwrap_err();
        assert!(err.contains("只读模式"), "{err}");
        assert!(!dir.join("a.txt").exists());
        std::fs::remove_dir_all(&dir).ok();
    }
}
