use std::path::PathBuf;

use anyhow::Result;
use collections::HashMap;
pub use ipc_channel::ipc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct IpcHandshake {
    pub requests: ipc::IpcSender<CliRequest>,
    pub responses: ipc::IpcReceiver<CliResponse>,
}

/// Controls how CLI paths are opened — whether to reuse existing windows,
/// create new ones, or add to the sidebar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum OpenBehavior {
    /// Consult the user's `cli_default_open_behavior` setting.
    #[default]
    Default,
    /// Always create a new window. No matching against existing worktrees.
    /// Corresponds to `zed -n`.
    AlwaysNew,
    /// Create a new window unless opening a subpath of an existing project.
    PreferNewWindow,
    /// Match broadly including subdirectories, and fall back to any existing
    /// window if no worktree matched. Corresponds to `zed -a`.
    Add,
    /// Open directories as a new workspace in the current Zed window's sidebar.
    /// Reuse existing windows for files in open worktrees.
    /// Corresponds to `zed -e`.
    ExistingWindow,
    /// New window for directories, reuse existing window for files in open
    /// worktrees. The classic pre-sidebar behavior.
    /// Corresponds to `zed --classic`.
    Classic,
    /// Replace the content of an existing window with a new workspace.
    /// Corresponds to `zed -r`.
    Reuse,
}

/// The setting-level enum for configuring default behavior. This only has
/// two values because the other modes are always explicitly requested via
/// CLI flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CliBehaviorSetting {
    /// Open directories as a new workspace in the current Zed window's sidebar.
    ExistingWindow,
    /// Open paths in a new window unless they are subpaths of an existing project.
    NewWindow,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CliRequest {
    Open {
        paths: Vec<String>,
        urls: Vec<String>,
        diff_paths: Vec<[String; 2]>,
        diff_all: bool,
        wsl: Option<String>,
        wait: bool,
        #[serde(default)]
        open_behavior: OpenBehavior,
        env: Option<HashMap<String, String>>,
        user_data_dir: Option<String>,
        dev_container: bool,
        #[serde(default)]
        cwd: Option<PathBuf>,
    },
    SetOpenBehavior {
        behavior: CliBehaviorSetting,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CliResponse {
    Ping,
    Stdout { message: String },
    Stderr { message: String },
    Exit { status: i32 },
    PromptOpenBehavior,
}

/// When Zed started not as an *.app but as a binary (e.g. local development),
/// there's a possibility to tell it to behave "regularly".
///
/// Note that in the main zed binary, this variable is unset after it's read for the first time,
/// therefore it should always be accessed through the `FORCE_CLI_MODE` static.
pub const FORCE_CLI_MODE_ENV_VAR_NAME: &str = "ZED_FORCE_CLI_MODE";

/// Abstracts the transport for sending CLI responses (Zed → CLI).
///
/// Production code uses `IpcSender<CliResponse>`. Tests can provide in-memory
/// implementations to avoid OS-level IPC.
pub trait CliResponseSink: Send + 'static {
    fn send(&self, response: CliResponse) -> Result<()>;
}

impl CliResponseSink for ipc::IpcSender<CliResponse> {
    fn send(&self, response: CliResponse) -> Result<()> {
        ipc::IpcSender::send(self, response).map_err(|error| anyhow::anyhow!("{error}"))
    }
}

/// Unix domain socket 的文件名前缀，服务于「第二个实例把打开请求转发给已运行实例」
/// 这条链路（`--new` / 命令行路径 / desktop 的 [Desktop Action NewWorkspace] 都走它）。
///
/// 【有意偏离 Zed】Zed 用 `zed-{channel}.sock`（crates/zed/src/zed.rs:594）——它只有
/// 自己一个产品，前缀叫什么无所谓。aacode 与 Zed 官方版共用同一个 data_dir
/// （`paths::APP_NAME` 仍是 `"Zed"`）且默认 channel 同为 `stable`，共用 socket 会导致：
/// 1. 两者的 bind 互相失败 → 都判定「已有实例在跑」而退出，无法同时运行；
/// 2. app 侧那段「清理残留 socket」的 `remove_file` 会删掉对方正在用的 socket。
///
/// 定义在此处（而非各自硬编码）是因为 bind 方在 app、connect 方在 cli，两处必须一致 ——
/// 不一致时 cli 会连到不存在的 socket，静默退化成「另起一个实例」而非转发。
pub const CLI_SOCKET_PREFIX: &str = "aacode-";

/// 首个实例监听的 socket 路径。调用方：`cli` 的转发逻辑、`app` 的
/// `listen_for_cli_connections`。
///
/// 参数 `data_dir` 由调用方决定（cli 支持 `--user-data-dir` 覆盖，故不能在此直接用
/// `paths::data_dir()`）。
pub fn cli_socket_path(data_dir: &std::path::Path) -> PathBuf {
    data_dir.join(format!(
        "{CLI_SOCKET_PREFIX}{}.sock",
        *release_channel::RELEASE_CHANNEL_NAME
    ))
}
