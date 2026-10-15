use std::sync::Arc;

use alacritty_terminal::{
    grid::Dimensions as _,
    index::{Column, Direction as AlacDirection, Line, Point as AlacPoint},
    sync::FairMutex,
    term::{Term, TermMode, cell::Cell as AlacCell},
    vi_mode::ViModeCursor,
    vte::ansi::{ClearMode, NamedPrivateMode, PrivateMode},
};
use futures::channel::mpsc::UnboundedSender;
use vte::ansi::Handler;

use crate::{
    Point, Scroll, Selection, SelectionSide, ViMotion, alacritty::AlacrittyTerm,
    terminal_settings::AlternateScroll,
};

use super::convert::*;
use super::listener::ZedListener;
use super::types::{AlacrittyTermConfig, AlacrittyTermLock};
use crate::{PtyEvent, TerminalBounds};

pub(crate) fn new_term(
    config: &AlacrittyTermConfig,
    bounds: TerminalBounds,
    events_tx: UnboundedSender<PtyEvent>,
    alternate_scroll: AlternateScroll,
    generation: u64,
) -> Arc<AlacrittyTermLock> {
    let mut term = Term::new(
        config.clone(),
        &bounds,
        ZedListener {
            sender: events_tx,
            generation,
        },
    );

    if let AlternateScroll::Off = alternate_scroll {
        term.unset_private_mode(PrivateMode::Named(NamedPrivateMode::AlternateScroll));
    }

    Arc::new(FairMutex::new(term))
}
pub(crate) fn resize(term: &mut AlacrittyTerm, bounds: TerminalBounds) { term.resize(bounds); }

pub(crate) fn display_offset(term: &AlacrittyTerm) -> usize { term.grid().display_offset() }

pub(crate) fn scroll_display(term: &mut Term<ZedListener>, scroll: Scroll) {
    term.scroll_display(scroll.to_alacritty());
}
pub(crate) fn set_selection(term: &mut Term<ZedListener>, selection: Option<&Selection>) {
    term.selection = selection.map(Selection::to_alacritty);
}
pub(crate) fn update_selection(
    term: &mut AlacrittyTerm,
    point: Point,
    side: SelectionSide,
) -> bool {
    let Some(mut selection) = term.selection.take() else {
        return false;
    };
    selection.update(point.to_alacritty(), side.to_alacritty());
    term.selection = Some(selection);
    true
}
pub(crate) fn selection_text(term: &Term<ZedListener>) -> Option<String> {
    term.selection_to_string()
}
pub(crate) fn scroll_to_point(term: &mut AlacrittyTerm, point: Point) {
    term.scroll_to_point(point.to_alacritty());
}
pub(crate) fn vi_goto_point(term: &mut AlacrittyTerm, point: Point) {
    term.vi_goto_point(point.to_alacritty());
}
pub(crate) fn toggle_vi_mode(term: &mut Term<ZedListener>) { term.toggle_vi_mode(); }
pub(crate) fn vi_motion(term: &mut Term<ZedListener>, motion: ViMotion) {
    term.vi_motion(motion.to_alacritty());
}
pub(crate) fn clear_saved_screen(term: &mut Term<ZedListener>) {
    term.clear_screen(ClearMode::Saved);

    let cursor = term.grid().cursor.point;

    term.grid_mut().reset_region(..cursor.line);

    let line = term.grid()[cursor.line][..Column(term.grid().columns())]
        .iter()
        .cloned()
        .enumerate()
        .collect::<Vec<(usize, AlacCell)>>();

    for (index, cell) in line {
        term.grid_mut()[Line(0)][Column(index)] = cell;
    }

    term.grid_mut().cursor.point = AlacPoint::new(Line(0), term.grid_mut().cursor.point.column);
    let new_cursor = term.grid().cursor.point;

    if (new_cursor.line.0 as usize) < term.screen_lines() - 1 {
        term.grid_mut().reset_region((new_cursor.line + 1)..);
    }
}
pub(crate) fn shrink_to_used(term: &mut Term<ZedListener>) { term.grid_mut().truncate(); }
pub(crate) fn used_lines(term: &Term<ZedListener>) -> usize {
    if term.mode().contains(TermMode::ALT_SCREEN) {
        return term.total_lines();
    }
    let grid = term.grid();
    let cursor_line = grid.cursor.point.line.0.max(0) as usize;
    let last_occupied_line = (cursor_line + 1..term.screen_lines())
        .rev()
        .find(|&line| !grid[Line(line as i32)].is_clear())
        .unwrap_or(cursor_line);
    term.history_size() + last_occupied_line + 1
}
pub(crate) fn update_vi_cursor_for_scroll(term: &mut Term<ZedListener>, scroll: Scroll) {
    match scroll {
        Scroll::Delta(delta) => {
            term.vi_mode_cursor = term.vi_mode_cursor.scroll(term, delta);
        }
        Scroll::PageUp => {
            let lines = term.screen_lines() as i32;
            term.vi_mode_cursor = term.vi_mode_cursor.scroll(term, lines);
        }
        Scroll::PageDown => {
            let lines = -(term.screen_lines() as i32);
            term.vi_mode_cursor = term.vi_mode_cursor.scroll(term, lines);
        }
        Scroll::Top => {
            let point = AlacPoint::new(term.topmost_line(), Column(0));
            term.vi_mode_cursor = ViModeCursor::new(point);
        }
        Scroll::Bottom => {
            let point = AlacPoint::new(term.bottommost_line(), Column(0));
            term.vi_mode_cursor = ViModeCursor::new(point);
        }
    }
}
pub(crate) fn update_selection_to_vi_cursor(term: &mut Term<ZedListener>) -> Option<Point> {
    let mut selection = term.selection.take()?;
    let point = term.vi_mode_cursor.point;
    selection.update(point, AlacDirection::Right);
    term.selection = Some(selection);
    Some(terminal_point_from_alacritty(point))
}

pub(crate) unsafe fn append_text_to_term(term: &mut Term<ZedListener>, text_lines: &[&str]) {
    term.newline();
    term.grid_mut().cursor.point.column = Column(0);
    for line in text_lines {
        for character in line.chars() {
            term.input(character);
        }
        term.newline();
        term.grid_mut().cursor.point.column = Column(0);
    }
}
