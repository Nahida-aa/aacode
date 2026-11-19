//! `glob`：按模式找文件（只列路径，不读内容）。

use std::sync::Arc;

use aa_agent::tool::{AgentTool, ToolEvents};
use serde::{Deserialize, Serialize};

use crate::tools::Workspace;

/// 默认最多列多少条。
const DEFAULT_LIMIT: usize = 200;
const MAX_LIMIT: usize = 2000;

/// 默认跳过的目录：列出来只会淹没真正的目标。
const SKIP_DIRS: [&str; 5] = ["target", "node_modules", ".git", ".venv", "dist"];

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct GlobInput {
    /// glob 模式，相对工作目录，例如 `src/**/*.rs` 或 `**/*.toml`。
    pub pattern: String,
    /// 最多返回多少条（默认 200）。
    #[serde(default)]
    pub limit: Option<usize>,
}

pub struct GlobFiles {
    pub workspace: Arc<Workspace>,
}

impl AgentTool for GlobFiles {
    type Input = GlobInput;
    type Output = String;
    const NAME: &'static str = "glob";

    async fn run(
        self: Arc<Self>,
        input: GlobInput,
        _events: &dyn ToolEvents,
    ) -> Result<String, String> {
        let pattern = self.workspace.resolve(&input.pattern)?;
        let pattern = pattern
            .to_str()
            .ok_or_else(|| format!("模式 `{}` 不是合法 UTF-8", input.pattern))?;
        let limit = input.limit.unwrap_or(DEFAULT_LIMIT).clamp(1, MAX_LIMIT);

        let entries =
            glob::glob(pattern).map_err(|e| format!("模式 `{}` 无效：{e}", input.pattern))?;

        let root = self.workspace.root().to_path_buf();
        let mut matches = Vec::new();
        let mut more = false;
        for entry in entries {
            let Ok(path) = entry else { continue };
            if path.is_dir() || path.components().any(|c| is_skipped(c.as_os_str())) {
                continue;
            }
            let shown = path.strip_prefix(&root).unwrap_or(&path);
            matches.push(shown.display().to_string());
            if matches.len() == limit {
                more = true;
                break;
            }
        }

        if matches.is_empty() {
            return Ok(format!("没有匹配 `{}` 的文件", input.pattern));
        }
        matches.sort();

        let mut out = matches.join("\n");
        out.push('\n');
        if more {
            out.push_str(&format!(
                "…[只显示前 {limit} 条；用更具体的模式或提高 limit 继续]\n"
            ));
        }
        Ok(out)
    }
}

fn is_skipped(component: &std::ffi::OsStr) -> bool {
    let name = component.to_string_lossy();
    SKIP_DIRS.contains(&name.as_ref())
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

    fn tree() -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "aacode-glob-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        std::fs::create_dir_all(dir.join("src")).unwrap();
        std::fs::create_dir_all(dir.join("target")).unwrap();
        std::fs::write(dir.join("src/a.rs"), "").unwrap();
        std::fs::write(dir.join("src/b.rs"), "").unwrap();
        std::fs::write(dir.join("target/c.rs"), "").unwrap();
        std::fs::write(dir.join("README.md"), "").unwrap();
        dir
    }

    #[tokio::test]
    async fn finds_files_and_skips_build_directories() {
        let dir = tree();
        let tool = Arc::new(GlobFiles {
            workspace: Arc::new(Workspace::new(&dir, false)),
        });
        let out = tool
            .run(
                GlobInput {
                    pattern: "**/*.rs".to_owned(),
                    limit: None,
                },
                &NoEvents,
            )
            .await
            .unwrap();
        assert_eq!(out, "src/a.rs\nsrc/b.rs\n");
        std::fs::remove_dir_all(&dir).ok();
    }

    #[tokio::test]
    async fn reports_no_match_instead_of_failing() {
        let dir = tree();
        let tool = Arc::new(GlobFiles {
            workspace: Arc::new(Workspace::new(&dir, false)),
        });
        let out = tool
            .run(
                GlobInput {
                    pattern: "**/*.zig".to_owned(),
                    limit: None,
                },
                &NoEvents,
            )
            .await
            .unwrap();
        assert!(out.contains("没有匹配"), "{out}");
        std::fs::remove_dir_all(&dir).ok();
    }
}
