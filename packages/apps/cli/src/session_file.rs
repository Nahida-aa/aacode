//! 会话文件：路径、追加、恢复。
//!
//! 格式由 `aa_agent::session` 定义（JSONL，一行一个事件）；这里只决定
//! 「放在哪、什么时候写」。库层不碰路径，也不读环境变量。

use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use aa_agent::Session;

/// `$AACODE_SESSIONS`，否则 `{config_dir}/aacode/sessions`。
pub fn sessions_dir() -> PathBuf {
    if let Some(dir) = std::env::var_os("AACODE_SESSIONS") {
        return PathBuf::from(dir);
    }
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("aacode")
        .join("sessions")
}

/// 校验会话名：只允许 ASCII 字母数字和 `._-`，避免路径穿越。
pub fn validate_name(name: &str) -> Result<&str, String> {
    if name.is_empty() {
        return Err("会话名不能为空".to_owned());
    }
    if !name
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-'))
    {
        return Err(format!("会话名 `{name}` 只能包含字母、数字、`.`、`_`、`-`"));
    }
    if name == "." || name == ".." {
        return Err(format!("会话名 `{name}` 不可用"));
    }
    Ok(name)
}

/// `{sessions_dir}/{name}.jsonl`。
pub fn path_for(name: &str) -> Result<PathBuf, String> {
    Ok(sessions_dir().join(format!("{}.jsonl", validate_name(name)?)))
}

/// 最近修改过的会话文件（`--continue` 不带名字时用）。
pub fn latest() -> Result<PathBuf, String> {
    let dir = sessions_dir();
    let entries = fs::read_dir(&dir).map_err(|error| {
        format!(
            "读不到会话目录 {}（{error}）；先不带 --continue 跑一次",
            dir.display()
        )
    })?;

    let mut newest: Option<(SystemTime, PathBuf)> = None;
    for entry in entries {
        let entry = entry.map_err(|error| format!("读会话目录失败：{error}"))?;
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("jsonl") {
            continue;
        }
        let modified = entry
            .metadata()
            .and_then(|meta| meta.modified())
            .unwrap_or(SystemTime::UNIX_EPOCH);
        if newest.as_ref().is_none_or(|(best, _)| modified > *best) {
            newest = Some((modified, path));
        }
    }

    newest.map(|(_, path)| path).ok_or_else(|| {
        format!(
            "{} 里没有可恢复的会话；先不带 --continue 跑一次",
            dir.display()
        )
    })
}

/// 读一个会话文件。格式错误直接报错（带文件名）。
pub fn load(path: &Path) -> Result<Session, String> {
    let text = fs::read_to_string(path)
        .map_err(|error| format!("读会话 {} 失败：{error}", path.display()))?;
    Session::from_jsonl(&text).map_err(|error| format!("会话 {} 无法恢复：{error}", path.display()))
}

/// 追加式会话文件。
///
/// `written` 是已经落盘的事件数；每次 [`sync`](Self::sync) 只写新增的事件。
/// 每轮结束后调用一次：崩溃最多丢掉当前这一轮，之前的对话都在。
pub struct SessionFile {
    path: PathBuf,
    written: usize,
}

impl SessionFile {
    /// 新会话：从第 0 个事件（header）开始写。
    pub fn new(path: PathBuf) -> Self {
        Self { path, written: 0 }
    }

    /// 恢复的会话：文件里已有的事件都算写过。
    pub fn resumed(path: PathBuf, session: &Session) -> Self {
        Self {
            path,
            written: session.event_count(),
        }
    }

    /// 把 `session` 里尚未落盘的事件追加到文件。
    pub fn sync(&mut self, session: &Session) -> Result<(), String> {
        let events = session.events();
        if self.written > events.len() {
            return Err(format!(
                "会话事件从 {} 减少到 {}，拒绝覆盖 {}",
                self.written,
                events.len(),
                self.path.display()
            ));
        }
        if self.written == events.len() {
            return Ok(());
        }

        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)
                .map_err(|error| format!("创建会话目录 {} 失败：{error}", parent.display()))?;
        }
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .map_err(|error| format!("打开会话 {} 失败：{error}", self.path.display()))?;

        let mut buffer = String::new();
        for event in &events[self.written..] {
            buffer.push_str(&event.to_json_line().map_err(|error| error.to_string())?);
            buffer.push('\n');
        }
        file.write_all(buffer.as_bytes())
            .and_then(|()| file.flush())
            .map_err(|error| io_error(&self.path, error))?;

        self.written = events.len();
        Ok(())
    }
}

fn io_error(path: &Path, error: io::Error) -> String {
    format!("写会话 {} 失败：{error}", path.display())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_rejects_path_separators_and_empty() {
        assert!(validate_name("my-session_1.2").is_ok());
        assert!(validate_name("").is_err());
        assert!(validate_name("../x").is_err());
        assert!(validate_name("a/b").is_err());
        assert!(validate_name("..").is_err());
    }
}
