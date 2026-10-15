pub mod builder;
pub mod colors;
pub mod content;
pub mod cwd;
pub mod hyperlink;
pub mod init_command;
pub mod input;
pub mod keyboard;
pub mod mouse;
pub mod process;
pub mod process_name;
pub mod pty;
pub mod scroll;
pub mod selection;
pub mod subprocess;
pub mod task;
pub mod title;

use async_channel::Sender;
use gpui::{BackgroundExecutor, Pixels, Point as GpuiPoint, Task};
use std::{collections::VecDeque, process::ExitStatus, sync::Arc, time::Instant};
use vte::ansi::{Processor, StdSyncHandler};

use crate::alacritty::{AlacrittyTermConfig, AlacrittyTermLock, HyperlinkMatch, RegexSearches};
use crate::events::{InternalEvent, PtyEvent};
use crate::types::{
    content::Content,
    point::{Point, Range},
    selection::{SelectionPhase, SelectionSide},
};
use crate::{CopyTemplate, terminal::task::TaskState};
use util::paths::PathStyle;

use cwd::CwdHistoryEntry;
use pty::TerminalType;
use subprocess::SubprocessHandle;

pub struct Terminal {
    pub(crate) terminal_type: TerminalType,
    /// Set for non-PTY terminals (see [`HeadlessTerminal`]); owns the spawned
    /// subprocess and the task pumping its output into the grid.
    pub(crate) subprocess: Option<SubprocessHandle>,
    pub(crate) completion_tx: Option<Sender<Option<ExitStatus>>>,
    pub(crate) term: Arc<AlacrittyTermLock>,
    pub(crate) term_config: AlacrittyTermConfig,
    pub(crate) output_processor: Option<Processor<StdSyncHandler>>,
    pub(crate) output_events_tx: Option<futures::channel::mpsc::UnboundedSender<PtyEvent>>,
    pub(crate) output_generation: u64,
    pub(crate) events: VecDeque<InternalEvent>,
    /// This is only used for mouse mode cell change detection
    pub(crate) last_mouse: Option<(Point, SelectionSide)>,
    /// Window-relative position of the most recent left mouse-down. Used to
    /// apply a drag threshold before starting a selection (see #58970).
    pub(crate) mouse_down_position: Option<GpuiPoint<Pixels>>,
    pub matches: Vec<Range>,
    pub last_content: Content,
    pub selection_head: Option<Point>,

    pub breadcrumb_text: String,
    pub(crate) title_override: Option<String>,
    pub(crate) scroll_px: Pixels,
    pub(crate) next_link_id: usize,
    pub(crate) selection_phase: SelectionPhase,
    pub(crate) hyperlink_regex_searches: RegexSearches,
    pub(crate) task: Option<TaskState>,
    pub(crate) vi_mode_enabled: bool,
    pub(crate) is_remote_terminal: bool,
    pub(crate) last_mouse_move_time: Instant,
    pub(crate) last_hyperlink_search_position: Option<GpuiPoint<Pixels>>,
    pub(crate) mouse_down_hyperlink: Option<HyperlinkMatch>,
    #[cfg(windows)]
    pub(crate) shell_program: Option<String>,
    pub(crate) template: CopyTemplate,
    pub(crate) activation_script: Vec<String>,
    pub(crate) child_exited: Option<ExitStatus>,
    pub(crate) keyboard_input_sent: bool,
    pub(crate) init_command_startup_marker: Option<String>,
    pub(crate) init_command_startup_tx: Option<Sender<()>>,
    pub(crate) event_loop_task: Task<Result<(), anyhow::Error>>,
    pub(crate) background_executor: BackgroundExecutor,
    pub(crate) path_style: PathStyle,
    pub(crate) cwd_history: Vec<CwdHistoryEntry>,
    pub(crate) pending_cwd_boundary: Option<i32>,
    #[cfg(any(test, feature = "test-support"))]
    pub(crate) input_log: Vec<Vec<u8>>,
    #[cfg(test)]
    pub(crate) suppress_hyperlink_throttle_once: bool,
    #[cfg(any(test, feature = "test-support"))]
    pub(crate) pty_write_log: std::cell::RefCell<Vec<Vec<u8>>>,
}
