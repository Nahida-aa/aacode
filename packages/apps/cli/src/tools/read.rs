//! `read`：读文件，带行号，结果有上限。

use std::io::BufRead as _;
use std::sync::Arc;

use aa_agent::tool::{AgentTool, ToolEvents};
use serde::{Deserialize, Serialize};

use crate::tools::Workspace;

/// 一次最多回多少行（防止一个大文件吃掉整个上下文窗口）。
const MAX_LINES: usize = 2000;
/// 单行最长字符数（压缩过的 JS / 生成的 JSON 会有超长行）。
const MAX_LINE_CHARS: usize = 2000;

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct ReadInput {
    /// 相对工作目录的路径。
    pub path: String,
    /// 从第几行开始（1 起始，默认 1）。
    #[serde(default)]
    pub offset: Option<usize>,
    /// 最多读多少行（默认 2000，上限 2000）。
    #[serde(default)]
    pub limit: Option<usize>,
}

pub struct ReadFile {
    pub workspace: Arc<Workspace>,
}

impl AgentTool for ReadFile {
    type Input = ReadInput;
    type Output = String;
    const NAME: &'static str = "read";

    async fn run(
        self: Arc<Self>,
        input: ReadInput,
        _events: &dyn ToolEvents,
    ) -> Result<String, String> {
        let path = self.workspace.resolve(&input.path)?;

        let file =
            std::fs::File::open(&path).map_err(|e| format!("打开 `{}` 失败：{e}", input.path))?;

        // 流式按行读：只读需要的窗口，不把整个文件读进内存。
        let offset = input.offset.unwrap_or(1).max(1);
        let limit = input.limit.unwrap_or(MAX_LINES).clamp(1, MAX_LINES);

        let mut reader = std::io::BufReader::new(file);
        let mut out = String::with_capacity(8 * 1024);
        let mut written = 0usize;
        let mut number = 0usize;
        let mut more = false;
        let mut buf = String::new();

        loop {
            buf.clear();
            let read = reader
                .read_line(&mut buf)
                .map_err(|e| format!("读取 `{}` 失败：{e}", input.path))?;
            if read == 0 {
                break;
            }
            number += 1;
            if number < offset {
                continue;
            }
            if written == limit {
                // 再多读一行就知道后面还有没有内容。
                more = true;
                break;
            }
            let line = buf.strip_suffix('\n').unwrap_or(&buf);
            let line = line.strip_suffix('\r').unwrap_or(line);
            out.push_str(&format!("{number:>6}│{}\n", truncate(line, MAX_LINE_CHARS)));
            written += 1;
        }

        if written == 0 {
            if number == 0 {
                return Ok(format!("<{} 是空文件>", input.path));
            }
            return Err(format!(
                "`{}` 只有 {number} 行，offset={offset} 超出了文件末尾",
                input.path
            ));
        }

        if more {
            out.push_str(&format!(
                "\n…[已显示 {offset}–{}；继续读请用 offset={}]\n",
                offset + written - 1,
                offset + written
            ));
        }
        Ok(out)
    }
}

/// 按**字符**截断（不是字节），避免在多字节字符中间切开。
fn truncate(line: &str, max_chars: usize) -> String {
    if line.chars().count() <= max_chars {
        return line.to_owned();
    }
    let head: String = line.chars().take(max_chars).collect();
    format!("{head}…[截断]")
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

    fn read(text: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "aacode-read-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("a.txt");
        std::fs::write(&path, text).unwrap();
        path
    }

    #[tokio::test]
    async fn reads_with_line_numbers() {
        let path = read("alpha\nbeta\n");
        let tool = Arc::new(ReadFile {
            workspace: Arc::new(Workspace::new(path.parent().unwrap(), false)),
        });
        let out = tool
            .run(
                ReadInput {
                    path: "a.txt".to_owned(),
                    offset: None,
                    limit: None,
                },
                &NoEvents,
            )
            .await
            .unwrap();
        assert_eq!(out, "     1│alpha\n     2│beta\n");
    }

    #[tokio::test]
    async fn reports_the_remaining_lines() {
        let path = read("a\nb\nc\nd\n");
        let tool = Arc::new(ReadFile {
            workspace: Arc::new(Workspace::new(path.parent().unwrap(), false)),
        });
        let out = tool
            .run(
                ReadInput {
                    path: "a.txt".to_owned(),
                    offset: Some(2),
                    limit: Some(2),
                },
                &NoEvents,
            )
            .await
            .unwrap();
        assert!(out.starts_with("     2│b\n     3│c\n"), "{out}");
        assert!(out.contains("继续读请用 offset=4"), "{out}");
    }

    #[tokio::test]
    async fn missing_file_is_an_error_result() {
        let path = read("x");
        let tool = Arc::new(ReadFile {
            workspace: Arc::new(Workspace::new(path.parent().unwrap(), false)),
        });
        let err = tool
            .run(
                ReadInput {
                    path: "nope.txt".to_owned(),
                    offset: None,
                    limit: None,
                },
                &NoEvents,
            )
            .await
            .unwrap_err();
        assert!(err.contains("打开"), "{err}");
    }
}
