mod config;
mod content;
mod convert;
mod listener;
mod geometry;
mod hyperlinks;
mod ops;
mod pty;
pub mod search;
mod text;
mod types;
pub(crate) use convert::*;
pub(super) use hyperlinks::{HyperlinkMatch, RegexSearches};
pub(super) use types::{
    AlacrittyCell, AlacrittyGridIterator, AlacrittyHyperlink, AlacrittyPty, AlacrittyTerm,
    AlacrittyTermConfig, AlacrittyTermLock,
};

pub(crate) use crate::alacritty::{
    config::{apply_config, display_only_term_config, pty_term_config, set_default_cursor_style},
    content::{full_content_range, screen_lines, total_lines},
    ops::{
        append_text_to_term, clear_saved_screen, display_offset, new_term, resize, scroll_display,
        scroll_to_point, selection_text, set_selection, shrink_to_used, toggle_vi_mode,
        update_selection, update_selection_to_vi_cursor, update_vi_cursor_for_scroll, used_lines,
        vi_goto_point, vi_motion,
    },
    pty::{PtySender, current_child_signal_mask, open_pty, pty_options, spawn_event_loop},
    search::{AlacrittySearch, find_from_terminal_point, search_matches},
    text::{logical_line_for_row, process_line},
};
pub(crate) use crate::grid::content::content::make_content;
pub(crate) use crate::grid::content::text::{content_text, last_non_empty_lines};

// Backward-compat re-exports for terminal_view crate (旧 API 名字)
// pub type AlacrittyBackend = crate::Terminal;
// pub type DisplayCell = crate::cell::Cell;
