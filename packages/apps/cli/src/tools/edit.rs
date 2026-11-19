//! `edit`：精确字符串替换（old → new）。
//!
//! 不做模糊匹配、不做行号定位：模型看到的文本就是文件内容，要求它给出**唯一**的
//! 原文片段。匹配不到或不唯一都报错，并把原因说清楚（模型据此修正再试）。

use std::sync::Arc;

use aa_agent::tool::{AgentTool, ToolEvents};
use serde::{Deserialize, Serialize};

use crate::tools::Workspace;

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct EditInput {
    /// 相对工作目录的路径。
    pub path: String,
    /// 要被替换的原文（必须在文件里唯一，除非 replace_all）。
    pub old_string: String,
    /// 替换成的新文本。
    pub new_string: String,
    /// 原文出现多次时是否全部替换（默认 false：多次出现即报错）。
    #[serde(default)]
    pub replace_all: bool,
}

pub struct EditFile {
    pub workspace: Arc<Workspace>,
}

impl AgentTool for EditFile {
    type Input = EditInput;
    type Output = String;
    const NAME: &'static str = "edit";

    async fn run(
        self: Arc<Self>,
        input: EditInput,
        _events: &dyn ToolEvents,
    ) -> Result<String, String> {
        self.workspace.require_write()?;
        let path = self.workspace.resolve(&input.path)?;

        if input.old_string.is_empty() {
            return Err("old_string 不能为空；要新建文件请用 write".to_owned());
        }
        if input.old_string == input.new_string {
            return Err("old_string 与 new_string 相同，无需修改".to_owned());
        }

        let content = std::fs::read_to_string(&path)
            .map_err(|e| format!("读取 `{}` 失败：{e}", input.path))?;

        let occurrences = content.matches(&input.old_string).count();
        if occurrences == 0 {
            return Err(format!(
                "`{}` 里找不到 old_string（要精确匹配，包括缩进和换行）",
                input.path
            ));
        }
        if occurrences > 1 && !input.replace_all {
            return Err(format!(
                "old_string 在 `{}` 里出现 {occurrences} 次；请给出更长的唯一片段，\
                 或设 replace_all=true",
                input.path
            ));
        }
        let updated = if input.replace_all {
            content.replace(&input.old_string, &input.new_string)
        } else {
            content.replacen(&input.old_string, &input.new_string, 1)
        };
        std::fs::write(&path, updated.as_bytes())
            .map_err(|e| format!("写入 `{}` 失败：{e}", input.path))?;

        Ok(format!(
            "已修改 {}（{} 处替换）",
            input.path,
            if input.replace_all { occurrences } else { 1 }
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

    fn setup(content: &str) -> (std::path::PathBuf, Arc<EditFile>) {
        // 并行测试必须各自一个目录，否则互相踩文件。
        let dir = std::env::temp_dir().join(format!(
            "aacode-edit-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("a.txt"), content).unwrap();
        let tool = Arc::new(EditFile {
            workspace: Arc::new(Workspace::new(&dir, true)),
        });
        (dir, tool)
    }

    fn input(old: &str, new: &str, replace_all: bool) -> EditInput {
        EditInput {
            path: "a.txt".to_owned(),
            old_string: old.to_owned(),
            new_string: new.to_owned(),
            replace_all,
        }
    }

    #[tokio::test]
    async fn replaces_a_unique_snippet() {
        let (dir, tool) = setup("fn a() {}\nfn b() {}\n");
        let out = tool
            .run(input("fn b() {}", "fn b() { x }", false), &NoEvents)
            .await
            .unwrap();
        assert!(out.contains("1 处"), "{out}");
        assert_eq!(
            std::fs::read_to_string(dir.join("a.txt")).unwrap(),
            "fn a() {}\nfn b() { x }\n"
        );
        std::fs::remove_dir_all(&dir).ok();
    }

    #[tokio::test]
    async fn ambiguous_snippet_is_rejected() {
        let (dir, tool) = setup("x\nx\n");
        let err = tool
            .run(input("x", "y", false), &NoEvents)
            .await
            .unwrap_err();
        assert!(err.contains("出现 2 次"), "{err}");
        // 文件没被改动。
        assert_eq!(
            std::fs::read_to_string(dir.join("a.txt")).unwrap(),
            "x\nx\n"
        );
        std::fs::remove_dir_all(&dir).ok();
    }

    #[tokio::test]
    async fn replace_all_replaces_every_occurrence() {
        let (dir, tool) = setup("x\nx\n");
        tool.run(input("x", "y", true), &NoEvents).await.unwrap();
        assert_eq!(
            std::fs::read_to_string(dir.join("a.txt")).unwrap(),
            "y\ny\n"
        );
        std::fs::remove_dir_all(&dir).ok();
    }

    #[tokio::test]
    async fn missing_snippet_is_an_error_result() {
        let (dir, tool) = setup("abc\n");
        let err = tool
            .run(input("zzz", "y", false), &NoEvents)
            .await
            .unwrap_err();
        assert!(err.contains("找不到"), "{err}");
        std::fs::remove_dir_all(&dir).ok();
    }
}
