use alacritty_terminal::{
    grid::Dimensions as _,
    index::{Column, Point as AlacPoint},
};

use crate::alacritty::{AlacrittyTerm, logical_line_for_row, process_line};

pub(crate) fn content_text(term: &AlacrittyTerm) -> String {
    let start = AlacPoint::new(term.topmost_line(), Column(0));
    let end = AlacPoint::new(term.bottommost_line(), term.last_column());
    term.bounds_to_string(start, end)
}

pub(crate) fn last_non_empty_lines(term: &AlacrittyTerm, line_count: usize) -> Vec<String> {
    let grid = term.grid();
    let mut lines = Vec::new();

    let mut current_line = grid.bottommost_line().0;
    let topmost_line = grid.topmost_line().0;

    while current_line >= topmost_line && lines.len() < line_count {
        let (logical_line_start, logical_line) =
            logical_line_for_row(grid, current_line, topmost_line);

        if let Some(line) = process_line(logical_line) {
            lines.push(line);
        }

        current_line = logical_line_start - 1;
    }

    lines.reverse();
    lines
}
