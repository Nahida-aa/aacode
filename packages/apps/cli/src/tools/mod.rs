//! 工具集：`read` / `write` / `edit` / `glob`。
//!
//! 工具实现属于**应用层**（`aacode`），不是库层：它们访问用户磁盘、遵守本应用的
//! 工作目录与权限策略。库层（`aa_agent`）只提供 `AgentTool` 接口和注册表。
//!
//! 安全边界（都在这里，不进库）：
//! - 所有路径先 [`Workspace::resolve`]：绝对路径必须在工作目录内，相对路径相对它；
//! - 写操作默认**不注册**（见 [`Workspace::allow_write`]），只读模式模型看不到它们。

mod edit;
mod glob_files;
mod read;
mod write;

use std::path::{Component, Path, PathBuf};
use std::sync::Arc;

use aa_agent::tool::AgentToolRegistry;

pub use edit::EditFile;
pub use glob_files::GlobFiles;
pub use read::ReadFile;
pub use write::WriteFile;

/// 工作目录 + 权限策略。所有工具共享一个（`Arc`，克隆只加引用）。
pub struct Workspace {
    root: PathBuf,
    allow_write: bool,
}

impl Workspace {
    /// `root` 会被规范化为绝对路径（不要求存在时的父级链必须存在）。
    pub fn new(root: impl Into<PathBuf>, allow_write: bool) -> Self {
        let root = root.into();
        let root = root.canonicalize().unwrap_or(root);
        Self { root, allow_write }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn allow_write(&self) -> bool {
        self.allow_write
    }

    /// 把工具给的路径解析成工作目录内的绝对路径。
    ///
    /// 不存在也可以（`write` 要创建新文件），所以校验的是**词法上**不越界：
    /// 拒绝 `..` 上跳和绝对路径逃逸，不做符号链接解析（越界符号链接属于用户自己的
    /// 环境，本工具不试图防住）。
    pub fn resolve(&self, raw: &str) -> Result<PathBuf, String> {
        if raw.trim().is_empty() {
            return Err("path 不能为空".to_owned());
        }

        let raw = Path::new(raw);
        let relative = if raw.is_absolute() {
            raw.strip_prefix(&self.root).map_err(|_| {
                format!(
                    "路径 `{}` 不在工作目录 `{}` 内",
                    raw.display(),
                    self.root.display()
                )
            })?
        } else {
            raw
        };

        let mut out = self.root.clone();
        for component in relative.components() {
            match component {
                Component::Normal(part) => out.push(part),
                Component::CurDir => {}
                Component::ParentDir => {
                    if !out.pop() || !out.starts_with(&self.root) {
                        return Err(format!(
                            "路径 `{}` 试图跳出工作目录 `{}`",
                            raw.display(),
                            self.root.display()
                        ));
                    }
                }
                Component::RootDir | Component::Prefix(_) => {
                    return Err(format!("不支持的绝对路径 `{}`", raw.display()));
                }
            }
        }

        if !out.starts_with(&self.root) {
            return Err(format!("路径 `{}` 不在工作目录内", raw.display()));
        }
        Ok(out)
    }

    /// 写操作的前置检查：只读模式下直接拒绝（工具通常根本没注册，这里是双保险）。
    pub fn require_write(&self) -> Result<(), String> {
        if self.allow_write {
            Ok(())
        } else {
            Err("只读模式：需要写操作请加 --allow-write 重新运行".to_owned())
        }
    }
}

/// 按权限策略组装工具集。
///
/// 只读模式只注册 `read` / `glob`：模型看不到 `write` / `edit`，就不会去试。
pub fn registry(workspace: Arc<Workspace>) -> AgentToolRegistry {
    let registry = AgentToolRegistry::new();
    registry.register(Arc::new(ReadFile {
        workspace: workspace.clone(),
    }));
    registry.register(Arc::new(GlobFiles {
        workspace: workspace.clone(),
    }));
    if workspace.allow_write() {
        registry.register(Arc::new(WriteFile {
            workspace: workspace.clone(),
        }));
        registry.register(Arc::new(EditFile {
            workspace: workspace.clone(),
        }));
    }
    registry
}

#[cfg(test)]
mod tests {
    use super::*;

    fn workspace() -> Workspace {
        Workspace::new("/tmp/aacode-ws", false)
    }

    #[test]
    fn relative_paths_stay_inside_the_workspace() {
        let ws = workspace();
        assert_eq!(
            ws.resolve("src/main.rs").unwrap(),
            PathBuf::from("/tmp/aacode-ws/src/main.rs")
        );
        assert_eq!(
            ws.resolve("./src/../src/main.rs").unwrap(),
            PathBuf::from("/tmp/aacode-ws/src/main.rs")
        );
    }

    #[test]
    fn escaping_the_workspace_is_rejected() {
        let ws = workspace();
        assert!(ws.resolve("../etc/passwd").is_err());
        assert!(ws.resolve("/etc/passwd").is_err());
        assert!(ws.resolve("src/../../x").is_err());
        assert!(ws.resolve("").is_err());
    }

    #[test]
    fn absolute_paths_inside_the_workspace_are_allowed() {
        let ws = workspace();
        assert_eq!(
            ws.resolve("/tmp/aacode-ws/a.rs").unwrap(),
            PathBuf::from("/tmp/aacode-ws/a.rs")
        );
    }

    #[test]
    fn read_only_mode_registers_only_read_tools() {
        let read_only = registry(Arc::new(Workspace::new("/tmp/aacode-ws", false)));
        assert_eq!(read_only.names(), vec!["glob", "read"]);

        let writable = registry(Arc::new(Workspace::new("/tmp/aacode-ws", true)));
        assert_eq!(writable.names(), vec!["edit", "glob", "read", "write"]);
    }
}
