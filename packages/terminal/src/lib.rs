//! AA Terminal Backend — 纯 PTY + alacritty 模拟器，零 GPUI 依赖。
//!
//! 渲染层（Element + View）在 `aa-terminal-view` crate.

pub mod actions;
pub mod alacritty;
mod constants;
mod error;
mod events;
mod grid;
mod input;
mod mappings;
mod pty;
pub mod terminal_settings;

pub use actions::*;
pub use constants::MAX_SCROLL_HISTORY_LINES;
pub use error::TerminalError;
pub use events::{Event, MaybeNavigationTarget, PathLikeTarget, PtyEvent};

pub use pty::Terminal;
pub use pty::builder::{TerminalBuilder, insert_zed_terminal_env};
pub use pty::process::subprocess::HeadlessTerminal;
pub use pty::task::{TaskState, TaskStatus, TerminalMode};

pub use input::mouse::MouseInputMode;
pub use input::scroll::{Scroll, ViMotion};
pub use input::selection::{Selection, SelectionPhase, SelectionSide, SelectionType};

pub use grid::ansi::{AnsiSpans, ParsedAnsiText, parse_ansi_text, strip_ansi_text};
pub use grid::colors::{
    Color, NamedColor, get_color_at_index, is_app_chosen_exact_color, is_default_background_color,
    rgba_color,
};
pub use grid::content::cell::{Cell, IndexedCell, RenderableCells};
pub use grid::content::content::Content;
pub use grid::content::search::Search;
pub use grid::content::{GridLinesChange, HoveredWord};
pub use grid::geometry::bounds::TerminalBounds;
pub use grid::geometry::cursor::{Cursor, CursorShape};
pub use grid::geometry::modes::Modes;
pub use grid::geometry::point::{Point, Range, SelectionRange};
pub use grid::hyperlink::{Hyperlink, HyperlinkData};

use gpui::EventEmitter;

impl Drop for Terminal {
    fn drop(&mut self) {
        if let Some(subprocess) = self.subprocess.take() {
            subprocess.kill();
        }
        self.release_pty_resources();
    }
}

impl EventEmitter<Event> for Terminal {}
