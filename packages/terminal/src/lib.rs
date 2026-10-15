//! AA Terminal Backend — 纯 PTY + alacritty 模拟器，零 GPUI 依赖。
//!
//! 渲染层（Element + View）在 `aa-terminal-view` crate.

pub mod actions;
pub mod alacritty;
mod ansi;
mod cmd;
mod constants;
mod error;
mod events;
mod headless;
mod mappings;
mod pty_info;
mod terminal;
pub mod terminal_settings;
mod types;

pub use actions::*;
pub use ansi::{AnsiSpans, ParsedAnsiText, parse_ansi_text, strip_ansi_text};
pub use constants::MAX_SCROLL_HISTORY_LINES;
pub use error::TerminalError;
pub use events::{Event, MaybeNavigationTarget, PathLikeTarget, PtyEvent};
pub use headless::HeadlessTerminal;

pub use terminal::{
    Terminal,
    builder::{TerminalBuilder, insert_zed_terminal_env},
    task::{MouseInputMode, TaskState, TaskStatus, TerminalMode},
};

pub use types::{
    bounds::TerminalBounds,
    cell::{Cell, Hyperlink, HyperlinkData, IndexedCell, RenderableCells},
    content::{Content, GridLinesChange, HoveredWord},
    cursor::{Cursor, CursorShape},
    modes::Modes,
    point::{Point, Range, SelectionRange},
    scroll::{Scroll, ViMotion},
    selection::{Search, Selection, SelectionPhase, SelectionSide, SelectionType},
};

pub use terminal::colors::{
    Color, NamedColor, get_color_at_index, is_app_chosen_exact_color, is_default_background_color,
    rgba_color,
};

use collections::HashMap;
use gpui::EventEmitter;
use std::time::Duration;
use util::shell::Shell;

use terminal_settings::{AlternateScroll, CursorShape as SettingsCursorShape};

pub(crate) struct CopyTemplate {
    shell: Shell,
    env: HashMap<String, String>,
    cursor_shape: SettingsCursorShape,
    alternate_scroll: AlternateScroll,
    max_scroll_history_lines: Option<usize>,
    path_hyperlink_regexes: Vec<String>,
    path_hyperlink_timeout: Duration,
    window_id: u64,
}

impl Drop for Terminal {
    fn drop(&mut self) {
        if let Some(subprocess) = self.subprocess.take() {
            subprocess.kill();
        }
        self.release_pty_resources();
    }
}

impl EventEmitter<Event> for Terminal {}
